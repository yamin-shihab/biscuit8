//! Provides a way for CHIP-8's 16-bit instructions to be represented (the [`Instruction`] struct).

use std::fmt::{Display, Formatter, Result};

/// Used to represent an instruction (opcode and values).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instruction {
    raw: u16,
}

impl Instruction {
    /// Create an instruction from the given 16 bits.
    pub fn new(raw: u16) -> Self {
        Self { raw }
    }

    /// Returns all four nibbles (4 bits each) of the instruction.
    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        (
            ((self.raw & 0xF000) >> 12) as u8,
            ((self.raw & 0x0F00) >> 8) as u8,
            ((self.raw & 0x00F0) >> 4) as u8,
            (self.raw & 0x000F) as u8,
        )
    }

    /// Returns the second 4 bits (a register).
    pub fn x(&self) -> usize {
        ((self.raw & 0x0F00) >> 8) as usize
    }

    /// Returns the third 4 bits (a register).
    pub fn y(&self) -> usize {
        ((self.raw & 0x00F0) >> 4) as usize
    }

    /// Returns the last 4 bits (a size).
    pub fn n(&self) -> usize {
        (self.raw & 0x000F) as usize
    }

    /// Returns the last 8 bits (a constant).
    pub fn nn(&self) -> u8 {
        self.raw.to_be_bytes()[1]
    }

    /// Returns the last 12 bits (a memory address).
    pub fn nnn(&self) -> usize {
        (self.raw & 0x0FFF) as usize
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:#06X}", self.raw)
    }
}
