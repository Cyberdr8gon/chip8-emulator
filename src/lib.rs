

pub struct Chip8 {
    cpu: Chip8CPU,
    memory: Chip8Memory,
    graphics: Chip8Graphics,
    keypad: Chip8Keypad,
}

type opcode = u16;


struct Chip8CPU {
    // general purpose registers
    reg_gp: [u8; 16],

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
    stack: [u16; 16]
}

struct Chip8Memory {
    // Main Memory 
    // 4 kilobytes size
    //
    // Memory Map
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    memory: [u8; 4098],
}

struct Chip8Graphics {
    // Graphics Memory
    //
    memory: [bool; 64*32],
}

struct Chip8Keypad {
    // keypad on off state
    keys: [bool; 16],
}
