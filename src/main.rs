//! This program/binary takes in some CLI arguments, parses them, and runs the emulator with some
//! appropriate settings.

use biscuit8::{
    chip8::Chip8,
    cli::Args,
    drv::{pixels::PixelsDrv, Drv},
};
use std::fs;

/// Gets the ROM from the given path, creates an emulator using the chosen frontend, and starts the main instruction loop with some options/settings.
fn main() {
    let args = argh::from_env::<Args>();

    let chip8 = {
        let rom = fs::read(&args.path).expect("Failed to load file.");
        Chip8::new(&rom).expect("Failed to create emulator instance.")
    };

    match args.frontend.as_ref() {
        "pixels" => PixelsDrv::new(chip8).instruction_loop(),
        _ => (),
    }
}
