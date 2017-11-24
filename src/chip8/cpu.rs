use super::memory::*;
use super::graphics::*;
use super::keypad::*;
use super::Chip8;
use super::Chip8Bus;

type Opcode = u16;

enum Asm {

}


#[derive(Debug)]
pub struct Chip8CPU {
    // general purpose registers
    reg_gp: Box<[u8]>,

    // Usually used to store memory addresses
    reg_i: u16,

    // Special 8-bit registers
    reg_delay: u8,
    reg_sound: u8,

    // Program Counter
    reg_pc: u16,

    // Stack Pointer
    reg_sp: u16,
    // Stack
    stack: Box<[u16]>,

    timer_update_flag: u8,

}


impl Chip8CPU {
    pub fn new() -> Chip8CPU {
        Chip8CPU {
            reg_gp: vec![0; 16 as usize].into_boxed_slice(),
            reg_i: 0,

            reg_delay: 0,
            reg_sound: 0,

            reg_pc: 0x200,

            reg_sp: 0,
            stack: vec![0; 16 as usize].into_boxed_slice(),

            timer_update_flag: 9,


        }
    }

    pub fn step(&mut self, bus_ref: &mut Chip8Bus) 
    {
        let opcode = self.fetch_opcode(&mut bus_ref.memory);
        let instruction = self.decode_opcode(&mut bus_ref.memory, opcode);
        self.execute_opcode(bus_ref, instruction);
        self.update_timer();
    }

    pub fn fetch_opcode(&mut self, memory_ref: &mut Chip8Memory) -> Opcode {
        let mut opcode: Opcode = 0x0;
        opcode = memory_ref.read_byte(self.reg_pc) as u16;
        opcode = (opcode << 8) | memory_ref.read_byte(self.reg_pc + 1) as u16;
        println!("{:?}", opcode);
        opcode
    }

    pub fn decode_opcode(&mut self, memory_ref: &mut Chip8Memory, opcode: Opcode) -> Asm {
        let lowest_4_bits: u8 = (opcode & 0xF) as u8;
        let middle_lower_4_bits: u8 = ((opcode >> 4 ) & 0xF) as u8;
        let middle_upper_4_bits: u8 = ((opcode >> 8 ) & 0xF) as u8;
        let highest_4_bits: u8 = ((opcode >> 12 ) & 0xF) as u8;
        match (highest_4_bits, 
               middle_upper_4_bits, 
               middle_lower_4_bits, 
               lowest_4_bits)
        {
            // TODO add error passing to print full debug info on failiure
            (0x0, 0x0, 0xE, 0x0) => {
                // TODO Clear screen
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // TODO Return from a subroutine
            }
            _ => panic!("Error: Illegal Instruction: {:x} is not a Chip-8 Instruction.", opcode)
        }
    }

    pub fn execute_opcode(&mut self, bus_ref: &mut Chip8Bus, instruction: Asm) {

    }

    pub fn update_timer(&mut self) {
        self.timer_update_flag = (self.timer_update_flag + 1) % 9
    }

}
