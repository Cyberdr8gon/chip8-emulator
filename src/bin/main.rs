extern crate chip8_emulator;

use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Error, no rom file specified.");
        return
    }

    let mut file = File::open(&args[1]).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    let rom = file_buffer.into_boxed_slice();

    
    //println!("{:?}", rom);

}
