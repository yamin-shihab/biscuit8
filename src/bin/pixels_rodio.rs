use biscuit8::{chip8::Chip8, drivers::Drivers};
use std::{env, fs};

// Drivers that use pixels + winit for graphics + input and rodio for audio
struct PixelsRodio {}

impl PixelsRodio {
    // Creates a new set of drivers for pixles + winit graphics and rodio audio
    pub fn new() -> Self {
        Self {}
    }
}

impl Drivers for PixelsRodio {}

// Gets the ROM from the given path, creates an emulator, and starts the main loop
fn main() {
    let chip8 = {
        let path = env::args().nth(1).expect("Please give a path to the ROM");
        let rom = fs::read(path).expect("Failed to load file");
        Chip8::new(&rom, PixelsRodio::new()).expect("Failed to create emulator instance")
    };
    instruction_cycle(chip8);
}

// The fetch-decode-execute cycle
fn instruction_cycle(mut chip8: Chip8<PixelsRodio>) {
    while chip8.cycle() {}
    println!("ROM ended")
}
