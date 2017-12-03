extern crate chip8_emulator;
extern crate sdl2;

// std lib
use std::env;
use std::io::Read;
use std::fs::File;

// sdl imports
use sdl2::pixels::Color;
use sdl2::event::Event; use sdl2::keyboard::Keycode;
use std::time::Duration;

mod chip8;
use chip8::Chip8;

const SDL_SCREEN_WIDTH: u32 = 640;
const SDL_SCREEN_HEIGHT: u32 = 320;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Error, no rom file specified.");
        return
    }
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Rusty Chip-8 Emulator", SDL_SCREEN_WIDTH, SDL_SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    let mut file = File::open(&args[1]).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    //println!("{:?}", file_buffer);

    let mut chip8_vm = chip8::Chip8::new();
    chip8_vm.boot(&file_buffer);

    //println!("{:?}", chip8_vm);
    
    //println!("{:?}", rom);
    
    // see rust-sdl2 docs

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // step emulation 
        chip8_vm.step();

        // replace the below with a transfer from graphics memory
        // to the sdl screen

        // keyboard events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => {
                            chip8_vm.do_key_event(0);
                        },
                        Keycode::Num2 => {
                            chip8_vm.do_key_event(1);
                        },
                        Keycode::Num3 => {
                            chip8_vm.do_key_event(2);
                        },
                        Keycode::Num4 => {
                            chip8_vm.do_key_event(3);
                        },
                        Keycode::Q => {
                            chip8_vm.do_key_event(4);
                        },
                        Keycode::W => {
                            chip8_vm.do_key_event(5);
                        },
                        Keycode::E => {
                            chip8_vm.do_key_event(6);
                        },
                        Keycode::R => {
                            chip8_vm.do_key_event(7);
                        },
                        Keycode::A => {
                            chip8_vm.do_key_event(8);
                        },
                        Keycode::S => {
                            chip8_vm.do_key_event(9);
                        },
                        Keycode::D => {
                            chip8_vm.do_key_event(10);
                        },
                        Keycode::F => {
                            chip8_vm.do_key_event(11);
                        },
                        Keycode::Z => {
                            chip8_vm.do_key_event(12);
                        },
                        Keycode::X => {
                            chip8_vm.do_key_event(13);
                        },
                        Keycode::C => {
                            chip8_vm.do_key_event(14);
                        },
                        Keycode::V => {
                            chip8_vm.do_key_event(15);
                        },
                        _ => {}
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => {
                            chip8_vm.do_key_event(0);
                        },
                        Keycode::Num2 => {
                            chip8_vm.do_key_event(1);
                        },
                        Keycode::Num3 => {
                            chip8_vm.do_key_event(2);
                        },
                        Keycode::Num4 => {
                            chip8_vm.do_key_event(3);
                        },
                        Keycode::Q => {
                            chip8_vm.do_key_event(4);
                        },
                        Keycode::W => {
                            chip8_vm.do_key_event(5);
                        },
                        Keycode::E => {
                            chip8_vm.do_key_event(6);
                        },
                        Keycode::R => {
                            chip8_vm.do_key_event(7);
                        },
                        Keycode::A => {
                            chip8_vm.do_key_event(8);
                        },
                        Keycode::S => {
                            chip8_vm.do_key_event(9);
                        },
                        Keycode::D => {
                            chip8_vm.do_key_event(10);
                        },
                        Keycode::F => {
                            chip8_vm.do_key_event(11);
                        },
                        Keycode::Z => {
                            chip8_vm.do_key_event(12);
                        },
                        Keycode::X => {
                            chip8_vm.do_key_event(13);
                        },
                        Keycode::C => {
                            chip8_vm.do_key_event(14);
                        },
                        Keycode::V => {
                            chip8_vm.do_key_event(15);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        chip8_vm.render(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 540 ));
    } 

}


