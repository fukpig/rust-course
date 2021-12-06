use crate::devices::DeviceTrait;
use crate::devices::Device;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thermometer() {
        let thermometer = Thermometer::_new("test", "test description");
        assert_eq!("test".to_string(), thermometer._device._name);
        assert_eq!("test description".to_string(), thermometer._device._description);
    }

    #[test]
    fn test_get_temp() {
        let thermometer = Thermometer::_new("test", "test description");
        let tempreture = thermometer._get_temp();
        match tempreture {
            Some(_) => assert!(true),
            None => assert!(false)
        }
    }
    
    #[test]
    fn test_get_status() {
        let thermometer = Thermometer::_new("test", "test description");
        let status = thermometer.status();
        if status.contains("tempreture is ") {
            assert!(true);
        }
        assert!(false);
    }    
    
    #[test]
    fn test_get_name() {
        let thermometer = Thermometer::_new("test", "test description");
        let name = thermometer.get_name();
        assert_eq!("test".to_string(), name);
    }    
}
