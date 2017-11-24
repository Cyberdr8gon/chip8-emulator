
pub struct Chip8Keypad {
    // keypad on off state
    keys: Box<[bool]>,
}

impl Chip8Keypad {
    pub fn new() -> Chip8Keypad {
        Chip8Keypad {
            keys: vec![false; 4096 as usize].into_boxed_slice(),
        }

    }
}
