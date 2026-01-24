use serialport::{SerialPort, DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Serial port configuration
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115200;

    // Open the serial port
    let mut port = serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(100)) // Short timeout for non-blocking reads
        .open()?;

    println!("Serial port opened. Type a command (e.g., 'forward'):");

    // Loop to send commands
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim();

        // Exit if the user types "exit"
        if command == "exit" {
            break;
        }

        // Send the command to the Arduino
        port.write_all(command.as_bytes())?;
        port.write_all(b"\n")?;

        // Read the Arduino's response (non-blocking)
        let mut response = String::new();
        let mut buffer = [0u8; 64];
        loop {
            match port.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    // Data received, append to response
                    response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                }
                Ok(_) => {
                    // Timeout occurred, no data received
                    break;
                }
                Err(e) => {
                    // Handle other errors (e.g., port disconnected)
                    eprintln!("Error reading from serial port: {}", e);
                    break;
                }
            }
        }

        if !response.is_empty() {
            println!("Arduino response: {}", response);
        } else {
            println!("No response (timeout). Command sent successfully.");
        }
    }

    Ok(())
}
