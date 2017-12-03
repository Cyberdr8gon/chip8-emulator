
#[derive(Debug)]
pub struct Chip8Keypad {
    // keypad on off state
    pub keys: Box<[bool]>,
}

impl Chip8Keypad {
    pub fn new() -> Chip8Keypad {
        Chip8Keypad {
            keys: vec![false; 16 as usize].into_boxed_slice(),
        }

    }

    pub fn is_pressed(&self, key_index: u8) -> bool {
        (*self.keys)[key_index as usize]
    }
}
