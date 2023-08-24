//! BISCUIT-8 is a library for CHIP-8 emulation; it provides functionality for the logic of the
//! fetch-decode-execute loop itself and multiple frontends for things like graphics, input, audio,
//! and randomness.

pub mod chip8;
pub mod drv;
pub mod instruction;
