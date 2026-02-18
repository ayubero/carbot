use std::{io::Write, path::PathBuf, process::{Command, Stdio}, sync::Arc};
use axum::{Json, http::{StatusCode, header}, response::{IntoResponse, Response}};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::sync::Mutex;

// Shared state for recording
pub static RECORDING_FRAMES: Lazy<Mutex<Option<Vec<Vec<u8>>>>> = Lazy::new(|| {
    Mutex::new(None)
});
pub static IS_RECORDING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
    Arc::new(Mutex::new(false))
});
const RECORDING_PATH: &str = "/recordings/output.mp4";

pub async fn start_recording() -> impl IntoResponse {
    let mut is_recording = IS_RECORDING.lock().await;
    *is_recording = true;
    let mut recording_frames = RECORDING_FRAMES.lock().await;
    *recording_frames = Some(Vec::new());
    Json(json!({ "status": "recording started" }))
}

pub async fn stop_recording() -> impl IntoResponse {
    let mut is_recording = IS_RECORDING.lock().await;
    *is_recording = false;

    let mut recording_frames = RECORDING_FRAMES.lock().await;
    if let Some(frames) = recording_frames.take() {
        let output_path: PathBuf = PathBuf::from(RECORDING_PATH);
        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let output_str: &str = output_path.to_str().unwrap();
        let mut ffmpeg: std::process::Child = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-f", "image2pipe",
                "-i", "-",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                "-r", "15",
                output_str
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn ffmpeg");

        if let Some(stdin) = ffmpeg.stdin.as_mut() {
            for frame in frames {
                if let Err(e) = stdin.write_all(&frame) {
                    eprintln!("Failed to write frame to ffmpeg: {}", e);
                    return Json(json!({ "status": "error writing frames" }));
                }
            }
        }

        let status = ffmpeg.wait().expect("Failed to wait for ffmpeg");
        if !status.success() {
            eprintln!("ffmpeg failed: {:?}", status);
            return Json(json!({ "status": "ffmpeg encoding failed" }));
        }

        Json(json!({ "status": "recording stopped and saved" }))
    } else {
        Json(json!({ "status": "no frames recorded" }))
    }
}

pub async fn download_recording() -> Result<Response<axum::body::Body>, StatusCode> {
    let video_path: PathBuf = PathBuf::from(RECORDING_PATH);
    if !video_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file = tokio::fs::File::open(&video_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = axum::body::Body::from_stream(tokio_util::io::ReaderStream::new(file));

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "video/mp4")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", video_path.file_name().unwrap().to_string_lossy()),
        )
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}