use crate::devices::Device;
use crate::devices::DeviceTrait;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone)]
pub struct Thermometer {
    pub device: Device,
}

impl Thermometer {
    pub fn new(name: &str, description: &str) -> Self {
        Thermometer {
            device: Device {
                name: name.to_string(),
                description: description.to_string(),
            },
        }
    }
    fn get_temp(&self) -> Option<u16> {
        let mut rng = thread_rng();
        Some(rng.gen_range(1..41))
    }
}

impl DeviceTrait for Thermometer {
    fn status(&self) -> String {
        let temp = self.get_temp().unwrap_or(0);
        return format!("Thermometer - name:{}, temp: {}", self.get_name(), temp);
    }
    fn get_name(&self) -> String {
        self.device.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thermometer() {
        let thermometer = Thermometer::new("test", "test description");
        assert_eq!("test".to_string(), thermometer.device.name);
        assert_eq!(
            "test description".to_string(),
            thermometer.device.description
        );
    }

    #[test]
    fn test_get_temp() {
        let thermometer = Thermometer::new("test", "test description");
        let tempreture = thermometer.get_temp();
        match tempreture {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }

    #[test]
    fn test_get_status() {
        let thermometer = Thermometer::new("test", "test description");
        let status = thermometer.status();
        if status.contains("Thermometer - name:") && status.contains("temp: ") {
            assert!(true);
            return;
        }
        assert!(false);
    }

    #[test]
    fn test_get_name() {
        let thermometer = Thermometer::new("test", "test description");
        let name = thermometer.get_name();
        assert_eq!("test".to_string(), name);
    }
}
