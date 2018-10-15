use std::num::Wrapping;
use std::env;


mod constants;
use constants::{
    TERMINATE,
    SETUP_LOOP,
    END_LOOP,
    INC,
    DEC,
    RSHIFT,
    LSHIFT,
    DROP,
    ADD,
    WRITE,
    SHORT_OPCODE,
    LONG_OPCODE,
};


mod common;
use common::opcode_argument;


mod types;


fn interprete_code(bytecode: &Vec<u8>) {
    let mut program_counter: usize = 0;
    let mut data_pointer: usize = 0;
    let mut memory = [Wrapping(0u8); 30000];

    loop {
        let current_opcode = bytecode[program_counter];
        if current_opcode == TERMINATE {
            break
        }
        else {
            match current_opcode {
                // '<' n times
                LSHIFT => {
                    data_pointer -= opcode_argument(bytecode, program_counter);
                    program_counter += LONG_OPCODE;
                },
                // '>' n times
                RSHIFT => {
                    data_pointer += opcode_argument(bytecode, program_counter);
                    program_counter += LONG_OPCODE;
                },
                // '[' variation, that is aware about its corresponding ']'
                SETUP_LOOP => {
                    if memory[data_pointer].0 == 0 {
                        program_counter +=
                            LONG_OPCODE * 2 +
                            opcode_argument(bytecode, program_counter);
                    }
                    else {
                        program_counter += LONG_OPCODE;
                    }
                },
                // ']' variation, that is aware about its corresponding '['
                END_LOOP => {
                    if memory[data_pointer].0 == 0 {
                        program_counter += LONG_OPCODE;
                    }
                    else {
                        program_counter -=
                            LONG_OPCODE +
                            opcode_argument(bytecode, program_counter);
                    }
                },
                // '+' n times
                INC => {
                    memory[data_pointer] +=
                        Wrapping(
                            opcode_argument(bytecode, program_counter) as u8
                        );
                    program_counter += LONG_OPCODE;
                },
                // '-' n times
                DEC => {
                    memory[data_pointer] -= Wrapping(
                        opcode_argument(bytecode, program_counter) as u8
                    );
                    program_counter += LONG_OPCODE;
                },
                ADD => {
                    let lookup_data_pointer: usize;
                    if bytecode[program_counter+1] == 0 {
                        lookup_data_pointer =
                            data_pointer + bytecode[program_counter+2] as usize;
                    }
                    else {
                        if bytecode[program_counter+2] as usize > data_pointer {
                            lookup_data_pointer =
                                30000 - bytecode[program_counter+2] as usize - data_pointer;
                        }
                        else {
                            lookup_data_pointer = data_pointer - bytecode[program_counter+2] as usize;
                        }
                    }
                    memory[lookup_data_pointer] += memory[data_pointer];
                    memory[data_pointer] = Wrapping(0);
                    program_counter += LONG_OPCODE;
                },
                // '[-]' as a single instruction
                DROP => {
                    memory[data_pointer] = Wrapping(0);
                    program_counter += SHORT_OPCODE;
                },
                // '.'
                WRITE => {
                    print!("{}", memory[data_pointer].0 as char);
                    program_counter += SHORT_OPCODE;
                },
                _ => {
                    panic!("Unknown opcode");
                }
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program_binary: Vec<u8> = common::read_program(&args[1]);
    interprete_code(&program_binary)
}

