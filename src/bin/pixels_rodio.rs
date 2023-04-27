use biscuit8::{chip8::Chip8, drivers::Drivers};
use std::{env, fs};

struct PixelsRodio {}

impl PixelsRodio {
    fn new() -> Self {
        Self {}
    }
}

impl Drivers for PixelsRodio {}

fn main() {
    let path = env::args().nth(1).expect("Please give a path to the ROM");
    let rom = fs::read(path).expect("Failed to load file");
    let drivers = Box::new(PixelsRodio::new());
    let chip8 = Chip8::new(&rom, drivers).expect("Failed to create emulator instance");
}
