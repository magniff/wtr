use std::fs;
use std::io::prelude::*;


pub fn read_program(filename: &String) -> Vec<u8> {
    let mut program: Vec<u8> = Vec::new();
    fs::File::open(filename)
        .expect("File lookup")
        .read_to_end(&mut program)
        .expect("Unexpected");
    return program;
}
