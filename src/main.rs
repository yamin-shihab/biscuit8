use biscuit8::Chip8;
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Please give a path to the ROM");
    let rom = fs::read(path).expect("Failed to load file");
    let chip8 = Chip8::new(&rom).expect("Failed to create emulator instance");
}
