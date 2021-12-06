use crate::devices::DeviceTrait;
use crate::devices::Device;

pub struct Socket {
    _state: SocketState,
    _device: Device,
}

#[derive(PartialEq,Debug)]
enum SocketState {
    _Enabled,
    _Disabled,
}

impl Socket {
    pub fn _new(_name: &str, _description: &str) -> Self {
        todo!()
    }
    pub fn _interact(&mut self) {
        todo!()
    }

    fn _get_voltage(&self) -> Option<u16> {
        todo!()
    }
}

impl DeviceTrait for Socket {
    fn status(&self) -> String {
        todo!()
    }
    fn get_name(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket() {
        let socket = Socket::_new("test", "test description");
        assert_eq!("test".to_string(), socket._device._name);
        assert_eq!("test description".to_string(), socket._device._description);
    }
    
    #[test]
    fn test_intercat_enable() {
        let mut socket = Socket::_new("test", "test description");
        socket._interact();
        assert_eq!(SocketState::_Enabled, socket._state);
    }
    
    #[test]
    fn test_intercat_disable() {
        let mut socket = Socket::_new("test", "test description");
        socket._interact();
        socket._interact();
        assert_eq!(SocketState::_Disabled, socket._state);
    }

    #[test]
    fn test_get_voltage_when_disabled() {
        let socket = Socket::_new("test", "test description");
        let voltage = socket._get_voltage();
        match voltage {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_get_voltage_when_enabled() {
        let mut socket = Socket::_new("test", "test description");
        socket._interact();
        let voltage = socket._get_voltage();
        match voltage {
            Some(_) => assert!(true),
            None => assert!(false)
        }
    }
    
    #[test]
    fn test_get_status() {
        let socket = Socket::_new("test", "test description");
        let status = socket.status();
        if status.contains("voltage is ") {
            assert!(true);
        }
        assert!(false);
    }    
    
    #[test]
    fn test_get_name() {
        let socket = Socket::_new("test", "test description");
        let name = socket.get_name();
        assert_eq!("test".to_string(), name);
    }    
}
