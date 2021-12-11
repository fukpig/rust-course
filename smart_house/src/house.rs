use crate::room::Room;
//use crate::error::{Error, Result};
use crate::error::{Result};

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

    pub fn add_room(&mut self, _room: Room) -> Result<Room> {
        todo!()
    }

    pub fn remove_room(&mut self, _room_name: &str) -> Result<bool> {
        todo!()
    }

    pub fn get_rooms(&self) -> Vec<Room> {
        todo!()
    }

    pub fn report(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::room::create_room;

    #[test]
    fn test_create_house() {
        let rooms = Vec::<Room>::new();
        let test_house = House {
            _name: "test house".to_string(),
            _rooms: rooms
        };
        let created_house = create_house("test house");
        //assert_eq!(test_house, created_house); mb PartialEq?
        
        assert_eq!(test_house._name, created_house._name);
        assert_eq!(test_house._rooms.len(), created_house._rooms.len());
    }

    #[test]
    fn test_get_house_name() {
        let rooms = Vec::<Room>::new();
        let test_house = House {
            _name: "test house".to_string(),
            _rooms: rooms
        };
        assert_eq!("test_house", test_house.get_name());
    }
    
    #[test]
    fn test_add_room() {
        let rooms = Vec::<Room>::new();
        let mut test_house = House {
            _name: "test house".to_string(),
            _rooms: rooms
        };
        //let room = Room{};
        let room = create_room("test");
        let _add_result = test_house.add_room(room);
        match _add_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_remove_room() {
        let mut rooms = Vec::<Room>::new();
        let room = create_room("test");
        //let room = Room{_name:"test".to_string()};
        rooms.push(room);
        let mut test_house = House {
            _name: "test house".to_string(),
            _rooms: rooms
        };
        let _remove_result = test_house.remove_room("test");
        match _remove_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
    
    #[test]
    fn test_get_rooms() {
        let mut rooms = Vec::<Room>::new();
        //let room = Room{_name:"test".to_string()};
        let room = create_room("test");
        rooms.push(room);
        let test_house = House {
            _name: "test house".to_string(),
            _rooms: rooms
        };
        let rooms = test_house.get_rooms();
        assert_eq!(1, rooms.len());
    }
}
