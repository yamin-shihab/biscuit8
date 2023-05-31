use biscuit8::{chip8::Chip8, drv::Drv};
use std::{env, fs};

// Drivers that use pixels + winit for graphics + input, rodio for audio, and fastrand for randomness
struct PixelsRodioFastrand {}

impl PixelsRodioFastrand {
    // Creates a new set of drivers for pixles + winit graphics, rodio audio, and fastrand randomness
    pub fn new() -> Self {
        Self {}
    }
}

impl Drv for PixelsRodioFastrand {}

// Gets the ROM from the given path, creates an emulator, and starts the main loop
fn main() {
    let chip8 = {
        let path = env::args().nth(1).expect("Please give a path to the ROM");
        let rom = fs::read(path).expect("Failed to load file");
        Chip8::new(&rom, PixelsRodioFastrand::new()).expect("Failed to create emulator instance")
    };
    instruction_cycle(chip8);
}

// The fetch-decode-execute cycle
fn instruction_cycle(mut chip8: Chip8<PixelsRodioFastrand>) {
    while chip8.cycle() {}
    println!("Done")
}
