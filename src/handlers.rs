fn opcode_argument(bytecode: &Vec<u8>, program_counter: usize) -> usize {
    return
        bytecode[program_counter+1] as usize * 256 +
        bytecode[program_counter+2] as usize
}

