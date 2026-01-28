use axum::response::Json;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::sync::Mutex;
use std::sync::Arc;

// Global serial port instance
static SERIAL_PORT: Lazy<Arc<Mutex<Option<Box<dyn SerialPort>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

#[derive(Serialize)]
pub struct SerialDeviceInfo {
    port_name: String,
    vid: Option<u16>,
    pid: Option<u16>,
    serial_number: Option<String>,
    manufacturer: Option<String>,
    product: Option<String>,
}

pub async fn list_serial_devices() -> Json<Vec<SerialDeviceInfo>> {
    let ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_) => return Json(vec![]),
    };

    let serial_devices: Vec<SerialDeviceInfo> = ports
        .into_iter()
        .map(|p| match p.port_type {
            serialport::SerialPortType::UsbPort(info) => SerialDeviceInfo {
                port_name: p.port_name,
                vid: Some(info.vid),
                pid: Some(info.pid),
                serial_number: info.serial_number,
                manufacturer: info.manufacturer,
                product: info.product,
            },
            _ => SerialDeviceInfo {
                port_name: p.port_name,
                vid: None,
                pid: None,
                serial_number: None,
                manufacturer: None,
                product: None,
            },
        })
        .collect();

    Json(serial_devices)
}

#[derive(Deserialize)]
pub struct ConnectRequest {
    port_path: String,
}

pub async fn connect(Json(payload): Json<ConnectRequest>) -> (StatusCode, String) {
    let mut port_guard: tokio::sync::MutexGuard<'_, Option<Box<dyn serialport::SerialPort>>> =
        SERIAL_PORT.lock().await;

    if port_guard.is_some() {
        return (StatusCode::BAD_REQUEST, "Serial port already connected".to_string());
    }

    let port = match serialport::new(&payload.port_path, 115200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(100))
        .open()
    {
        Ok(p) => p,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Port error: {}", e)),
    };

    *port_guard = Some(port);

    (StatusCode::OK, format!("Connected to {}", payload.port_path))
}

pub async fn disconnect() -> (StatusCode, String) {
    let mut port_guard: tokio::sync::MutexGuard<'_, Option<Box<dyn serialport::SerialPort>>> =
        SERIAL_PORT.lock().await;

    if let Some(_port) = port_guard.take() {
        // The port will be closed when it goes out of scope
        (StatusCode::OK, "Disconnected from serial port".to_string())
    } else {
        (StatusCode::BAD_REQUEST, "Serial port not connected".to_string())
    }
}

#[derive(Deserialize)]
pub struct SerialMessage {
    message: String,
}

async fn wait_for_arduino_ready(port: &mut Box<dyn SerialPort>) -> Result<(), String> {
    let start = std::time::Instant::now();
    let mut buffer = [0u8; 64];
    
    while start.elapsed() < Duration::from_secs(3) {
        // Try sending a ping
        if port.write_all(b"\n").is_ok() {
            port.flush().ok();
            
            // Wait a bit for response
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            // Check if we can read (indicates Arduino is responsive)
            if let Ok(n) = port.read(&mut buffer) {
                if n > 0 {
                    return Ok(()); // Arduino responded
                }
            }
        }
        
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    Err("Timeout waiting for Arduino".to_string())
}

pub async fn send(Json(payload): Json<SerialMessage>) -> (StatusCode, String) {
    let mut port_guard: tokio::sync::MutexGuard<'_, Option<Box<dyn serialport::SerialPort>>> =
        SERIAL_PORT.lock().await;

    let port = match &mut *port_guard {
        Some(p) => p,
        None => return (StatusCode::BAD_REQUEST, "Serial port not connected".to_string()),
    };

    if let Err(e) = wait_for_arduino_ready(port).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e);
    }

    let message = format!("{}\n", payload.message);
    if let Err(e) = port.write_all(message.as_bytes()) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Write error: {}", e));
    }

    (StatusCode::OK, format!("Sent '{}'", payload.message))
}
