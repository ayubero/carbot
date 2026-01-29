use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};

pub struct MPU6050 {
    mpu: Mpu6050<I2cdev>,
}

impl MPU6050 {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let i2c = I2cdev::new("/dev/i2c-1")?; // Default I2C bus on Raspberry Pi
        let mut mpu = Mpu6050::new(i2c);
        let _ = mpu.init(&mut Delay);
        Ok(Self { mpu })
    }

    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Box<dyn std::error::Error>> {
        let accel = self.mpu.get_acc().unwrap();
        Ok((accel.x, accel.y, accel.z))
    }

    pub fn read_gyro(&mut self) -> Result<(f32, f32, f32), Box<dyn std::error::Error>> {
        let gyro = self.mpu.get_gyro().unwrap();
        Ok((gyro.x, gyro.y, gyro.z))
    }
}
