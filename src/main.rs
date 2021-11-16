pub enum DeviceType {
    _Socket,
    _Thermometer,
}

enum SocketState {
    _Enabled,
    _Disabled,
}

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

    pub fn add_room(&mut self, _room: Room) {
        todo!()
    }

    pub fn remove_room(&mut self, _room_name: &str) {
        todo!()
    }

    pub fn list_rooms(&self) {
        todo!()
    }

    pub fn report(&self) {
        todo!()
    }
}

pub struct Room {
    _name: String,
    _devices: Vec<Box<dyn DeviceTrait>>,
}

pub fn create_room(_name: &str) -> Room {
    todo!()
}

impl Room {
    pub fn get_name(&self) -> &String {
        todo!()
    }

    pub fn add_device<D: DeviceTrait + 'static>(&mut self, _device: D) {
        todo!()
    }

    pub fn remove_device(&mut self, _device_name: &str) {
        todo!()
    }

    pub fn list_devices(&self) {
        todo!()
    }
}

pub fn create_socket(_name: &str, _device_type: DeviceType, _description: &str) -> Socket {
    todo!()
}

pub trait DeviceTrait {
    fn status(&self) -> String;
    fn get_name(&self) -> String;
}

pub struct Device {
    _name: String,
    _description: String,
    _device_type: DeviceType,
}

pub struct Socket {
    _device: Device,
    _state: SocketState,
}

impl Socket {
    pub fn _interact(&mut self) {
        todo!()
    }

    fn _get_voltage(&self) -> u16 {
        todo!()
    }
}
pub fn create_thermometer(
    _name: &str,
    _device_type: DeviceType,
    _description: &str,
) -> Thermometer {
    todo!()
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
    fn _get_temp(&self) -> u16 {
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

fn main() {
    /*let mut house = create_house("home");
    println!("House name:{}", house.get_name());
    let mut hall = create_room("hall");
    house.add_room(hall);
    house.list_rooms();
    let mut socket = create_socket("socket hall", DeviceType::Socket, "typical socket");
    socket.status();
    socket.interact();
    socket.status();

    let thermometer = create_thermometer("thermo", DeviceType::Thermometer, "typical thermo");
    thermometer.status();

    hall.add_device(socket);
    hall.add_device(thermometer);
    hall.list_devices();

    house.report();
    hall.remove_device("socket hall");
    hall.remove_device("socket hall"); //Error NOT found
    hall.list_devices();

    house.report();
    house.remove_room("hall");
    house.report();*/
}
