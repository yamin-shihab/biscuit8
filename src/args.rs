//! CLI argument parsing is done here; you can use [`argh`] to get a struct
//! containing things like the path to the ROM and other options/settings.

use crate::chip8::{Chip8, Chip8Error};
use argh::FromArgs;
use std::{
    fmt::{Display, Error as FmtError, Formatter},
    fs,
    io::Error as IoError,
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};
use thiserror::Error;

/// A CHIP-8 emulator with support for multiple frontends and options.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromArgs)]
pub struct Args {
    /// the keyboard layout to use (QWERTY and Colemak supported)
    #[argh(option, short = 'l', default = "Layout::default()")]
    pub layout: Layout,
    /// the background color in #RRGGBB hex
    #[argh(option, default = "\"#000000\".to_string()")]
    pub bg: String,
    /// the foreground color in #RRGGBB hex
    #[argh(option, default = "\"#FFFFFF\".to_string()")]
    pub fg: String,
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
    #[error("Layout doesn't exist.")]
    Layout,
    #[error("Hexadecimal RGB color format is incorrect.")]
    HexRgb,
    #[error("{0}.")]
    Io(#[from] IoError),
    #[error("{0}")]
    Chip8(#[from] Chip8Error),
}

/// The supported keyboard layouts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Layout {
    #[default]
    Qwerty,
    Colemak,
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Qwerty => write!(f, "QWERTY"),
            Self::Colemak => write!(f, "Colemak"),
        }
    }
}

impl FromStr for Layout {
    type Err = ArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "qwerty" => Ok(Layout::Qwerty),
            "colemak" => Ok(Layout::Colemak),
            _ => Err(ArgsError::Layout),
        }
    }
}

/// Converts a given hexadecimal color to a 24-bit RGB color.
pub fn hex_to_rgb(color: String) -> Result<[u8; 3], ArgsError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ArgsError::HexRgb);
    }
    let color = (1..color.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&color[i..i + 2], 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .map_err(|_| ArgsError::HexRgb)?;
    Ok([color[0], color[1], color[2]])
}
