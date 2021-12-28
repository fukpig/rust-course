use crate::devices::DeviceType;
use crate::room::Room;
//use crate::error::{Error, Result};
use crate::error::Error::AlreadyExist;
use crate::error::Error::NotFound;
use crate::error::Result;

pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: &str) -> House {
        House {
            name: name.to_string(),
            rooms: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_room(&mut self, room: Room) -> Result<&Room> {
        let check_index = self
            .rooms
            .iter()
            .position(|x| x.get_name() == room.get_name());
        match check_index {
            Some(_) => return Err(AlreadyExist(room.get_name())),
            None => {}
        }
        self.rooms.push(room);
        let recently_added = self.rooms.last().unwrap();
        Ok(recently_added)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Result<Room> {
        let check_index = self.rooms.iter().position(|x| x.get_name() == room_name);
        let index = match check_index {
            Some(i) => i,
            None => return Err(NotFound(room_name.to_string())),
        };
        let room = self.rooms[index].clone();
        self.rooms.remove(index);
        Ok(room)
    }

    pub fn get_rooms(&self) -> impl Iterator<Item = &Room> {
        //let rooms:Vec<Room> = self.rooms.iter().cloned().collect();
        self.rooms.iter()
    }

    pub fn report(&self) {
        println!("House: {}", self.name);
        for r in self.get_rooms() {
            println!("Room: {}", r.get_name());
            for d in r.get_devices() {
                match d {
                    DeviceType::Socket(_) => println!("Get socket"),
                    DeviceType::Thermometer(_) => println!("Get thermometer"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_house() {
        let rooms = Vec::<Room>::new();
        let test_house = House {
            name: "test house".to_string(),
            rooms: rooms,
        };
        let created_house = House::new("test house");
        //assert_eq!(test_house, created_house); mb PartialEq?

        assert_eq!(test_house.name, created_house.name);
        assert_eq!(test_house.rooms.len(), created_house.rooms.len());
    }

    #[test]
    fn test_get_house_name() {
        let rooms = Vec::<Room>::new();
        let test_house = House {
            name: "test house".to_string(),
            rooms: rooms,
        };
        assert_eq!("test house", test_house.get_name());
    }

    #[test]
    fn test_add_room() {
        let rooms = Vec::<Room>::new();
        let mut test_house = House {
            name: "test house".to_string(),
            rooms: rooms,
        };
        let room = Room::new("test");
        let add_result = test_house.add_room(room);
        match add_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_remove_room() {
        let mut rooms = Vec::<Room>::new();
        let room = Room::new("test");
        //let room = Room{_name:"test".to_string()};
        rooms.push(room);
        let mut test_house = House {
            name: "test house".to_string(),
            rooms: rooms,
        };
        let remove_result = test_house.remove_room("test");
        match remove_result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_get_rooms() {
        let mut rooms = Vec::<Room>::new();
        //let room = Room{_name:"test".to_string()};
        let room = Room::new("test");
        rooms.push(room);
        let test_house = House {
            name: "test house".to_string(),
            rooms: rooms,
        };
        let rooms = test_house.get_rooms();
        assert_eq!(1, rooms.count());
    }
}
