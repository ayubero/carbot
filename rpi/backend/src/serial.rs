use axum::response::Json;
use axum::http::StatusCode;
use rusb;
use serde::{Serialize, Deserialize};
use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;

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
pub struct SerialMessage {
    port_path: String, // e.g. "/dev/ttyACM0" or "COM3"
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
    let mut port = match serialport::new(&payload.port_path, 115200)
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

    if let Err(e) = wait_for_arduino_ready(&mut port).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e);
    }

    let message = format!("{}\n", payload.message);
    if let Err(e) = port.write_all(message.as_bytes()) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Write error: {}", e));
    }

    (StatusCode::OK, format!("Sent '{}' to {}", payload.message, payload.port_path))
}