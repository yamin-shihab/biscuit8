//! Every frontend for this emulator using the library (backend) should implement the [`Drv`] trait
//! for a common and consistent interface. There are also some frontends each in a submodule here.

use crate::chip8::Chip8;

pub mod pixels;

/// Used to implement a frontend by providing an appropriate creation and instruction loop.
pub trait Drv {
    /// Creates a new set of drivers for the specific frontend; an instance of the emulator backend
    /// is given to be used.
    fn new(chip8: Chip8) -> Self;

    /// Loops through the emulator's instructions and performs any input or output actions
    /// accordingly.
    fn instruction_loop(self);
}
