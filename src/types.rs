use std::num::Wrapping;
use constants;


fn opcode_argument(bytecode: &Vec<u8>, state: &MachineState) -> usize {
    bytecode[state.ppointer+1] as usize * 256 +
    bytecode[state.ppointer+2] as usize
}


fn do_terminate(state: &mut MachineState, bytecode: &Vec<u8>) {
    // just a noop
}


fn do_lshift(state: &mut MachineState, bytecode: &Vec<u8>) {
    let shift_value = opcode_argument(bytecode, state);
    state.shift_head_left(shift_value);
    state.ppointer += constants::LONG_OPCODE;
}


fn do_rshift(state: &mut MachineState, bytecode: &Vec<u8>) {
    let shift_value = opcode_argument(bytecode, state);
    state.shift_head_right(shift_value);
    state.ppointer += constants::LONG_OPCODE;
}


fn do_setup_loop(state: &mut MachineState, bytecode: &Vec<u8>) {
    match state.get_current_cell() {
        0 => {
            state.ppointer +=
                constants::LONG_OPCODE * 2 +
                opcode_argument(bytecode, state);
        },
        _ => state.ppointer += constants::LONG_OPCODE
    }
}


fn do_end_loop(state: &mut MachineState, bytecode: &Vec<u8>) {
    match state.get_current_cell() {
        0 => state.ppointer += constants::LONG_OPCODE,
        _ => {
            state.ppointer -= {
                constants::LONG_OPCODE + opcode_argument(bytecode, state)
            }
        }
    }
}


fn do_inc(state: &mut MachineState, bytecode: &Vec<u8>) {
    let current_cell_value = Wrapping(state.get_current_cell());
    let cell_inc_value = Wrapping(opcode_argument(bytecode, state) as u8);
    state.set_current_cell(current_cell_value + cell_inc_value);
    state.ppointer += constants::LONG_OPCODE;
}


fn do_dec(state: &mut MachineState, bytecode: &Vec<u8>) {
    let current_cell_value = Wrapping(state.get_current_cell());
    let cell_inc_value = Wrapping(opcode_argument(bytecode, state) as u8);
    state.set_current_cell(current_cell_value - cell_inc_value);
    state.ppointer += constants::LONG_OPCODE;
}


fn do_drop(state: &mut MachineState) {
    state.set_current_cell(Wrapping(0));
    state.ppointer += constants::SHORT_OPCODE;
}


fn do_write(state: &mut MachineState) {
    print!("{}", state.get_current_cell() as char);
    state.ppointer += constants::SHORT_OPCODE;
}


pub struct MachineState {
    memory: [Wrapping<u8>; constants::TAPE_LEN],
    pub ppointer: usize,
    pub mpointer: usize,
}


impl MachineState {

    pub fn stepi(&mut self, bytecode: &Vec<u8>) {
        let current_opcode: u8 = bytecode[self.ppointer];
        match current_opcode {
            constants::TERMINATE  => do_terminate(self, bytecode),
            constants::LSHIFT     => do_lshift(self, bytecode),
            constants::RSHIFT     => do_rshift(self, bytecode),
            constants::SETUP_LOOP => do_setup_loop(self, bytecode),
            constants::END_LOOP   => do_end_loop(self, bytecode),
            constants::INC        => do_inc(self, bytecode),
            constants::DEC        => do_dec(self, bytecode),
            constants::DROP       => do_drop(self),
            constants::WRITE      => do_write(self),
            _                     => panic!(),
        }
    }

    pub fn shift_head_right(&mut self, value: usize) {
        self.mpointer = (self.mpointer + value) % (constants::TAPE_LEN + 1)
    }

    pub fn shift_head_left(&mut self, value: usize) {
        if value > self.mpointer {
            self.mpointer = constants::TAPE_LEN - (value - self.mpointer) + 1
        }
        else {
            self.mpointer -= value
        }
    }

    pub fn get_current_cell(&self) -> u8 {
        self.memory[self.mpointer].0
    }

    pub fn set_current_cell(&mut self, value: Wrapping<u8>) {
        self.memory[self.mpointer] = value;
    }

    pub fn new() -> MachineState {
        MachineState {
            memory: [Wrapping(0u8); constants::TAPE_LEN],
            ppointer: 0,
            mpointer: 0,
        }
    }
}


#[cfg(test)]
mod test {
    use super::MachineState;
    use constants::TAPE_LEN;

    #[test]
    fn test_memory_pointer_inc_simple() {
        let mut state = MachineState::new();
        state.shift_head_right(10);
        assert_eq!(state.mpointer, 10);
        state.shift_head_right(TAPE_LEN - 10);
        assert_eq!(state.mpointer, TAPE_LEN);
    }

    #[test]
    fn test_memory_pointer_inc_cycles() {
        let mut state = MachineState::new();
        state.shift_head_right(TAPE_LEN+1);
        assert_eq!(state.mpointer, 0);
        state.shift_head_right(2*TAPE_LEN);
        assert_eq!(state.mpointer, TAPE_LEN-1);
    }

    #[test]
    fn test_memory_pointer_dec_simple() {
        let mut state = MachineState::new();
        state.shift_head_right(1000);
        state.shift_head_left(1000);
        assert_eq!(state.mpointer, 0);
    }

    #[test]
    fn test_memory_pointer_dec_cycles() {
        let mut state = MachineState::new();
        state.shift_head_left(1000);
        assert_eq!(state.mpointer, 29001);
    }
}

