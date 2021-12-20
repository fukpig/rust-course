pub mod socket;
pub mod thermometer;

use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;

#[derive(Clone)]
pub enum DeviceType {
    Socket(Socket),
    Thermometer(Thermometer),
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
