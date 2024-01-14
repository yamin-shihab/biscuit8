//! CLI argument parsing is done here; you can use [`argh`] to get a struct containing things like
//! the path to the ROM and other options/settings.

use crate::{
    chip8::{Chip8, Chip8Error},
    input::Layout,
};
use argh::FromArgs;
use std::{fs, io::Error, path::PathBuf};
use thiserror::Error;

/// A CHIP-8 emulator with support for multiple frontends and options.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromArgs)]
pub struct Args {
    /// the keyboard layout to use (QWERTY and Colemak supported)
    #[argh(option, short = 'l', default = "Layout::default()")]
    pub layout: Layout,
    /// the background color in 0xRRGGBB hex
    #[argh(option, default = "0x000000")]
    pub bg: u32,
    /// the foreground color in 0xRRGGBB hex
    #[argh(option, default = "0xFFFFFF")]
    pub fg: u32,
    /// path of the ROM to execute
    #[argh(positional)]
    pub path: PathBuf,
}

impl Args {
    /// Attempts to return a constructed emulator using the provided arguments.
    pub fn chip8(&self) -> Result<Chip8, ArgsError> {
        let rom = fs::read(&self.path)?;
        Ok(Chip8::new(&rom)?)
    }
}

/// Error type for different ways emulator creation could fail.
#[derive(Debug, Error)]
pub enum ArgsError {
    #[error("{0}.")]
    Io(#[from] Error),
    #[error("{0}")]
    Chip8(#[from] Chip8Error),
}
