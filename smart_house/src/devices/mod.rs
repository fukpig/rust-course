pub mod socket;
pub mod thermometer;

use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;

#[derive(Clone)]
pub enum DeviceType {
    Socket(Socket),
    Thermometer(Thermometer),
}

impl DeviceType {
    pub fn get_name(&self) -> String {
        match self {
            DeviceType::Socket(d) => d.get_name(),
            DeviceType::Thermometer(d) => d.get_name(),
        }
    }
}

pub trait DeviceTrait {
    fn status(&self) -> String;
    fn get_name(&self) -> String;
}

#[derive(Clone)]
pub struct Device {
    name: String,
    description: String,
}
