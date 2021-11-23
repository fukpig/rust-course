pub enum DeviceType {
    _Socket(Socket),
    _Thermometer(Thermometer),
}

enum SocketState {
    _Enabled,
    _Disabled,
}

pub struct AddResult {}

pub struct RemoveResult {}

pub struct House {
    _name: String,
    _rooms: Vec<Room>,
}

pub fn create_house(_name: &str) -> House {
    todo!()
}
impl House {
    pub fn get_name(&self) -> &String {
        todo!()
    }

    pub fn add_room(&mut self, _room: Room) -> AddResult {
        todo!()
    }

    pub fn remove_room(&mut self, _room_name: &str) -> RemoveResult {
        todo!()
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        todo!()
    }

    pub fn report(&self) {
        todo!()
    }
}

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

    pub fn add_device<D: DeviceTrait + 'static>(&mut self, _device: D) -> AddResult {
        todo!()
    }

    pub fn remove_device(&mut self, _device_name: &str) -> RemoveResult {
        todo!()
    }

    pub fn get_devices(&self) -> Vec<DeviceType> {
        todo!()
    }

    pub fn get_device(&self, _name: &str) -> DeviceType {
        todo!()
    }
}

pub trait DeviceTrait {
    fn status(&self) -> String;
    fn get_name(&self) -> String;
}

pub struct Device {
    _name: String,
    _description: String,
}

pub struct Socket {
    _device: Device,
    _state: SocketState,
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

pub struct Thermometer {
    _device: Device,
}
impl Thermometer {
    pub fn _new(_name: &str, _description: &str) -> Self {
        todo!()
    }
    fn _get_temp(&self) -> Option<u16> {
        todo!()
    }
}

impl DeviceTrait for Thermometer {
    fn status(&self) -> String {
        todo!()
    }
    fn get_name(&self) -> String {
        todo!()
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
