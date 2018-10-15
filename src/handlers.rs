fn opcode_argument(bytecode: &Vec<u8>, program_counter: usize) -> usize {
    return
        bytecode[program_counter+1] as usize * 256 +
        bytecode[program_counter+2] as usize
}


pub fn handle_lshift(
    bytecode: &Vec<u8>, program_counter: mut usize, data_pointer: mut usize) {
    data_pointer -= opcode_argument(bytecode, program_counter);
    program_counter += constants::LONG_OPCODE;
}

