use axum::response::Json;
use rusb;
use serde::Serialize;

#[derive(Serialize)]
pub struct UsbDeviceInfo {
    vendor_id: u16,
    product_id: u16,
    bus_number: u8,
    address: u8,
}

pub async fn list_usb_devices() -> Json<Vec<UsbDeviceInfo>> {
    //let context = Context::new().unwrap();
    let devices = rusb::devices().unwrap();

    let usb_devices: Vec<UsbDeviceInfo> = devices
        .iter()
        .filter_map(|device| {
            let device_desc = device.device_descriptor().ok()?;
            Some(UsbDeviceInfo {
                vendor_id: device_desc.vendor_id(),
                product_id: device_desc.product_id(),
                bus_number: device.bus_number(),
                address: device.address(),
            })
        })
        .collect();

    Json(usb_devices)
}
