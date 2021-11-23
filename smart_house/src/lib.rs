use rand::thread_rng;
use rand::Rng;

pub enum DeviceType {
    Socket { name: String, description: String },
    Thermometer { name: String, description: String },
}
impl DeviceType {
    fn get_name(&self) -> String {
        let name = match &self { 
            DeviceType::Socket{name,description} => name.to_string(), 
            DeviceType::Thermometer{name, description} => name.to_string(), 
            _ => return "not_found".to_string(), 
        };
        name
    }
    fn get_status(&self) -> String {
        let name = match &self { 
            DeviceType::Socket{name,description} => { 
              //Socket::status()  
                name.to_string()
            }, 
            DeviceType::Thermometer{name, description} => { 
                //Thermometer::status();
                name.to_string()
            }, 
            _ => return "not_found".to_string(), 
        };
        name
    }
}
enum SocketState {
    Enabled,
    Disabled,
}

pub struct AddResult {}

pub struct RemoveResult {}

pub struct House {
    name: String,
    rooms: Vec<Room>,
}

pub fn create_house(name: &str) -> House {
    let mut rooms = Vec::<Room>::new();
    House {
        name: String::from(name),
        rooms: rooms,
    }
}

impl House {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_room(&mut self, room: Room) -> AddResult {
        self.rooms.push(room);
        //Ok(true)    
        AddResult{}
    }

    pub fn remove_room(&mut self, room_name: &str) -> RemoveResult {
        let index = self.rooms.iter().position(|x| *x.get_name() == room_name).unwrap();
        self.rooms.remove(index);
        RemoveResult{}
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn report(&self) {
        for r in self.rooms {
            r.report()
        }
    }

    pub fn get_device(&self, name: &str) -> Option<DeviceType> {
        for r in self.rooms {
            let index = r.devices.iter().position(|x| *x.get_name() == name.to_string()).unwrap();
            return Some(r.devices[index]);
        }
        None
    }
}

pub struct Room {
    name: String,
    devices: Vec<DeviceType>,
}

pub fn create_room(name: &str) -> Room {
    let mut devices = Vec::<DeviceType>::new();
    Room {
        name: String::from(name),
        devices: devices,
    }
}

impl Room {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_device(&mut self, device: DeviceType) -> AddResult {
        self.devices.push(device);
        AddResult{}    
    }

    pub fn remove_device(&mut self, device_name: &str) -> RemoveResult {
        let index = self.devices.iter().position(|x| *x.get_name() == device_name.to_string()).unwrap();
        self.devices.remove(index);
        RemoveResult{}
    }

    pub fn report(&self){
        println!("room: {}", self.name);
        for d in self.devices {
            println!("{}", d.get_status());
        }
    }

    pub fn get_devices(&self) -> &Vec<DeviceType> {
        &self.devices
    }

    pub fn get_device(&self, name: &str) -> DeviceType {
         let index = self.devices.iter().position(|x| *x.get_name() == name.to_string()).unwrap();
         return self.devices[index]
    }
}

/*pub fn create_socket(name: &str, device_type: DeviceType, description: &str) -> Socket {
    todo!()
}*/

pub trait DeviceTrait {
    fn status(&self) -> String;
    fn get_name(&self) -> String;
}

pub struct Device {
    name: String,
    description: String,
    device_type: DeviceType,
}

pub struct Socket {
    device: Device,
    state: SocketState,
}

impl Socket {
    pub fn interact(&mut self) {
        todo!()
    }

    fn get_voltage(&self) -> Option<u16> {
        todo!()
    }
}
/*pub fn create_thermometer(
    name: &str,
    device_type: DeviceType,
    description: &str,
) -> Thermometer {
    todo!()
}*/
impl DeviceTrait for Socket {
    fn status(&self) -> String {
        let current_state = match self.state {
            SocketState::Enabled => "enabled",
            SocketState::Disabled => "disabled",
        };
        let voltage = match self.get_voltage() {
            None => 0,
            Some(value) => value,
        };
        format!(
            "currente state of {} is: {} and voltage: {}",
            self.device.name, current_state, voltage
        )
    }
    fn get_name(&self) -> String {
        self.device.name
    }
}

pub struct Thermometer {
    device: Device,
}
impl Thermometer {
    fn get_temp(&self) -> Option<u16> {
        let mut rng = thread_rng();
        return Some(rng.gen_range(0, 30));
    }
}

impl DeviceTrait for Thermometer {
    fn status(&self) -> String {
        let temp = match self.get_temp() {
            None => 0,
            Some(value) => value,
        };
        format!("currente state of {} is: {}", self.device.name, temp)
    }
    fn get_name(&self) -> String {
        self.device.name
    }
}

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}*/
