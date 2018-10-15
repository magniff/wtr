use std::fs;
use std::io::prelude::*;


pub fn opcode_argument(bytecode: &Vec<u8>, program_counter: usize) -> usize {
    return
        bytecode[program_counter+1] as usize * 256 +
        bytecode[program_counter+2] as usize
}


pub fn read_program(filename: &String) -> Vec<u8> {
    let mut program: Vec<u8> = Vec::new();

    fs::File::open(filename)
        .expect("File lookup")
        .read_to_end(&mut program)
        .expect("Unexpected");

    return program;
}


