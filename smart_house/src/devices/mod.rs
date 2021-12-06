pub mod socket;
pub mod thermometer;

use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;

pub enum DeviceType {
    _Socket(Socket),
    _Thermometer(Thermometer),
}

pub trait DeviceTrait {
    fn status(&self) -> String;
    fn get_name(&self) -> String;
}

pub struct Device {
    _name: String,
    _description: String,
}
