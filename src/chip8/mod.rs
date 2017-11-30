extern crate sdl2;

mod cpu;
mod memory;
mod graphics;
mod keypad;

use self::cpu::*;
use self::memory::*;
use self::graphics::*;
use self::keypad::*;

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
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        
    }

    pub fn do_key_event(&self, key: u8) {
        
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
