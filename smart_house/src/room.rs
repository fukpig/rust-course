use crate::devices::DeviceType;
//use crate::error::{Error, Result};
//use crate::devices::DeviceTrait;
use crate::error::Error::AlreadyExist;
use crate::error::Error::NotFound;
use crate::error::Result;

#[derive(Clone)]
pub struct Room {
    name: String,
    devices: Vec<DeviceType>,
}

impl Room {
    pub fn new(name: &str) -> Room {
        Room {
            name: name.to_string(),
            devices: vec![],
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    //pub fn add_device<D: DeviceTrait + 'static>(&mut self, _device: D) -> AddResult {
    pub fn add_device(&mut self, device: DeviceType) -> Result<&DeviceType> {
        let check_index = self.devices.iter().position(|x| {
            if x.get_name() == device.get_name() {
                return true;
            }
            false
        });
        match check_index {
            Some(_) => return Err(AlreadyExist(device.get_name())),
            None => {}
        }
        self.devices.push(device);
        let recently_added = self.devices.last().unwrap();
        Ok(recently_added)
    }

    pub fn remove_device(&mut self, name: &str) -> Result<DeviceType> {
        let check_index = self.devices.iter().position(|x| {
            //if x.get_name() == name.to_string() {
            if x.get_name() == *name {
                return true;
            }
            false
        });
        let index = match check_index {
            Some(i) => i,
            None => return Err(NotFound(name.to_string())),
        };
        let device = self.devices[index].clone();
        self.devices.remove(index);
        Ok(device)
    }

    pub fn get_devices(&self) -> Vec<DeviceType> {
        self.devices.to_vec()
    }

    pub fn get_device(&self, name: &str) -> Result<&DeviceType> {
        let check_index = self.devices.iter().position(|x| {
            if x.get_name() == *name {
                return true;
            }
            false
        });
        let index = match check_index {
            Some(i) => i,
            None => return Err(NotFound(name.to_string())),
        };
        let device = &self.devices[index];
        Ok(device)
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
            name: "test room".to_string(),
            devices: devices,
        };
        let created_room = Room::new("test room");
        //assert_eq!(test_house, created_house); mb PartialEq?

        assert_eq!(test_room.name, created_room.name);
        assert_eq!(test_room.devices.len(), created_room.devices.len());
    }

    #[test]
    fn test_get_room_name() {
        let devices = Vec::<DeviceType>::new();
        let test_room = Room {
            name: "test room".to_string(),
            devices: devices,
        };
        assert_eq!("test room", test_room.get_name());
    }

    #[test]
    fn test_add_device() {
        let devices = Vec::<DeviceType>::new();
        let mut test_room = Room {
            name: "test room".to_string(),
            devices: devices,
        };
        //let room = Room{};
        let socket = Socket::new("test", "test");
        let device = DeviceType::Socket(socket);
        let add_result = test_room.add_device(device);
        match add_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::new("test");
        let socket = Socket::new("test", "test");
        let device = DeviceType::Socket(socket);
        room.devices.push(device);
        let remove_result = room.remove_device("test");
        match remove_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_get_devices() {
        let mut devices = Vec::<DeviceType>::new();
        let socket = Socket::new("test", "test");
        let device = DeviceType::Socket(socket);
        devices.push(device);
        let test_room = Room {
            name: "test room".to_string(),
            devices: devices,
        };
        let devices = test_room.get_devices();
        assert_eq!(1, devices.len());
    }
}
