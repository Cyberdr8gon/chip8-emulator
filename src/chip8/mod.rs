extern crate sdl2;
extern crate rand;

mod cpu;
mod memory;
mod graphics;
mod keypad;

use self::cpu::*;
use self::memory::*;
use self::graphics::*;
use self::keypad::*;

use chip8::graphics::CHIP8_SCREEN_HEIGHT;
use chip8::graphics::CHIP8_SCREEN_WIDTH;

use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;

// TODO add global constants for CPU clock speed and screen refresh rate

#[derive(Debug)]
pub struct Chip8 {
    cpu: Chip8CPU,
    bus: Chip8Bus,
}



impl Chip8 {

    pub fn new() -> Chip8 {
        let cpu = Chip8CPU::new();
        let bus = Chip8Bus::new();
        Chip8 {
            cpu,
            bus
        }
    }

    pub fn boot(&mut self, rom: &Vec<u8>) {
        let mut i = 0x200;
        for byte in rom {
            (*self.bus.memory.memory)[i] = *byte;
            i = i + 1;
        }

        // hex character font
        
        // "0" character
        // 0x0 address
        (*self.bus.memory.memory)[0] = 0xF0; 
        (*self.bus.memory.memory)[1] = 0x90; 
        (*self.bus.memory.memory)[2] = 0x90; 
        (*self.bus.memory.memory)[3] = 0x90; 
        (*self.bus.memory.memory)[4] = 0xF0; 

        // "1" character
        // 0x5
        (*self.bus.memory.memory)[5] = 0x20; 
        (*self.bus.memory.memory)[6] = 0x60; 
        (*self.bus.memory.memory)[7] = 0x20; 
        (*self.bus.memory.memory)[8] = 0x20; 
        (*self.bus.memory.memory)[9] = 0x70; 

        // "2" character
        // 0xa
        (*self.bus.memory.memory)[10] = 0xF0; 
        (*self.bus.memory.memory)[11] = 0x10; 
        (*self.bus.memory.memory)[12] = 0xF0; 
        (*self.bus.memory.memory)[13] = 0x80; 
        (*self.bus.memory.memory)[14] = 0xF0; 

        // "3" character
        // 0xf
        (*self.bus.memory.memory)[15] = 0xF0; 
        (*self.bus.memory.memory)[16] = 0x10; 
        (*self.bus.memory.memory)[17] = 0xF0; 
        (*self.bus.memory.memory)[18] = 0x10; 
        (*self.bus.memory.memory)[19] = 0xF0; 
        
        // "4" character
        // 0x14
        (*self.bus.memory.memory)[20] = 0x90; 
        (*self.bus.memory.memory)[21] = 0x90; 
        (*self.bus.memory.memory)[22] = 0xF0; 
        (*self.bus.memory.memory)[23] = 0x10; 
        (*self.bus.memory.memory)[24] = 0x10; 
        
        // "5" character
        // 0x19
        (*self.bus.memory.memory)[25] = 0xF0; 
        (*self.bus.memory.memory)[26] = 0x80; 
        (*self.bus.memory.memory)[27] = 0xF0; 
        (*self.bus.memory.memory)[28] = 0x10; 
        (*self.bus.memory.memory)[29] = 0xF0; 

        // "6" character
        // 0x1E
        (*self.bus.memory.memory)[30] = 0xF0; 
        (*self.bus.memory.memory)[31] = 0x80; 
        (*self.bus.memory.memory)[32] = 0xF0; 
        (*self.bus.memory.memory)[33] = 0x90; 
        (*self.bus.memory.memory)[34] = 0xF0; 

        // "7" character
        // 0x23
        (*self.bus.memory.memory)[35] = 0xF0; 
        (*self.bus.memory.memory)[36] = 0x10; 
        (*self.bus.memory.memory)[37] = 0x20; 
        (*self.bus.memory.memory)[38] = 0x40; 
        (*self.bus.memory.memory)[39] = 0x40; 

        // "8" character
        // 0x28
        (*self.bus.memory.memory)[40] = 0xF0; 
        (*self.bus.memory.memory)[41] = 0x90; 
        (*self.bus.memory.memory)[42] = 0xF0; 
        (*self.bus.memory.memory)[43] = 0x90; 
        (*self.bus.memory.memory)[44] = 0xF0; 

        // "9" character
        // 0x2D
        (*self.bus.memory.memory)[45] = 0xF0; 
        (*self.bus.memory.memory)[46] = 0x90; 
        (*self.bus.memory.memory)[47] = 0xF0; 
        (*self.bus.memory.memory)[48] = 0x10; 
        (*self.bus.memory.memory)[49] = 0xF0; 

        // "A" character
        // 0x32
        (*self.bus.memory.memory)[50] = 0xF0; 
        (*self.bus.memory.memory)[51] = 0x90; 
        (*self.bus.memory.memory)[52] = 0xF0; 
        (*self.bus.memory.memory)[53] = 0x90; 
        (*self.bus.memory.memory)[54] = 0x90; 

        // "B" character
        // 0x37
        (*self.bus.memory.memory)[55] = 0xE0; 
        (*self.bus.memory.memory)[56] = 0x90; 
        (*self.bus.memory.memory)[57] = 0xE0; 
        (*self.bus.memory.memory)[58] = 0x90; 
        (*self.bus.memory.memory)[59] = 0xE0; 

        // "C" character
        // 0x3C
        (*self.bus.memory.memory)[60] = 0xF0; 
        (*self.bus.memory.memory)[61] = 0x80; 
        (*self.bus.memory.memory)[62] = 0x80; 
        (*self.bus.memory.memory)[63] = 0x80; 
        (*self.bus.memory.memory)[64] = 0xF0; 

        // "D" character
        // 0x41
        (*self.bus.memory.memory)[65] = 0xE0; 
        (*self.bus.memory.memory)[66] = 0x90; 
        (*self.bus.memory.memory)[67] = 0x90; 
        (*self.bus.memory.memory)[68] = 0x90; 
        (*self.bus.memory.memory)[69] = 0xE0; 

        // "E" character
        // 0x46
        (*self.bus.memory.memory)[70] = 0xD0; 
        (*self.bus.memory.memory)[71] = 0x80; 
        (*self.bus.memory.memory)[72] = 0xF0; 
        (*self.bus.memory.memory)[73] = 0x80; 
        (*self.bus.memory.memory)[74] = 0xF0; 

        // "F" character
        // 0x4B
        (*self.bus.memory.memory)[75] = 0xF0; 
        (*self.bus.memory.memory)[76] = 0x80; 
        (*self.bus.memory.memory)[77] = 0xF0; 
        (*self.bus.memory.memory)[78] = 0x80; 
        (*self.bus.memory.memory)[79] = 0x80; 


    }

    pub fn step(&mut self) {
        if !self.cpu.is_halted_flag  {
            self.cpu.step(&mut self.bus);
            //println!("{:?}", self);
        }

    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.cpu.draw_to_screen_flag {
            // TODO render graphics memory state to screen
            canvas.set_draw_color(Color::RGB(0,0,0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(255,255,255));
            for y in 0..CHIP8_SCREEN_HEIGHT {
                for x in 0..CHIP8_SCREEN_WIDTH {
                    if self.bus.graphics.get_pixel_value(x, y) {
                        canvas.fill_rect(Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10));
                    }
                }
            }

            canvas.present();

            self.cpu.draw_to_screen_flag = false;


            //println!("{:?}", self.bus.graphics);
        }
        
    }

    // !!!!NOTE!!!! this is the only interface to wake the CPU!!!!
    pub fn do_key_event(&mut self, key: u8) {
        let keyLong = key as usize;
        if (*self.bus.keypad.keys)[keyLong] {
            self.bus.keypad.keys[keyLong] = false;
        } else {
            if self.cpu.is_halted_flag  {
                self.cpu.is_halted_flag = false;
                (*self.cpu.reg_gp)[self.cpu.halted_register as usize] = key;

            }
            self.bus.keypad.keys[keyLong] = true;
        }
    }
}

#[derive(Debug)]
pub struct Chip8Bus {
    memory: Chip8Memory,
    graphics: Chip8Graphics,
    keypad: Chip8Keypad,
}

impl Chip8Bus {
    fn new() -> Chip8Bus {
        let memory = Chip8Memory::new();
        let graphics = Chip8Graphics::new();
        let keypad = Chip8Keypad::new();
        Chip8Bus {
            memory,
            graphics,
            keypad,

        }
    }
}
