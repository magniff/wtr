use std::ops;
use std::cmp;


use constants;


#[derive(Debug)]
pub struct MonoidalAddress(usize);


impl MonoidalAddress {
    pub fn new(value: usize) -> MonoidalAddress {
        MonoidalAddress(value % 30001)
    }
}


impl cmp::PartialEq for MonoidalAddress {
    fn eq(&self, other: &MonoidalAddress) -> bool {
        self.0 == other.0
    }
}



impl ops::Add for MonoidalAddress {
    type Output = MonoidalAddress;

    fn add(self, other: MonoidalAddress) -> MonoidalAddress {
        MonoidalAddress((self.0 + other.0) % 30001)
    }
}


impl ops::Sub for MonoidalAddress {
    type Output = MonoidalAddress;

    fn sub(self, other: MonoidalAddress) -> MonoidalAddress {
        match other.0 > self.0 {
            true => {
                MonoidalAddress(1 + constants::TAPE_LEN - other.0 + self.0)
            },
            false => MonoidalAddress(self.0 - other.0),
        }
    }

}


pub struct MachineState {
    pub program_counter: usize,
    pub memory_pointer: MonoidalAddress,
    pub buffer: [u8; constants::TAPE_LEN],
}


impl MachineState {

    pub fn new() -> MachineState {
        return MachineState {
            program_counter: 0,
            memory_pointer: MonoidalAddress(0),
            buffer: [0u8; constants::TAPE_LEN],
        }
    }
}


#[cfg(test)]
mod test {
    use super::MonoidalAddress;

    #[test]
    fn test_monoidal_address_new() {
        assert_eq!(MonoidalAddress::new(0), MonoidalAddress(0));
        assert_eq!(MonoidalAddress::new(100), MonoidalAddress(100));
        assert_eq!(MonoidalAddress::new(30000), MonoidalAddress(30000));
        assert_eq!(MonoidalAddress::new(60000), MonoidalAddress(29999));
    }

    #[test]
    fn monoidal_addresses_add() {
        assert_eq!(
            MonoidalAddress(30000) + MonoidalAddress(1), MonoidalAddress(0)
        );
        assert_eq!(
            MonoidalAddress(29999) + MonoidalAddress(10), MonoidalAddress(8)
        );
    }

    #[test]
    fn monoidal_addresses_sub() {
        assert_eq!(
            MonoidalAddress(30000) - MonoidalAddress(1), MonoidalAddress(29999)
        );
        assert_eq!(
            MonoidalAddress(0) - MonoidalAddress(1), MonoidalAddress(30000)
        );
    }
}

