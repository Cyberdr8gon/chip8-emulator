
pub struct Chip8Memory {
    // Main Memory 
    // 4 kilobytes size
    //
    // Memory Map
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    memory: Box<[u8]>,
}

impl Chip8Memory {
    pub fn new() -> Chip8Memory {

        Chip8Memory {
            // initializing with all 0s this could be wrong
            // TODO check this
            memory: vec![0; 4096 as usize].into_boxed_slice(),
        }
    }
}
