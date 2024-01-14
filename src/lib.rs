//! `biscuit8` is a modular CHIP-8 emulator library written in Rust with
//! multiple supported and implemented frontends included. The `biscuit8`
//! library crate provides a backend: the logic, processing, and instruction
//! loop of a CHIP-8 emulator. Things like graphics, input, and audio are
//! required to be implemented by the frontend, but numerous helper constructs
//! are provided to assist with bridging the gap. This project also implements
//! some frontends itself too.

pub mod chip8;
pub mod cli;
pub mod input;
pub mod instruction;
pub mod output;
