use crate::devices::DeviceType;
//use crate::error::{Error, Result};
use crate::error::{Result};

pub struct Room {
    _name: String,
    _devices: Vec<DeviceType>,
}

pub fn create_room(_name: &str) -> Room {
    todo!()
}

impl Room {
    pub fn get_name(&self) -> &String {
        todo!()
    }

    //pub fn add_device<D: DeviceTrait + 'static>(&mut self, _device: D) -> AddResult {
    pub fn add_device(&mut self, _device: DeviceType) -> Result<DeviceType> {
        todo!()
    }

    pub fn remove_device(&mut self, _device_name: &str) -> Result<bool> {
        todo!()
    }

    pub fn get_devices(&self) -> Vec<DeviceType> {
        todo!()
    }

    pub fn get_device(&self, _name: &str) -> DeviceType {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::socket::Socket;

    #[test]
    fn test_create_room() {
        let devices = Vec::<DeviceType>::new();
        let test_room = Room {
            _name: "test room".to_string(),
            _devices: devices
        };
        let created_room = create_room("test room");
        //assert_eq!(test_house, created_house); mb PartialEq?
        
        assert_eq!(test_room._name, created_room._name);
        assert_eq!(test_room._devices.len(), created_room._devices.len());
    }

    #[test]
    fn test_get_room_name() {
        let devices = Vec::<DeviceType>::new();
        let test_room = Room {
            _name: "test room".to_string(),
            _devices: devices
        };
        assert_eq!("test_room", test_room.get_name());
    }
    
    #[test]
    fn test_add_device() {
        let devices = Vec::<DeviceType>::new();
        let mut test_room = Room {
            _name: "test room".to_string(),
            _devices: devices
        };
        //let room = Room{};
        let socket = Socket::_new("test", "test");
        let device = DeviceType::_Socket(socket);
        let _add_result = test_room.add_device(device);
        match _add_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_remove_device() {
        let mut room = create_room("test");
        let socket = Socket::_new("test", "test");
        let device = DeviceType::_Socket(socket);
        room._devices.push(device);
        let _remove_result = room.remove_device("test device");
        match _remove_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_get_devices() {
        let mut devices = Vec::<DeviceType>::new();
        let socket = Socket::_new("test", "test");
        let device = DeviceType::_Socket(socket);
        devices.push(device);
        let test_room = Room {
            _name: "test room".to_string(),
            _devices: devices
        };
        let devices = test_room.get_devices();
        assert_eq!(1, devices.len());
    }
}
