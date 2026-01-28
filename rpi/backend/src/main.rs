use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use realsense_rust::{
    context::Context,
    frame::{PixelKind, ColorFrame},
    pipeline::InactivePipeline,
    kind::{Rs2Format, Rs2StreamKind}
};
use image::{ImageBuffer, Rgb};
use std::time::Duration;

mod serial;
use serial::{list_serial_devices, connect, disconnect, send};
mod websocket;
use websocket::{LAST_FRAME, websocket_handler};

#[tokio::main]
async fn main() {
    // Task to capture frames from the camera
    tokio::spawn(async move {
        let mut config = realsense_rust::config::Config::new();
        let _ = config.enable_stream(Rs2StreamKind::Color, None, 848, 480, Rs2Format::Bgr8, 15);

        let context = Context::new().unwrap();
        let pipeline = InactivePipeline::try_from(&context).unwrap();
        let mut pipeline = pipeline.start(Some(config)).unwrap();

        loop {
            let timeout = Duration::from_millis(5000);
            let frames = pipeline.wait(Some(timeout)).unwrap();
            let mut color_frames = frames.frames_of_type::<ColorFrame>();
            if color_frames.is_empty() {
                continue;
            }
            let color_frame = color_frames.pop().unwrap();
            let frame_data = encode_frame(&color_frame);
            let last_frame = LAST_FRAME.clone();
            let mut frame_guard = last_frame.lock().await;
            *frame_guard = Some(frame_data);
        }
    });

    // Build CORS layer allowing requests
    let cors = CorsLayer::new()
        .allow_origin(Any) // allows any origin (for dev; restrict in prod)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/list", get(list_serial_devices))
        .route("/connect", post(connect))
        .route("/disconnect", post(disconnect))
        .route("/send", post(send))
        .route("/camera_ws", get(websocket_handler)) // Camera websocket
        .layer(cors); // CORS middleware

    let addr = "0.0.0.0:5000";
    println!("🚀 Server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn encode_frame(color_frame: &ColorFrame) -> Vec<u8> {
    let width = color_frame.width() as u32;
    let height = color_frame.height() as u32;

    let mut img_buf = ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        if let Some(PixelKind::Bgr8 { b, g, r }) = color_frame.get(x as usize, y as usize) {
            *pixel = Rgb([*r, *g, *b]);
        }
    }

    let mut encoded_img = Vec::new();
    img_buf.write_to(&mut std::io::Cursor::new(&mut encoded_img), image::ImageOutputFormat::Jpeg(90)).unwrap();
    encoded_img
}