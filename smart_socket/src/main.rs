use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
//use std::thread;
use rand::thread_rng;
use rand::Rng;
//use std::sync::atomic::AtomicBool;
//use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Socket {
    state: SocketState,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SocketState {
    Enabled,
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    action: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    voltage: u16,
    status: String,
    error: String,
}

impl Socket {
    pub fn new() -> Self {
        Socket {
            state: SocketState::Disabled,
        }
    }
    pub fn set_state(&mut self, new_state: SocketState) {
        self.state = new_state
    }
    pub fn interact(&mut self) -> SocketState {
        match self.state {
            SocketState::Enabled => SocketState::Disabled,
            SocketState::Disabled => SocketState::Enabled,
        };
        SocketState::Disabled
    }

    fn get_voltage(&self) -> Option<u16> {
        if self.state == SocketState::Disabled {
            return None;
        }
        let mut rng = thread_rng();
        Some(rng.gen_range(1..221))
    }
}

fn handle_client(mut stream: TcpStream, mut socket: Socket) -> Option<SocketState> {
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let u = Request::deserialize(&mut de).unwrap();
    match u.action.as_str() {
        "interact" => {
            let new_state = socket.interact();
            let req = Response {
                error: "".to_string(),
                status: "success".to_string(),
                voltage: 0,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
            return Some(new_state);
        }
        "get_voltage" => {
            let voltage = socket.get_voltage().unwrap_or(0);
            let req = Response {
                error: "".to_string(),
                status: "success".to_string(),
                voltage,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        }
        _ => {
            let req = Response {
                error: "unknown action".to_string(),
                status: "error".to_string(),
                voltage: 0,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        }
    }

    None
}
/* fn interact(mut is_socket_enabled: AtomicBool) {
        if *is_socket_enabled.get_mut() == false {
            *is_socket_enabled.get_mut() = true;
        }
        *is_socket_enabled.get_mut() = false;
    }

    fn get_voltage(mut is_socket_enabled: AtomicBool) -> Option<u16> {
        if *is_socket_enabled.get_mut() == false {
            return None;
        }
        let mut rng = thread_rng();
        println!("voltage");
        Some(rng.gen_range(1..221))
    }

fn handle_client(mut stream: TcpStream, is_socket_enabled: AtomicBool) {
    let mut de = serde_json::Deserializer::from_reader(&stream);
    let u = Request::deserialize(&mut de).unwrap();
    match u.action.as_str() {
        "interact" => {
            interact(is_socket_enabled);
            let req = Response {
                error: "".to_string(),
                status: "success".to_string(),
                voltage: 0,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        },
        "get_voltage" => {
            //socket.get_voltage();
            let voltage = match get_voltage(is_socket_enabled) {
                Some(v) => v,
                None => 0,
            };
            let req = Response {
                error: "".to_string(),
                status: "success".to_string(),
                voltage: voltage,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        },
        _ => {
            let req = Response {
                error: "unknown action".to_string(),
                status: "error".to_string(),
                voltage: 0,
            };
            let msg = serde_json::to_string(&req).unwrap();
            stream.write_all(msg.as_bytes()).unwrap();
        }
    }

                println!("end");
}*/

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    let mut socket = Socket::new();
    //let is_socket_enabled = Arc::new(AtomicBool::new(false));
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                //thread::spawn(move || handle_client(stream, &is_socket_enabled));
                let new_state = handle_client(stream, socket);
                if let Some(s) = new_state {
                    socket.set_state(s)
                }
                /*match new_state {
                    Some(s) => socket.set_state(s),
                    None => {}
                };*/
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
