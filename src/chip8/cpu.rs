use super::memory::*;
use super::graphics::*;
use super::keypad::*;
use super::Chip8;
use super::Chip8Bus;

type Opcode = u16;

enum IntermediateAsm {
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

    // Flag to denote if the timer needs to be updated
    timer_update_flag: u8,

    // a flag to halt operation until a event that wakes the
    // cpu (i.e. keydown)
    pub is_halted_flag: bool,

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
        println!("{:?}", opcode);
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
                IntermediateAsm::CLS
            },
            (0x0, 0x0, 0xE, 0xE) => {
                // Opcode: 00EE
                // Type: Flow
                // C Psuedo:
                // return;
                // TODO Return from a subroutine
                IntermediateAsm::RET
            },
            (0x0, N1, N2, N3) => {
                // Opcode: 00EE
                // Type: Call
                // C Psuedo:
                // Not Applicable
                // TODO Call RCA 1802 program at address N1N2N3
                IntermediateAsm::SYS {addr: opcode & 0x0fff}
            },
            (0x1, N1, N2, N3) => {
                // Opcode: 1NNN
                // Type: Flow
                // C Psuedo:
                // goto NNN;
                // TODO jump to address N1N2N3
                IntermediateAsm::JUMP {addr: opcode & 0x0fff}
            },
            (0x2, N1, N2, N3) => {
                // Opcode: 2NNN
                // Type: Flow
                // C Psuedo:
                // *(0xNNN)()
                // TODO Call subroutine at N1N2N3
                IntermediateAsm::CALL {addr: opcode & 0x0fff}

            },
            (0x3, X, N1, N2) => {
                // Opcode: 3XNN
                // Type: Cond
                // C Psuedo:
                // if(Vx==NN)
                // TODO Skip the next instruction if register VX equals NN.
                IntermediateAsm::SE_CONST {
                    reg_index: ((opcode & 0x0f00) >> 8) as u8 , 
                    constant: (opcode & 0x00ff) as u8
                }
            },
            (0x4, X, N1, N2) => {
                // Opcode: 4XNN
                // Type: Cond
                // C Psuedo:
                // if(Vx!=NN)
                // TODO Skip the next instruction if register VX does not equal NN.
                IntermediateAsm::SNE_CONST {
                    reg_index: ((opcode & 0x0f00) >> 8)  as u8,
                    constant: (opcode & 0x00ff) as u8 
                }
            }, 
            (0x5, X, Y, 0x0) => {
                // Opcode: 5XY0
                // Type: Cond
                // C Psuedo:
                // if(Vx==Vy)
                // TODO Skip the next instruction if register VX does not 
                // equal register VY.
                IntermediateAsm::SE_REG {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8,
                }
            }, 
            (0x6, X, N1, N2) => {
                // Opcode: 6XNN
                // Type: Const
                // C Psuedo:
                // Vx = NN
                // TODO Set register VX to NN
                IntermediateAsm::LOAD_CONST {
                    reg_index: ((opcode & 0x0f00) >> 8) as u8, 
                    constant: (opcode & 0x00ff) as u8
                }
            },
            (0x7, X, N1, N2) => {
                // Opcode: 7XNN
                // Type: Const
                // C Psuedo:
                // Vx += NN
                // TODO Add NN to Vx (carry flag is not changed)
                IntermediateAsm::ADD_CONST {
                    reg_index: ((opcode & 0x0f00) >> 8) as u8, 
                    constant: (opcode & 0x00ff) as u8
                }
            }, 
            (0x8, X, Y, 0x0) => {
                // Opcode: 8XY0
                // Type: Assign
                // C Psuedo:
                // Vx = Vy
                // TODO set register Vx to the value in register Vy
                IntermediateAsm::LOAD_REG {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }

            },
            (0x8, X, Y, 0x1) => {
                // Opcode: 8XY1
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx|Vy
                // TODO Set register Vx to Vx | Vy 
                // (bitwise OR)
                IntermediateAsm::OR {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }

            },
            (0x8, X, Y, 0x2) => {
                // Opcode: 8XY2
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx&Vy
                // TODO Set register Vx to Vx & Vy 
                // (bitwise AND)
                IntermediateAsm::AND {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }
            },
            (0x8, X, Y, 0x3) => {
                // Opcode: 8XY3
                // Type: BitOp
                // C Psuedo:
                // Vx=Vx^Vy
                // TODO Set register Vx to Vx ^ Vy 
                // (bitwise XOR)
                IntermediateAsm::XOR {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
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
                IntermediateAsm::ADD_REG {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
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
                IntermediateAsm::SUB_REG {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
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
                IntermediateAsm::SHR {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }

            },
            (0x8, X, Y, 0x7) => {
                // Opcode: 8XY7
                // Type: Math
                // C Psuedo:
                // Vx=Vy-Vx
                // TODO Set Vx to Vy minux Vx. VF is set to 0 when there
                // is a borrow and 1 when there isn't
                IntermediateAsm::SUBN {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
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
                IntermediateAsm::SHL {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }
            },
            (0x9, X, Y, 0) => {
                // Opcode: 9XY0
                // Type: Cond
                // C Psuedo:
                // if(Vx != Vy)
                // TODO Skip the next instruction if Vx doesn't euqal Vy.
                IntermediateAsm::SNE_REG {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8
                }
            },
            (0xA, N1, N2, N3) => {
                // Opcode: ANNN
                // Type: MEM
                // C Psuedo:
                // I=NNN
                // TODO set reg I to the address NNN
                IntermediateAsm::LOAD_ADDR {addr: opcode & 0x0fff}
            },
            (0xB, N1, N2, N3) => {
                // Opcode: BNNN
                // Type: Flow
                // C Psuedo:
                // PC=V0+NNN
                // TODO jump to the address NNN plus V0
                IntermediateAsm::JUMP_V0 {addr: opcode & 0x0fff}
            },
            (0xC, X, N1, N2) => {
                // Opcode: CXNN
                // Type: Rand
                // C Psuedo:
                // Vx=rand()&NN
                // TODO set Vx to a random number (typically 0 to 255) 
                // that is bitwise and'd with NN
                IntermediateAsm::RND {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    bitmask: (opcode & 0x00ff) as u8
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
                IntermediateAsm::DRW {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8, 
                    reg_y_index: ((opcode & 0x00f0) >> 4) as u8,
                    nibble: (opcode & 0x000f) as u8
                }

            },
            (0xE, X, 0x9, 0xE) => {
                // Opcode: EX9E
                // Type: KeyOp
                // C Psuedo:
                // if(key()==Vx)
                // TODO Skips the next instruction if the key stored in 
                // VX is pressed.
                IntermediateAsm::SKP {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xE, X, 0xA, 0x1) => {
                // Opcode: EXA1
                // Type: KeyOp
                // C Psuedo:
                // if(key()!=Vx)
                // TODO Skips the next instruction if the key stored in 
                // VX isn't pressed.
                IntermediateAsm::SKNP {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xF, X, 0x0, 0x7) => {
                // Opcode: FX07
                // Type: Timer
                // C Psuedo:
                // Vx = get_delay()
                // TODO Sets Vx to the value of the delay timer
                IntermediateAsm::LOAD_DELAY_TIMER {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
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
                IntermediateAsm::WAIT_FOR_KEY_PRESS {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xF, X, 0x1, 0x5) => {
                // Opcode: FX15
                // Type: Timer
                // C Psuedo:
                // delay_timer(Vx)
                // TODO set delay timer to Vx
                IntermediateAsm::SET_DELAY_TIMER {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xF, X, 0x1, 0x8) => {
                // Opcode: FX18
                // Type: Sound
                // C Psuedo:
                // sound_timer(Vx)
                // TODO set sound timer to Vx
                IntermediateAsm::SET_SOUND_TIMER {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xF, X, 0x1, 0xE) => {
                // Opcode: FX1E
                // Type: MEM
                // C Psuedo:
                // I += Vx
                // TODO Add Vx to I
                IntermediateAsm::ADD_I {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
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
                IntermediateAsm::LOAD_SPRITE_LOCATION {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
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
                IntermediateAsm::STORE_BCD {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },
            (0xF, X, 0x5, 0x5) => {
                // Opcode: FX55
                // Type: MEM
                // C Psuedo:
                // reg_dump(Vx,&I)
                // TODO Stores V0 to VX (including VX) in memory starting 
                // at address I. I is increased by 1 for each value written.
                IntermediateAsm::STORE_REG_ARR {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
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
                IntermediateAsm::LOAD_REG_ARR {
                    reg_x_index: ((opcode & 0x0f00) >> 8) as u8
                }

            },

            // TODO add error passing to print full debug info on failiure
            _ => panic!("Error: Illegal Instruction: {:x} is not a Chip-8 Instruction.", opcode)
        }
    }

    pub fn execute_opcode(&mut self, bus_ref: &mut Chip8Bus, instruction: IntermediateAsm) {

    }

    pub fn update_timer(&mut self) {
        self.timer_update_flag = (self.timer_update_flag + 1) % 9
    }

}
