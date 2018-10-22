mod types;
mod constants;
mod handlers;
mod common;
// juse std::num::Wrapping;
// juse std::env;
// j
// j
// jmod constants;
// juse constants::{
// j    TERMINATE,
// j    SETUP_LOOP,
// j    END_LOOP,
// j    INC,
// j    DEC,
// j    RSHIFT,
// j    LSHIFT,
// j    DROP,
// j    ADD,
// j    WRITE,
// j    SHORT_OPCODE,
// j    LONG_OPCODE,
// j};
// j
// j
// jmod common;
// juse common::opcode_argument;
// j
// j
// jmod types;
// j
// j
// jfn interprete_code(bytecode: &Vec<u8>) {
// j    let mut program_counter: usize = 0;
// j    let mut data_pointer: usize = 0;
// j    let mut memory = [MonoidalAddress::new(0u8); 30000];
// j
// j    loop {
// j        let current_opcode = bytecode[program_counter];
// j        if current_opcode == TERMINATE {
// j            break
// j        }
// j        else {
// j            match current_opcode {
// j                // '<' n times
// j                LSHIFT => {
// j                    data_pointer -= opcode_argument(bytecode, program_counter);
// j                    program_counter += LONG_OPCODE;
// j                },
// j                // '>' n times
// j                RSHIFT => {
// j                    data_pointer += opcode_argument(bytecode, program_counter);
// j                    program_counter += LONG_OPCODE;
// j                },
// j                // '[' variation, that is aware about its corresponding ']'
// j                SETUP_LOOP => {
// j                    if memory[data_pointer].0 == 0 {
// j                        program_counter +=
// j                            LONG_OPCODE * 2 +
// j                            opcode_argument(bytecode, program_counter);
// j                    }
// j                    else {
// j                        program_counter += LONG_OPCODE;
// j                    }
// j                },
// j                // ']' variation, that is aware about its corresponding '['
// j                END_LOOP => {
// j                    if memory[data_pointer].0 == 0 {
// j                        program_counter += LONG_OPCODE;
// j                    }
// j                    else {
// j                        program_counter -=
// j                            LONG_OPCODE +
// j                            opcode_argument(bytecode, program_counter);
// j                    }
// j                },
// j                // '+' n times
// j                INC => {
// j                    memory[data_pointer] +=
// j                        Wrapping(
// j                            opcode_argument(bytecode, program_counter) as u8
// j                        );
// j                    program_counter += LONG_OPCODE;
// j                },
// j                // '-' n times
// j                DEC => {
// j                    memory[data_pointer] -= Wrapping(
// j                        opcode_argument(bytecode, program_counter) as u8
// j                    );
// j                    program_counter += LONG_OPCODE;
// j                },
// j                ADD => {
// j                    let lookup_data_pointer: usize;
// j                    if bytecode[program_counter+1] == 0 {
// j                        lookup_data_pointer =
// j                            data_pointer + bytecode[program_counter+2] as usize;
// j                    }
// j                    else {
// j                        if bytecode[program_counter+2] as usize > data_pointer {
// j                            lookup_data_pointer =
// j                                30000 - bytecode[program_counter+2] as usize - data_pointer;
// j                        }
// j                        else {
// j                            lookup_data_pointer = data_pointer - bytecode[program_counter+2] as usize;
// j                        }
// j                    }
// j                    memory[lookup_data_pointer] += memory[data_pointer];
// j                    memory[data_pointer] = Wrapping(0);
// j                    program_counter += LONG_OPCODE;
// j                },
// j                // '[-]' as a single instruction
// j                DROP => {
// j                    memory[data_pointer] = Wrapping(0);
// j                    program_counter += SHORT_OPCODE;
// j                },
// j                // '.'
// j                WRITE => {
// j                    print!("{}", memory[data_pointer].0 as char);
// j                    program_counter += SHORT_OPCODE;
// j                },
// j                _ => {
// j                    panic!("Unknown opcode");
// j                }
// j            }
// j        }
// j    }
// j}
// j
// j
// jfn main() {
// j    let args: Vec<String> = env::args().collect();
// j    let program_binary: Vec<u8> = common::read_program(&args[1]);
// j    interprete_code(&program_binary)
// j}
// j
