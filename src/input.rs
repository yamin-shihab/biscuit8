//! This module provides intermediary constructs for input between a frontend and the backend.

use std::{
    fmt::{Display, Error, Formatter},
    str::FromStr,
};
use thiserror::Error;

/// The supported keyboard layouts.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Layout {
    #[default]
    Qwerty,
    Colemak,
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Qwerty => write!(f, "QWERTY"),
            Self::Colemak => write!(f, "Colemak"),
        }
    }
}

impl FromStr for Layout {
    type Err = ParseLayoutError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "qwerty" => Ok(Layout::Qwerty),
            "colemak" => Ok(Layout::Colemak),
            _ => Err(ParseLayoutError),
        }
    }
}

/// Used when a given keyboard layout is unknown.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[error("Keyboard layout is unknown (QWERTY and Colemak supported).")]
pub struct ParseLayoutError;

/// This represents any keys for input currently held down or released.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Keys {
    raw: u16,
}

impl Keys {
    /// Constructs a new set of keys.
    pub fn new() -> Self {
        Self { raw: 0 }
    }

    /// Presses the specified key.
    pub fn press_key(&mut self, key: usize) {
        self.raw |= 1 << key
    }

    /// Releases the specified key.
    pub fn release_key(&mut self, key: usize) {
        self.raw &= !(1 << key)
    }

    /// Returns whether the specified key is currently being pressed or not.
    pub fn key_pressed(&self, key: usize) -> bool {
        (self.raw & (1 << key)) != 0
    }
}
