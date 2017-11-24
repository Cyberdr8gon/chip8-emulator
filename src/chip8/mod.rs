extern crate sdl2;

mod cpu;
mod memory;
mod graphics;
mod keypad;

use self::cpu::*;
use self::memory::*;
use self::graphics::*;
use self::keypad::*;


pub struct Chip8 {
    cpu: Chip8CPU,
    memory: Chip8Memory,
    graphics: Chip8Graphics,
    keypad: Chip8Keypad,
}


impl Chip8 {

    pub fn new() -> Chip8 {
        let memory = Chip8Memory::new();
        let graphics = Chip8Graphics::new();
        let keypad = Chip8Keypad::new();
        let cpu = Chip8CPU::new();
        Chip8 {
            cpu,
            memory,
            graphics,
            keypad,
        }
    }

    pub fn boot(&mut self, rom: &Box<[u8]>) {

    }

    pub fn step(&mut self) {
        
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        
    }

    pub fn do_key_event(&self, key: u8) {
        
    }
}

