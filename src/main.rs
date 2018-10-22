use std::env::args;


mod types;
mod constants;
mod common;


fn main() {
    let args: Vec<String> = args().collect();
    let program_binary = common::read_program(&args[1]);
    let mut machine = types::MachineState::new();

    loop {
        let is_done: bool = machine.stepi(&program_binary);
        if is_done {
            break;
        }
    }
}
