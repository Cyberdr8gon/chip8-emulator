use chip8::rand::random;

use super::memory::*;
use super::graphics::*;
use super::keypad::*;
use super::Chip8;
use super::Chip8Bus;

type Opcode = u16;

pub enum IntermediateAsm {
    CLS,
    RET,
    SYS {addr: u16},
    JUMP {addr: u16},
    CALL {addr: u16},
    SE_CONST {reg_index: u8, constant: u8},
    SNE_CONST {reg_index: u8, constant: u8},
    SE_REG {reg_x_index: u8, reg_y_index: u8},
    LOAD_CONST {reg_index: u8, constant: u8},
    ADD_CONST {reg_index: u8, constant: u8},
    LOAD_REG {reg_x_index: u8, reg_y_index: u8},
    OR {reg_x_index: u8, reg_y_index: u8},
    AND {reg_x_index: u8, reg_y_index: u8},
    XOR {reg_x_index: u8, reg_y_index: u8},
    ADD_REG {reg_x_index: u8, reg_y_index: u8},
    SUB_REG {reg_x_index: u8, reg_y_index: u8},
    SHR {reg_x_index: u8, reg_y_index: u8},
    SUBN {reg_x_index: u8, reg_y_index: u8},
    SHL {reg_x_index: u8, reg_y_index: u8},
    SNE_REG {reg_x_index: u8, reg_y_index: u8},
    LOAD_ADDR {addr: u16},
    JUMP_V0 {addr: u16},
    RND {reg_x_index: u8, bitmask: u8},
    DRW {reg_x_index: u8, reg_y_index: u8, nibble: u8},
    SKP {reg_x_index: u8},
    SKNP {reg_x_index: u8},
    LOAD_DELAY_TIMER {reg_x_index: u8},
    WAIT_FOR_KEY_PRESS {reg_x_index: u8},
    SET_DELAY_TIMER {reg_x_index: u8},
    SET_SOUND_TIMER {reg_x_index: u8},
    ADD_I {reg_x_index: u8},
    LOAD_SPRITE_LOCATION {reg_x_index: u8},
    STORE_BCD {reg_x_index: u8},
    STORE_REG_ARR {reg_x_index: u8},
    LOAD_REG_ARR {reg_x_index: u8},

}


#[derive(Debug)]
pub struct Chip8CPU {
    // general purpose registers
    pub reg_gp: Box<[u8]>,

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

    // Flag to denote if the timer needs to be updated
    timer_update_flag: u8,

    // a flag to halt operation until a event that wakes the
    // cpu (i.e. keydown)
    pub is_halted_flag: bool,
    pub halted_register: u8,

    pub draw_to_screen_flag: bool,

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

            is_halted_flag: false,

            halted_register: 0,

            draw_to_screen_flag: true,


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
        println!("{:x}", opcode);
        opcode
    }

    pub fn decode_opcode(&mut self, memory_ref: &mut Chip8Memory, opcode: Opcode) -> IntermediateAsm {
        let lowest_4_bits: u8 = (opcode & 0xF) as u8;
        let middle_lower_4_bits: u8 = ((opcode >> 4 ) & 0xF) as u8;
        let middle_upper_4_bits: u8 = ((opcode >> 8 ) & 0xF) as u8;
        let highest_4_bits: u8 = ((opcode >> 12 ) & 0xF) as u8;
        match (highest_4_bits, 
               middle_upper_4_bits, 
               middle_lower_4_bits, 
               lowest_4_bits)
        {
            (0x0, 0x0, 0xE, 0x0) => {
                // Opcode: 00E0
                // Type: Display
                // C Psuedo:
                // disp_clear();
                // TODO Clear the screen
                println!("CLS");
                IntermediateAsm::CLS
            },
            (0x0, 0x0, 0xE, 0xE) => {
                // Opcode: 00EE
                // Type: Flow
                // C Psuedo:
                // return;
                // TODO Return from a subroutine
                println!("RET");
                IntermediateAsm::RET
            },
            (0x0, N1, N2, N3) => {
                // Opcode: 00EE
                // Type: Call
                // C Psuedo:
                // Not Applicable
                // TODO Call RCA 1802 program at address N1N2N3
                println!("SYS");
                IntermediateAsm::SYS {addr: opcode & 0x0fff}
            },
            (0x1, N1, N2, N3) => {
                // Opcode: 1NNN
                // Type: Flow
                // C Psuedo:
                // goto NNN;
                // TODO jump to address N1N2N3
                let addr = opcode & 0x0fff;
                println!("JUMP {:x}", addr);
                IntermediateAsm::JUMP {addr: addr}
            },
            (0x2, N1, N2, N3) => {
                // Opcode: 2NNN
                // Type: Flow
                // C Psuedo:
                // *(0xNNN)()
                // TODO Call subroutine at N1N2N3
                let addr = opcode & 0x0fff;
                println!("CALL {:x}", addr);
                IntermediateAsm::CALL {addr: addr}

            },
            (0x3, X, N1, N2) => {
                // Opcode: 3XNN
                // Type: Cond
                // C Psuedo:
                // if(Vx==NN)
                // TODO Skip the next instruction if register VX equals NN.
                let reg_index = ((opcode & 0x0f00) >> 8) as u8 ; 
                let constant = (opcode & 0x00ff) as u8;
                println!("SE_CONST X{:?}, {:?}", reg_index, constant);
                IntermediateAsm::SE_CONST {
                    reg_index: reg_index,
                    constant: constant
                }
            },
            (0x4, X, N1, N2) => {
                // Opcode: 4XNN
                // Type: Cond
                // C Psuedo:
                // if(Vx!=NN)
                // TODO Skip the next instruction if register VX does not equal NN.
                let reg_index = ((opcode & 0x0f00) >> 8) as u8;
                let constant = (opcode & 0x00ff) as u8;
                println!("SNE_CONST X{:?}, {:?}", reg_index, constant);
                IntermediateAsm::SNE_CONST {
                    reg_index: reg_index,
                    constant: constant,
                }
            }, 
            (0x5, X, Y, 0x0) => {
                // Opcode: 5XY0
                // Type: Cond
                // C Psuedo:
                // if(Vx==Vy)
                // TODO Skip the next instruction if register VX does not 
                // equal register VY.
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SE_REG X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SE_REG {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }
            }, 
            (0x6, X, N1, N2) => {
                // Opcode: 6XNN
                // Type: Const
                // C Psuedo:
                // Vx = NN
                // TODO Set register VX to NN
                let reg_index = ((opcode & 0x0f00) >> 8) as u8;
                let constant = (opcode & 0x00ff) as u8;
                println!("LOAD_CONST X{:?}, {:?}", reg_index, constant);
                IntermediateAsm::LOAD_CONST {
                    reg_index: reg_index,
                    constant: constant,
                }
            },
            (0x7, X, N1, N2) => {
                // Opcode: 7XNN
                // Type: Const
                // C Psuedo:
                // Vx += NN
                // TODO Add NN to Vx (carry flag is not changed)
                let reg_index = ((opcode & 0x0f00) >> 8) as u8;
                let constant = (opcode & 0x00ff) as u8;
                println!("ADD_CONST X{:?}, {:?}", reg_index, constant);
                IntermediateAsm::ADD_CONST {
                    reg_index: reg_index,
                    constant: constant,
                }
            }, 
            (0x8, X, Y, 0x0) => {
                // Opcode: 8XY0
                // Type: Assign
                // C Psuedo:
                // Vx = Vy
                // TODO set register Vx to the value in register Vy
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("LOAD_REG X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::LOAD_REG {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0x1) => {
                // Opcode: 8XY1
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx|Vy
                // TODO Set register Vx to Vx | Vy 
                // (bitwise OR)
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("OR X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::OR {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0x2) => {
                // Opcode: 8XY2
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx&Vy
                // TODO Set register Vx to Vx & Vy 
                // (bitwise AND)
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("AND X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::AND {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }
            },
            (0x8, X, Y, 0x3) => {
                // Opcode: 8XY3
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx^Vy
                // TODO Set register Vx to Vx ^ Vy 
                // (bitwise XOR)
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("XOR X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::XOR {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0x4) => {
                // Opcode: 8XY4
                // Type: Math
                // C Psuedo:
                // Vx+=Vy
                // TODO Set register Vx to Vx + Vy 
                // If there is a carry, set register VF to 1
                // else, set register VF to 0
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("ADD_REG X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::ADD_REG {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }
            },
            (0x8, X, Y, 0x5) => {
                // Opcode: 8XY5
                // Type: Math
                // C Psuedo:
                // Vx-=Vy
                // TODO Set register Vx to Vx - Vy 
                // If there is a borrow, set register VF to 0
                // else, set register VF to 1
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SUB_REG X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SUB_REG {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0x6) => {
                // Opcode: 8XY6
                // Type: BitOp 
                // C Psuedo:
                // Vx=Vy=Vy>>1
                // TODO Shift Vy right by one and copy the result
                // to Vx.
                // VF is set to the value of the least significant bit
                // of Vy before the shift
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SHR X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SHR {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0x7) => {
                // Opcode: 8XY7
                // Type: Math
                // C Psuedo:
                // Vx=Vy-Vx
                // TODO Set Vx to Vy minux Vx. VF is set to 0 when there
                // is a borrow and 1 when there isn't
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SUBN X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SUBN {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }

            },
            (0x8, X, Y, 0xE) => {
                // Opcode: 8XYE
                // Type: BitOp
                // C Psuedo:
                // Vx=Vy=Vy<<1
                // TODO Shift Vy left by one and copy the result to Vx.
                // Set VF to the value of the most significant bit
                // of Vy before the shift
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SHL X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SHL {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }
            },
            (0x9, X, Y, 0) => {
                // Opcode: 9XY0
                // Type: Cond
                // C Psuedo:
                // if(Vx != Vy)
                // TODO Skip the next instruction if Vx doesn't euqal Vy.
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                println!("SNE_REG X{:?}, X{:?}", reg_x_index, reg_y_index);
                IntermediateAsm::SNE_REG {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                }
            },
            (0xA, N1, N2, N3) => {
                // Opcode: ANNN
                // Type: MEM
                // C Psuedo:
                // I=NNN
                // TODO set reg I to the address NNN
                let addr = opcode & 0x0fff;
                println!("LOAD_ADDR {:x}", addr);
                IntermediateAsm::LOAD_ADDR {addr: addr}
            },
            (0xB, N1, N2, N3) => {
                // Opcode: BNNN
                // Type: Flow
                // C Psuedo:
                // PC=V0+NNN
                // TODO jump to the address NNN plus V0
                let addr = opcode & 0x0fff;
                println!("JUMP_V0 {:x}", addr);
                IntermediateAsm::JUMP_V0 {addr: addr}
            },
            (0xC, X, N1, N2) => {
                // Opcode: CXNN
                // Type: Rand
                // C Psuedo:
                // Vx=rand()&NN
                // TODO set Vx to a random number (typically 0 to 255) 
                // that is bitwise and'd with NN
                let reg_index = ((opcode & 0x0f00) >> 8) as u8;
                let constant = (opcode & 0x00ff) as u8;
                println!("RND X{:?}, {:?}", reg_index, constant);
                IntermediateAsm::RND {
                    reg_x_index: reg_index,
                    bitmask: constant
                }
            },
            (0xD, X, Y, N) => {
                // Opcode: DXYN
                // Type: Disp
                // C Psuedo:
                // draw(Vx,Vy,N)
                // TODO Draws a sprite at coordinate (VX, VY) that has 
                // a width of 8 pixels and a height of N pixels. 
                // Each row of 8 pixels is read as bit-coded starting 
                // from memory location I; I value doesn’t change after 
                // the execution of this instruction. 
                // As described above, VF is set to 1 if any screen pixels 
                // are flipped from set to unset when the sprite is drawn, 
                // and to 0 if that doesn’t happen
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                let reg_y_index = ((opcode & 0x00f0) >> 4) as u8;
                let nibble = (opcode & 0x000f) as u8;
                println!("DRW X{:?}, X{:?}, n: {:?}", reg_x_index, reg_y_index, nibble);
                IntermediateAsm::DRW {
                    reg_x_index: reg_x_index,
                    reg_y_index: reg_y_index,
                    nibble: nibble,
                }

            },
            (0xE, X, 0x9, 0xE) => {
                // Opcode: EX9E
                // Type: KeyOp
                // C Psuedo:
                // if(key()==Vx)
                // TODO Skips the next instruction if the key stored in 
                // VX is pressed.
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("SKP X{:?}", reg_x_index);
                IntermediateAsm::SKP {
                    reg_x_index: reg_x_index,
                }

            },
            (0xE, X, 0xA, 0x1) => {
                // Opcode: EXA1
                // Type: KeyOp
                // C Psuedo:
                // if(key()!=Vx)
                // TODO Skips the next instruction if the key stored in 
                // VX isn't pressed.
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("SKNP X{:?}", reg_x_index);
                IntermediateAsm::SKNP {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x0, 0x7) => {
                // Opcode: FX07
                // Type: Timer
                // C Psuedo:
                // Vx = get_delay()
                // TODO Sets Vx to the value of the delay timer
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("LOAD_DELAY_TIMER X{:?}", reg_x_index);
                IntermediateAsm::LOAD_DELAY_TIMER {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x0, 0xA) => {
                // Opcode: FX0A
                // Type: KeyOp
                // C Psuedo:
                // Vx = get_key()
                // TODO A key press is awaited, and then stored in VX. 
                // NOTE!!! Blocking Operation. 
                // All instruction halted until next key event
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("WAIT_FOR_KEY_PRESS X{:?}", reg_x_index);
                IntermediateAsm::WAIT_FOR_KEY_PRESS {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x1, 0x5) => {
                // Opcode: FX15
                // Type: Timer
                // C Psuedo:
                // delay_timer(Vx)
                // TODO set delay timer to Vx
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("SET_DELAY_TIMER X{:?}", reg_x_index);
                IntermediateAsm::SET_DELAY_TIMER {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x1, 0x8) => {
                // Opcode: FX18
                // Type: Sound
                // C Psuedo:
                // sound_timer(Vx)
                // TODO set sound timer to Vx
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("SET_SOUND_TIMER X{:?}", reg_x_index);
                IntermediateAsm::SET_SOUND_TIMER {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x1, 0xE) => {
                // Opcode: FX1E
                // Type: MEM
                // C Psuedo:
                // I += Vx
                // TODO Add Vx to I
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("ADD_I X{:?}", reg_x_index);
                IntermediateAsm::ADD_I {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x2, 0x9) => {
                // Opcode: FX29
                // Type: MEM
                // C Psuedo:
                // I=sprite_addr[Vx]
                // TODO Sets I to the location of the sprite for the 
                // character in VX. Characters 0-F (in hexadecimal) 
                // are represented by a 4x5 font.
                //
                // TODO Do more research here
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("LOAD_SPRITE_LOCATION X{:?}", reg_x_index);
                IntermediateAsm::LOAD_SPRITE_LOCATION {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x3, 0x3) => {
                // Opcode: FX33
                // Type: BCD
                // C Psuedo:
                // set_BCD(Vx);
                // *(I+0)=BCD(3);
                // *(I+1)=BCD(2);
                // *(I+2)=BCD(1); 
                //
                // TODO Stores the binary-coded decimal representation of VX, 
                // with the most significant of three digits at the address 
                // in I, the middle digit at I plus 1, and the least 
                // significant digit at I plus 2. 
                // (In other words, take the decimal representation of VX, 
                // place the hundreds digit in memory at location in I, 
                // the tens digit at location I+1, and the ones digit at 
                // location I+2.)
                //
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("STORE_BCD X{:?}", reg_x_index);
                IntermediateAsm::STORE_BCD {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x5, 0x5) => {
                // Opcode: FX55
                // Type: MEM
                // C Psuedo:
                // reg_dump(Vx,&I)
                // TODO Stores V0 to VX (including VX) in memory starting 
                // at address I. I is increased by 1 for each value written.
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("STORE_REG_ARR X{:?}", reg_x_index);
                IntermediateAsm::STORE_REG_ARR {
                    reg_x_index: reg_x_index,
                }

            },
            (0xF, X, 0x6, 0x5) => {
                // Opcode: FX65
                // Type: MEM
                // C Psuedo:
                // reg_load(Vx,&I)
                // TODO Fills V0 to VX (including VX) with values from 
                // memory starting at address I. I is increased by 1 for 
                // each value written. 
                let reg_x_index = ((opcode & 0x0f00) >> 8) as u8; 
                println!("LOAD_REG_ARR X{:?}", reg_x_index);
                IntermediateAsm::LOAD_REG_ARR {
                    reg_x_index: reg_x_index,
                }

            },

            // TODO add error passing to print full debug info on failiure
            _ => panic!("Error: Illegal Instruction: {:x} is not a Chip-8 Instruction. \n {:?}", opcode, self)
        }
    }

    pub fn execute_opcode(&mut self, bus_ref: &mut Chip8Bus, instruction: IntermediateAsm) {
        match instruction {
            IntermediateAsm::CLS => {
                bus_ref.graphics.clear();
                self.draw_to_screen_flag = true;
                // TODO check this is the proper program counter advancement
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::RET => {
                if self.reg_sp == 0 {
                    panic!("Error: Attempted to return with no return with a empty stack\nCPU State: \n{:?}", self);
                }
                // TODO do return
                self.reg_pc = (*self.stack)[self.reg_sp as usize];
                self.reg_sp = self.reg_sp - 1;
            },
            IntermediateAsm::SYS { addr } => {
                panic!("Error: Unimplemented Instruction \nAttempted to call SYS instruction. \nCPU State: \n{:?}", self);
            },
            IntermediateAsm::JUMP { addr } => {
                self.reg_pc = addr;
            },
            IntermediateAsm::CALL { addr } => {
                self.reg_sp = self.reg_sp + 1;
                (*self.stack)[self.reg_sp as usize] = self.reg_pc + 2;
                self.reg_pc = addr;
            },
            IntermediateAsm::SE_CONST { reg_index, constant } => {
                if (*self.reg_gp)[reg_index as usize] == constant  {
                    self.reg_pc = self.reg_pc + 4;
                } else {
                    self.reg_pc = self.reg_pc + 2;
                }
            },
            IntermediateAsm::SNE_CONST {reg_index, constant} => {
                if (*self.reg_gp)[reg_index as usize] == constant {
                    self.reg_pc = self.reg_pc + 2;
                } else {
                    self.reg_pc = self.reg_pc + 4;
                }
            },
            IntermediateAsm::SE_REG { reg_x_index, reg_y_index } => {
                if (*self.reg_gp)[reg_x_index as usize] 
                        == (*self.reg_gp)[reg_y_index as usize]  {
                    self.reg_pc = self.reg_pc + 4;
                } else {
                    self.reg_pc = self.reg_pc + 2;
                }
            },
            IntermediateAsm::LOAD_CONST {reg_index, constant} => {
                (*self.reg_gp)[reg_index as usize] = constant;
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::ADD_CONST {reg_index, constant} => {
                (*self.reg_gp)[reg_index as usize] = ((*self.reg_gp)[reg_index as usize] as u16 + constant as u16) as u8;
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::LOAD_REG {reg_x_index, reg_y_index} => {
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize];
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::OR {reg_x_index, reg_y_index} => {
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize] 
                                                | (*self.reg_gp)[reg_x_index as usize];
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::AND {reg_x_index, reg_y_index} => {
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize] 
                                                & (*self.reg_gp)[reg_x_index as usize];
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::XOR {reg_x_index, reg_y_index} => {
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize] 
                                                ^ (*self.reg_gp)[reg_x_index as usize];
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::ADD_REG {reg_x_index, reg_y_index} => {
                let src: u16 = (*self.reg_gp)[reg_x_index as usize] as u16;
                let dst: u16 = (*self.reg_gp)[reg_x_index as usize] as u16;
                let sum: u16 = src + dst;
                if (sum & 0xffffff00) != 0  {
                    (*self.reg_gp)[0xf] = 1;
                } else {
                    (*self.reg_gp)[0xf] = 0;
                }

                (*self.reg_gp)[reg_x_index as usize] = (((*self.reg_gp)[reg_y_index as usize] as u16)
                                                + ((*self.reg_gp)[reg_x_index as usize] as u16)) as u8;
                self.reg_pc = self.reg_pc + 2;

            },
            IntermediateAsm::SUB_REG {reg_x_index, reg_y_index} => {
                if (*self.reg_gp)[reg_x_index as usize] > (*self.reg_gp)[reg_y_index as usize]  {
                    (*self.reg_gp)[0xf] = 1;
                } else {
                    (*self.reg_gp)[0xf] = 0;
                }

                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_x_index as usize] 
                                                - (*self.reg_gp)[reg_y_index as usize];
                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::SHR {reg_x_index, reg_y_index} => {
                if ((*self.reg_gp)[reg_y_index as usize] & 0x1 ) != 0  {
                    (*self.reg_gp)[0xf] = 1;
                } else {
                    (*self.reg_gp)[0xf] = 0;
                }
                (*self.reg_gp)[reg_y_index as usize] = (*self.reg_gp)[reg_y_index as usize] >> 1;
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize];

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::SUBN {reg_x_index, reg_y_index} => {
                if (*self.reg_gp)[reg_y_index as usize] > (*self.reg_gp)[reg_x_index as usize]  {
                    (*self.reg_gp)[0xf] = 1;
                } else {
                    (*self.reg_gp)[0xf] = 0;
                }

                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize] 
                                                - (*self.reg_gp)[reg_x_index as usize];
                self.reg_pc = self.reg_pc + 2;

            },
            IntermediateAsm::SHL {reg_x_index, reg_y_index} => {
                if (*self.reg_gp)[reg_y_index as usize] & 0x80 != 0 {
                    (*self.reg_gp)[0xf] = 1;
                } else {
                    (*self.reg_gp)[0xf] = 0;
                }
                (*self.reg_gp)[reg_y_index as usize] = (*self.reg_gp)[reg_y_index as usize] << 1;
                (*self.reg_gp)[reg_x_index as usize] = (*self.reg_gp)[reg_y_index as usize];

                self.reg_pc = self.reg_pc + 2;
                
            },
            IntermediateAsm::SNE_REG {reg_x_index, reg_y_index} => {
                if (*self.reg_gp)[reg_y_index as usize] == (*self.reg_gp)[reg_x_index as usize]  {
                    self.reg_pc = self.reg_pc + 2;
                } else {
                    self.reg_pc = self.reg_pc + 4;
                }
            },
            IntermediateAsm::LOAD_ADDR {addr} => {
                self.reg_i = addr;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::JUMP_V0 {addr} => {
                self.reg_pc = addr + ((*self.reg_gp)[0] as u16);
            },
            IntermediateAsm::RND {reg_x_index, bitmask} => {
                let rand255:u8 = random::<u8>();
                let postAND = rand255 & bitmask;
                (*self.reg_gp)[reg_x_index as usize] = postAND;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::DRW {reg_x_index, reg_y_index, nibble} => {
                let mut memory_ref = &bus_ref.memory.memory;
                let x = (*self.reg_gp)[reg_x_index as usize];
                let y = (*self.reg_gp)[reg_y_index as usize];
                let hadCollision = bus_ref.graphics.draw_sprite(memory_ref, x as usize, y as usize, self.reg_i, nibble);

                if hadCollision {
                    (*self.reg_gp)[0xf] = 0x1;
                } else {
                    (*self.reg_gp)[0xf] = 0x0;
                }

                self.draw_to_screen_flag = true;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::SKP {reg_x_index} => {
                if  bus_ref.keypad.is_pressed((*self.reg_gp)[reg_x_index as usize])  {
                    self.reg_pc = self.reg_pc + 4;
                } else {
                    self.reg_pc = self.reg_pc + 2;
                }
            },
            IntermediateAsm::SKNP {reg_x_index} => {
                if  bus_ref.keypad.is_pressed((*self.reg_gp)[reg_x_index as usize])  {
                    self.reg_pc = self.reg_pc + 2;
                } else {
                    self.reg_pc = self.reg_pc + 4;
                }
            },
            IntermediateAsm::LOAD_DELAY_TIMER {reg_x_index} => {
                (*self.reg_gp)[reg_x_index as usize] = self.reg_delay;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::WAIT_FOR_KEY_PRESS {reg_x_index} => {
                self.halted_register = reg_x_index;
                self.is_halted_flag = true;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::SET_DELAY_TIMER {reg_x_index} => {
                self.reg_delay = (*self.reg_gp)[reg_x_index as usize];

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::SET_SOUND_TIMER {reg_x_index} => {
                self.reg_sound = (*self.reg_gp)[reg_x_index as usize];

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::ADD_I {reg_x_index} => {
                self.reg_i = ((*self.reg_gp)[reg_x_index as usize] as u16) + self.reg_i;

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::LOAD_SPRITE_LOCATION {reg_x_index} => {
                let hexval = (*self.reg_gp)[reg_x_index as usize];

                self.reg_i = match hexval {
                    0x0 => 0x0,
                    0x1 => 0x5,
                    0x2 => 0xa,
                    0x3 => 0xf,
                    0x4 => 0x14,
                    0x5 => 0x19,
                    0x6 => 0x1e,
                    0x7 => 0x23,
                    0x8 => 0x28,
                    0x9 => 0x2d,
                    0xa => 0x32,
                    0xb => 0x37,
                    0xc => 0x3c,
                    0xd => 0x41,
                    0xe => 0x46,
                    0xf => 0x4B,
                    _ => panic!("Error: Attempted to read system character that doesn't exist.")
                };

                self.reg_pc = self.reg_pc + 2;
            },
            IntermediateAsm::STORE_BCD {reg_x_index} => {
                let value = (*self.reg_gp)[reg_x_index as usize];
                (bus_ref.memory.memory)[(self.reg_i as usize)] = value / 100;
                (bus_ref.memory.memory)[(self.reg_i as usize) + 1] = (value % 100) /10;
                (bus_ref.memory.memory)[(self.reg_i as usize) + 2] = value % 10;

                self.reg_pc = self.reg_pc + 2;
            },

            IntermediateAsm::STORE_REG_ARR {reg_x_index} => {
                for i in 0..(reg_x_index + 1) {
                    (*bus_ref.memory.memory)[(self.reg_i as usize) + (i as usize)] = (*self.reg_gp)[i as usize];
                }

                self.reg_pc = self.reg_pc + 2;
            },

            IntermediateAsm::LOAD_REG_ARR {reg_x_index} => {
                for i in 0..(reg_x_index + 1) {
                     (*self.reg_gp)[i as usize] = (*bus_ref.memory.memory)[(self.reg_i as usize) + (i as usize)] ;
                }

                self.reg_pc = self.reg_pc + 2;
            },



        }

        

    }

    

    pub fn update_timer(&mut self) {
        self.timer_update_flag = (self.timer_update_flag + 1) % 9;
        if self.timer_update_flag == 0 {
            if self.reg_delay > 0 {
                self.reg_delay = self.reg_delay - 1;
            }
            if self.reg_sound > 0 {
                self.reg_sound = self.reg_sound - 1;
            }
        }
    }

}
