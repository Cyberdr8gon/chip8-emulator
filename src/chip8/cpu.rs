use super::memory::*;
use super::graphics::*;
use super::keypad::*;
type opcode = u16;


pub struct Chip8CPU {
    // general purpose registers
    reg_gp: Box<[u8]>,

    // Usually used to store memory addresses
    reg_i: u8,

    // Special 8-bit registers
    reg_delay: u8,
    reg_sound: u8,

    // Program Counter
    reg_pc: u8,

    // Stack Pointer
    reg_sp: u8,
    // Stack
    stack: Box<[u8]>,

}


impl Chip8CPU {
    pub fn new() -> Chip8CPU {
        Chip8CPU {
            reg_gp: vec![0; 16 as usize].into_boxed_slice(),
            reg_i: 0,

            reg_delay: 0,
            reg_sound: 0,

            reg_pc: 0,

            reg_sp: 0,
            stack: vec![0; 16 as usize].into_boxed_slice(),



        }
    }
}
