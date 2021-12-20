use crate::devices::Device;
use crate::devices::DeviceTrait;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
pub struct Socket {
    state: SocketState,
    pub device: Device,
}

#[derive(PartialEq, Debug, Clone)]
enum SocketState {
    Enabled,
    Disabled,
}

impl Socket {
    pub fn new(name: &str, description: &str) -> Self {
        Socket {
            device: Device {
                name: name.to_string(),
                description: description.to_string(),
            },
            state: SocketState::Disabled,
        }
    }
    pub fn interact(&mut self) {
        let new_state = match self.state {
            SocketState::Enabled => SocketState::Disabled,
            SocketState::Disabled => SocketState::Enabled,
        };
        self.state = new_state;
    }

    fn get_voltage(&self) -> Option<u16> {
        if self.state == SocketState::Disabled {
            return None;
        }
        let mut rng = thread_rng();
        Some(rng.gen_range(1..221))
    }
}

impl DeviceTrait for Socket {
    fn status(&self) -> String {
        let state = match self.state {
            SocketState::Enabled => "enabled",
            SocketState::Disabled => "disabled",
        };
        let voltage = self.get_voltage().unwrap_or(0);
        return format!(
            "Socket - name: {}, status: {}, voltage: {}",
            self.get_name(),
            state,
            voltage
        );
    }
    fn get_name(&self) -> String {
        self.device.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_socket() {
        let socket = Socket::new("test", "test description");
        assert_eq!("test".to_string(), socket.device.name);
        assert_eq!("test description".to_string(), socket.device.description);
    }

    #[test]
    fn test_intercat_enable() {
        let mut socket = Socket::new("test", "test description");
        socket.interact();
        assert_eq!(SocketState::Enabled, socket.state);
    }

    #[test]
    fn test_intercat_disable() {
        let mut socket = Socket::new("test", "test description");
        socket.interact();
        socket.interact();
        assert_eq!(SocketState::Disabled, socket.state);
    }

    #[test]
    fn test_get_voltage_when_disabled() {
        let socket = Socket::new("test", "test description");
        let voltage = socket.get_voltage();
        match voltage {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_get_voltage_when_enabled() {
        let mut socket = Socket::new("test", "test description");
        socket.interact();
        let voltage = socket.get_voltage();
        match voltage {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_status() {
        let socket = Socket::new("test", "test description");
        let status = socket.status();
        if status.contains("Socket - name:")
            && status.contains("status:")
            && status.contains("voltage:")
        {
            assert!(true);
            return;
        }
        assert!(false);
    }

    #[test]
    fn test_get_name() {
        let socket = Socket::new("test", "test description");
        let name = socket.get_name();
        assert_eq!("test".to_string(), name);
    }
}
