use std::{io::Write, path::PathBuf, process::{Command, Stdio, ExitStatus}, sync::Arc};
use axum::{Json, body::Body, http::{StatusCode, header}, response::{IntoResponse, Response}};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::{fs::File, sync::Mutex, io::AsyncReadExt};
use zip::{ZipWriter, write::FileOptions};

// Shared state for recording
pub static COLOR_FRAMES: Lazy<Mutex<Option<Vec<Vec<u8>>>>> = Lazy::new(|| {
    Mutex::new(None)
});
pub static DEPTH_FRAMES: Lazy<Mutex<Option<Vec<Vec<u8>>>>> = Lazy::new(|| {
    Mutex::new(None)
});
pub static IS_RECORDING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| {
    Arc::new(Mutex::new(false))
});
const RGB_VIDEO_PATH: &str = "/recordings/rgb.mp4";
const DEPTH_VIDEO_PATH: &str = "/recordings/depth.mp4";

pub async fn start_recording() -> impl IntoResponse {
    let mut is_recording = IS_RECORDING.lock().await;
    *is_recording = true;
    let mut recording_color_frames = COLOR_FRAMES.lock().await;
    *recording_color_frames = Some(Vec::new());
    let mut recording_depth_frames = DEPTH_FRAMES.lock().await;
    *recording_depth_frames = Some(Vec::new());
    Json(json!({ "status": "recording started" }))
}

pub async fn stop_recording() -> impl IntoResponse {
    let mut is_recording = IS_RECORDING.lock().await;
    *is_recording = false;

    // Save RGB video
    let mut recorded_frames = COLOR_FRAMES.lock().await;
    if let Some(frames) = recorded_frames.take() {
        let output_path: PathBuf = PathBuf::from(RGB_VIDEO_PATH);
        let status = save_video(frames, output_path);
        if !status.success() {
            eprintln!("ffmpeg failed: {:?}", status);
            return Json(json!({ "status": "ffmpeg color encoding failed" }));
        }
    } else {
        return Json(json!({ "status": "no color frames recorded" }));
    }

    // Save depth video
    let mut recorded_frames = DEPTH_FRAMES.lock().await;
    if let Some(frames) = recorded_frames.take() {
        let output_path: PathBuf = PathBuf::from(DEPTH_VIDEO_PATH);
        let status = save_video(frames, output_path);
        if !status.success() {
            return Json(json!({ "status": "ffmpeg depth encoding failed" }));
        }
    } else {
        return Json(json!({ "status": "no depth frames recorded" }));
    }

    Json(json!({ "status": "recordings stopped and saved" }))
}

fn save_video(frames: Vec<Vec<u8>>, output_path: PathBuf) -> ExitStatus {
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
            }
        }
    }

    let status = ffmpeg.wait().expect("Failed to wait for ffmpeg");
    status
}

pub async fn download_recordings() -> Result<impl IntoResponse, StatusCode> {
    let filepaths = [RGB_VIDEO_PATH, DEPTH_VIDEO_PATH];

    // Create a temporary ZIP file in memory
    let mut zip_buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));

        for filepath in &filepaths {
            let path = PathBuf::from(filepath);
            if !path.exists() {
                return Err(StatusCode::NOT_FOUND);
            }

            // Add the file to the ZIP archive
            zip.start_file::<&str, ()>(filepath, FileOptions::default())

                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut file = File::open(&path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            zip.write_all(&buffer)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

        // Finish the ZIP archive
        zip.finish().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Stream the ZIP file to the client
    let stream = tokio_util::io::ReaderStream::new(std::io::Cursor::new(zip_buffer));
    let body = Body::from_stream(stream);

    let mut response = Response::new(body);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "application/zip".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"recordings.zip\"".parse().unwrap(),
    );

    Ok(response)
}