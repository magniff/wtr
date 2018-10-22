use std::env::args;


mod types;
mod constants;
mod common;


fn main() {
    let args: Vec<String> = args().collect();
    let program_binary = common::read_program(&args[1]);
    let mut machine = types::MachineState::new();

    loop {
        machine.stepi(&program_binary);
        if machine.ppointer() == program_binary.len() - 1 {
            break;
        }
    }
}

