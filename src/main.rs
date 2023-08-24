use biscuit8::{
    chip8::Chip8,
    drv::{pixels::PixelsDrv, Drv},
};
use std::{env, fs};

// Gets the ROM from the given path, creates an emulator, and starts the main instruction loop
fn main() {
    let chip8 = {
        let path = env::args()
            .nth(1)
            .expect("Please provide a path to the ROM");
        let rom = fs::read(path).expect("Failed to load file");
        Chip8::new(&rom).expect("Failed to create emulator instance")
    };
    PixelsDrv::new(chip8).instruction_loop();
}
