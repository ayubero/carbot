use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;
use tower_http::cors::{CorsLayer, Any};
use realsense_rust::{
    context::Context, frame::{ColorFrame, DepthFrame, PixelKind}, kind::{Rs2Format, Rs2StreamKind}, pipeline::InactivePipeline, processing_blocks::align::Align
};
use image::{ImageBuffer, Rgb, RgbImage};
use std::{sync::Arc, time::Duration};

mod mpu6050;
use mpu6050::MPU6050;
mod recording;
use recording::{IS_RECORDING, COLOR_FRAMES, DEPTH_FRAMES, start_recording, stop_recording, download_recordings};
mod serial;
use serial::{list_serial_devices, connect, disconnect, send, read_mpu6050};
mod websocket;
use websocket::{LAST_FRAME, websocket_handler};

#[tokio::main]
async fn main() {
    // Initialize MPU6050
    let mpu = Arc::new(Mutex::new(MPU6050::new().expect("Failed to initialize MPU6050")));

    
    let handle = tokio::runtime::Handle::current();
    
    // Task to capture frames from the camera
    //tokio::spawn(async move {
    std::thread::spawn(move || {
        let mut config = realsense_rust::config::Config::new();
        let _ = config
            .enable_stream(Rs2StreamKind::Color, None, 640, 360, Rs2Format::Bgr8, 15).unwrap()
            .enable_stream(Rs2StreamKind::Depth, None, 640, 360, Rs2Format::Z16, 15).unwrap();

        let context = Context::new().unwrap();
        let pipeline = InactivePipeline::try_from(&context).unwrap();
        let mut pipeline = pipeline.start(Some(config)).unwrap();

        let mut align = Align::new(Rs2StreamKind::Color, 10).expect("Failed to create align block");

        loop {
            let timeout = Duration::from_millis(5000);
            let frames = pipeline.wait(Some(timeout)).unwrap();

            align.queue(frames).unwrap();
            let aligned_frames = match align.wait(Duration::from_millis(100)) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let mut color_frames = aligned_frames.frames_of_type::<ColorFrame>();
            let mut depth_frames = aligned_frames.frames_of_type::<DepthFrame>();

            if color_frames.is_empty() || depth_frames.is_empty() {
                continue;
            }

            let color_frame = color_frames.pop().unwrap();
            let depth_frame = depth_frames.pop().unwrap();

            let color_frame_data = encode_color_frame(&color_frame);
            let depth_frame_data = encode_depth_frame(&depth_frame);

            // Block on handle
            handle.block_on(async {
                // Update last frame
                {
                    let mut frame_guard = LAST_FRAME.lock().await;
                    *frame_guard = Some(color_frame_data.clone());
                }

                // Store frames if recording
                let is_recording = IS_RECORDING.lock().await;
                if *is_recording {
                    let mut c_guard = COLOR_FRAMES.lock().await;
                    c_guard.get_or_insert_with(Vec::new).push(color_frame_data);

                    let mut d_guard = DEPTH_FRAMES.lock().await;
                    d_guard.get_or_insert_with(Vec::new).push(depth_frame_data);
                }
            });
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
        .route("/read_imu", get(read_mpu6050))
        .route("/camera_ws", get(websocket_handler)) // Camera websocket
        .route("/start_recording", post(start_recording))
        .route("/stop_recording", post(stop_recording))
        .route("/download_recordings", get(download_recordings))
        .with_state(mpu)
        .layer(cors); // CORS middleware

    let addr = "0.0.0.0:5000";
    println!("🚀 Server running at http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn encode_color_frame(color_frame: &ColorFrame) -> Vec<u8> {
    let width = color_frame.width() as u32;
    let height = color_frame.height() as u32;

    let mut img_buf = ImageBuffer::new(width, height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        if let Some(PixelKind::Bgr8 { b, g, r }) = color_frame.get(x as usize, y as usize) {
            *pixel = Rgb([*r, *g, *b]);
        }
    }

    let mut encoded_img = Vec::new();
    img_buf.write_to(&mut std::io::Cursor::new(&mut encoded_img), image::ImageOutputFormat::Png).unwrap();
    encoded_img
}

fn depth_to_color(normalized: f32) -> [u8; 3] {
    // Invert the normalized value so nearer points get higher values
    let inverted = 1.0 - normalized;
    let inverted = inverted.clamp(0.0, 1.0);

    // Jet colormap: blue -> cyan -> green -> yellow -> red
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if inverted < 0.25 {
        b = 0.5 + 2.0 * inverted;
    } else if inverted < 0.5 {
        b = 1.0;
        g = -1.0 + 4.0 * inverted;
    } else if inverted < 0.75 {
        b = -3.0 + 4.0 * inverted;
        g = 1.0;
        r = -0.5 + 2.0 * inverted;
    } else {
        g = 1.0 - 4.0 * (inverted - 0.75);
        r = 1.0;
    }

    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    ]
}

fn encode_depth_frame(depth_frame: &DepthFrame) -> Vec<u8> {
    let width = depth_frame.width();
    let height = depth_frame.height();
    let mut img_buf = RgbImage::new(width as u32, height as u32);

    // Get the multiplier to convert raw units to meters (millimeters to meters: 0.001)
    let units = depth_frame.depth_units().unwrap_or(0.001);

    let raw_data: &[u16] = unsafe {
        let ptr = depth_frame.get_data() as *const _ as *const u16; 
        std::slice::from_raw_parts(ptr, (width * height) as usize)
    };

    // Visualization range in meters (adjust based on your environment)
    let min_m = 0.2; 
    let max_m = 5.0; 

    for (i, &raw_val) in raw_data.iter().enumerate() {
        let x = (i % width) as u32;
        let y = (i / width) as u32;

        let dist_m = raw_val as f32 * units;

        let color = if raw_val == 0 {
            [0, 0, 0] // Black for no-data/out-of-range
        } else {
            // Normalize to 0.0 - 1.0 for the colormap
            let normalized = ((dist_m - min_m) / (max_m - min_m)).clamp(0.0, 1.0);
            depth_to_color(normalized)
        };

        img_buf.put_pixel(x, y, Rgb(color));
    }

    let mut encoded_img = Vec::new();
    img_buf.write_to(
        &mut std::io::Cursor::new(&mut encoded_img),
        image::ImageOutputFormat::Png,
    ).unwrap();
    encoded_img
}
