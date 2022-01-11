use crate::devices::Device;
use crate::devices::DeviceTrait;
use crate::error::Error::Message;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

#[derive(Clone)]
pub struct Socket {
    state: SocketState,
    pub device: Device,
    pub host: String,
}

#[derive(PartialEq, Debug, Clone)]
enum SocketState {
    Enabled,
    Disabled,
}

#[derive(Serialize, Debug)]
struct Request {
    action: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    voltage: u16,
    status: String,
    error: String,
}

impl Socket {
    pub fn new(name: &str, description: &str, host: &str) -> Self {
        Socket {
            device: Device {
                name: name.to_string(),
                description: description.to_string(),
            },
            host: host.to_string(),
            state: SocketState::Disabled,
        }
    }

    fn make_call(&self, req: Request) -> Result<Response> {
        match TcpStream::connect(&self.host) {
            Ok(mut stream) => {
                println!("Successfully connected to server in port {}", &self.host);

                let msg = serde_json::to_string(&req);
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => return Err(Message(e.to_string())),
                };

                stream.write_all(msg.as_bytes()).unwrap();

                let mut data = [0u8; 250]; // using 6 byte buffer
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap();
                        let response = serde_json::from_str(text);
                        let response: Response = match response {
                            Ok(r) => r,
                            Err(e) => return Err(Message(e.to_string())),
                        };
                        Ok(response)
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        Err(Message(e.to_string()))
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err(Message(e.to_string()))
            }
        }
    }

    pub fn interact(&mut self) {
        let req = Request {
            action: "interact".to_string(),
        };
        let result = self.make_call(req);
        match result {
            Ok(r) => {
                if r.error.is_empty() {
                    let new_state = match self.state {
                        SocketState::Enabled => SocketState::Disabled,
                        SocketState::Disabled => SocketState::Enabled,
                    };
                    self.state = new_state;
                }
            }
            Err(e) => {
                println!("Failed to make call: {}", e);
            }
        }
    }

    fn get_voltage(&self) -> Option<u16> {
        if self.state == SocketState::Disabled {
            return None;
        }
        let req = Request {
            action: "get_voltage".to_string(),
        };
        let result = self.make_call(req);
        match result {
            Ok(r) => {
                if !r.error.is_empty() {
                    println!("Voltage: {}", r.voltage);
                    Some(r.voltage)
                } else {
                    println!("Get voltage error: {}", r.error);
                    None
                }
            }
            Err(e) => {
                println!("Failed to make call: {}", e);
                None
            }
        }
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
