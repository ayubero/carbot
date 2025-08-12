use axum::response::Json;
use axum::http::StatusCode;
use rusb;
use serde::{Serialize, Deserialize};
use serialport::SerialPort;
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

pub async fn send(Json(payload): Json<SerialMessage>) -> (StatusCode, String) {
    // Try to open the serial port
    match serialport::new(&payload.port_path, 115_200)
        .timeout(Duration::from_secs(2))
        .open()
    {
        Ok(mut port) => {
            // Send message bytes
            if let Err(e) = port.write_all(payload.message.as_bytes()) {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Write error: {}", e));
            }
            (StatusCode::OK, format!("Sent '{}' to {}", payload.message, payload.port_path))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to open port: {}", e)),
    }
}