use chip8::memory::Chip8Memory;

pub const CHIP8_SCREEN_WIDTH: usize = 64;
pub const CHIP8_SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Chip8Graphics {
    // Graphics Memory
    //
    memory: Box<[bool]>,
}

impl Chip8Graphics {
    pub fn new() -> Chip8Graphics {
        Chip8Graphics {
            memory: vec![false; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT].into_boxed_slice(),
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(CHIP8_SCREEN_HEIGHT * CHIP8_SCREEN_WIDTH) {
            (*self.memory)[i] = false;
        }
    }

    pub fn get_pixel_value(&self, x: usize, y: usize) -> bool {
        (*self.memory)[self.get_index(x, y)]
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * CHIP8_SCREEN_WIDTH + x
    }

    // returns if there was a collision
    pub fn draw_sprite(&mut self, 
                       memory: &Box<[u8]>, 
                       x:usize, 
                       y:usize, 
                       mem_location: u16, 
                       length: u8) -> bool
    {
        let mut collision = false;

        for i in 0..length {
            let unseperatedBits = (*memory)[(mem_location + (i as u16)) as usize];
            let mut bits: [bool; 8] = [false; 8];

            bits[0] = 0x1 & unseperatedBits != 0;
            bits[1] = ((0x2 & unseperatedBits)  >> 1) != 0;
            bits[2] = ((0x4 & unseperatedBits)  >> 2) != 0;
            bits[3] = ((0x8 & unseperatedBits)  >> 3) != 0;
            bits[4] = ((0x10 & unseperatedBits) >> 4) != 0;
            bits[5] = ((0x20 & unseperatedBits) >> 5) != 0;
            bits[6] = ((0x40 & unseperatedBits) >> 6) != 0;
            bits[7] = ((0x80 & unseperatedBits) >> 7) != 0;

            for j in 0..8 {
                if (self.get_index(x + (j as usize), y + (i as usize)) < 2048) {
                    if (self.get_pixel_value(x + (j as usize), y + (i as usize))) && bits[j] {
                        collision = true;
                        (*self.memory)[self.get_index(x + (j as usize), y + (i as usize))] = false;
                    } else if bits[j] {
                        (*self.memory)[self.get_index(x + (j as usize), y + (i as usize))] = true;
                    }
                } else {
                    collision = true;
                }
                
            }
        }

        collision
    }
}
