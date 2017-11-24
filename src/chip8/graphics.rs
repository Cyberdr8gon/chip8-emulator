
pub struct Chip8Graphics {
    // Graphics Memory
    //
    memory: Box<[bool]>,
}

impl Chip8Graphics {
    pub fn new() -> Chip8Graphics {
        Chip8Graphics {
            memory: vec![false; 64 * 32 as usize].into_boxed_slice(),
        }
    }
}
