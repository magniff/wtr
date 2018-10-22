// Defines a set of constants used by compiler, that is currently written
// in Python.

// Len of machines memory buffer
pub const TAPE_LEN: usize = 30000;

// Opcodes used
pub const SETUP_LOOP: u8 = 0;
pub const END_LOOP: u8 = 1;
pub const INC: u8 = 2;
pub const DEC: u8 = 3;
pub const RSHIFT: u8 = 4;
pub const LSHIFT: u8 = 5;
pub const WRITE: u8 = 6;
pub const READ: u8 = 7;
pub const DROP: u8 = 8;
pub const ADD: u8 = 9;
pub const TERMINATE: u8 = 255;

// Opcode`s widths
pub const LONG_OPCODE: usize = 3;
pub const SHORT_OPCODE: usize = 1;
