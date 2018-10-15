use std::ops;
use constants;


pub struct MonoidalAddress(usize);


impl MonoidalAddress {
    pub fn new(value: usize) -> MonoidalAddress {
        MonoidalAddress(value % constants::TAPE_LEN)
    }
}


impl ops::Add for MonoidalAddress {
    type Output = MonoidalAddress;

    fn add(self, other: MonoidalAddress) -> MonoidalAddress {
        MonoidalAddress(self.0 + other.0)
    }
}


impl ops::Sub for MonoidalAddress {
    type Output = MonoidalAddress;

    fn sub(self, other: MonoidalAddress) -> MonoidalAddress {
        if other.0 > self.0 {
            MonoidalAddress(constants::TAPE_LEN - other.0 + self.0)
        }
        else {
            MonoidalAddress(self.0 - other.0)
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

