//! This module provides the logic of the emulator itself, primarily through the [`Chip8`] struct.
//! The error type [`Chip8Error`] is also provided.

use crate::instruction::Instruction;
use thiserror::Error;

/// How many bytes to allocate for the emulator's RAM.
const RAM_SIZE: usize = 0x1000;

/// Where to put the ROM in the emulator's RAM.
const ROM_LOC: usize = 0x200;

/// The sprites for every hexadecimal digit as a font (stored at the beginning of the emulator's
/// RAM).
const FONT_SPRITES: [u8; 0x50] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Used to represent the emulator.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    v: [u8; 0x10],
    i: usize,
    pc: usize,
    dt: u8,
    st: u8,
    stack: Vec<usize>,
    instruction: Instruction,
}

impl Chip8 {
    /// Create an emulator from the given ROM.
    pub fn new(rom: &[u8]) -> Result<Self, Chip8Error> {
        if rom.len() > RAM_SIZE - ROM_LOC {
            return Err(Chip8Error::RomTooBig);
        }

        let mut ram = [0; RAM_SIZE];
        ram[..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);
        ram[ROM_LOC..rom.len() + ROM_LOC].copy_from_slice(rom);

        Ok(Self {
            ram,
            v: [0; 0x10],
            i: 0,
            pc: ROM_LOC,
            dt: 0,
            st: 0,
            stack: Vec::new(),
            instruction: Instruction::new(0),
        })
    }

    /// Performs one iteration of the fetch-decode-execute cycle and returns an error if there
    /// isn't another [`Instruction`] to be decoded and executed or the opcode of the current
    /// [`Instruction`] is unknown.
    pub fn instruction_cycle(&mut self) -> Result<(), Chip8Error> {
        let Some(instruction) = self.fetch_instruction() else {
            return Err(Chip8Error::NoMoreInstructions);
        };
        self.instruction = instruction;
        self.pc += 2;
        self.decode_execute()?;
        Ok(())
    }

    /// Fetches the current [`Instruction`] from the program counter (if there still is one).
    fn fetch_instruction(&self) -> Option<Instruction> {
        let first = self.ram.get(self.pc)?;
        let second = self.ram.get(self.pc + 1)?;
        Some(Instruction::new(u16::from_be_bytes([*first, *second])))
    }

    /// Decodes the current [`Instruction`] and executes the appropriate method. An error is
    /// returned when the instruction opcode is unknown.
    fn decode_execute(&mut self) -> Result<(), Chip8Error> {
        match self.instruction.nibbles() {
            (0, 0, 0, 0) => (),
            (0x0, 0x0, 0xE, 0x0) => self.clr_screen(),
            (0x0, 0x0, 0xE, 0xE) => self.ret_subroutine(),
            (0x1, _, _, _) => self.jp_addr(),
            (0x2, _, _, _) => self.call_subroutine(),
            (0x3, _, _, _) => self.skip_eq_byte(),
            (0x4, _, _, _) => self.skip_not_byte(),
            (0x5, _, _, 0x0) => self.skip_eq_reg(),
            (0x6, _, _, _) => self.set_reg_byte(),
            (0x7, _, _, _) => self.add_byte(),
            (0x8, _, _, 0x0) => self.set_reg_reg(),
            (0x8, _, _, 0x1) => self.or_reg(),
            (0x8, _, _, 0x2) => self.and_reg(),
            (0x8, _, _, 0x3) => self.xor_reg(),
            (0x8, _, _, 0x4) => self.add_reg(),
            (0x8, _, _, 0x5) => self.sub_reg(),
            (0x8, _, _, 0x6) => self.right_shift(),
            (0x8, _, _, 0x7) => self.rev_sub_reg(),
            (0x8, _, _, 0xE) => self.left_shift(),
            (0x9, _, _, 0x0) => self.skip_not_reg(),
            (0xA, _, _, _) => self.set_idx_addr(),
            (0xB, _, _, _) => self.jp_add_addr(),
            (0xC, _, _, _) => self.rand_and_byte(),
            (0xD, _, _, _) => self.drw_sprite(),
            (0xE, _, 0x9, 0xE) => self.skip_eq_key(),
            (0xE, _, 0xA, 0x1) => self.skip_not_key(),
            (0xF, _, 0x0, 0x7) => self.set_reg_delay(),
            (0xF, _, 0x0, 0xA) => self.set_reg_key(),
            (0xF, _, 0x1, 0x5) => self.set_delay_reg(),
            (0xF, _, 0x1, 0x8) => self.set_sound_reg(),
            (0xF, _, 0x1, 0xE) => self.add_idx_reg(),
            (0xF, _, 0x2, 0x9) => self.set_idx_char(),
            (0xF, _, 0x3, 0x3) => self.set_idx_bcd(),
            (0xF, _, 0x5, 0x5) => self.set_idx_reg(),
            (0xF, _, 0x6, 0x5) => self.set_reg_idx(),
            _ => return Err(Chip8Error::UnknownInstruction(self.instruction)),
        }
        Ok(())
    }

    /// Clears the screen.
    fn clr_screen(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Returns from the current subroutine using the stack.
    fn ret_subroutine(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Jumps to the given address.
    fn jp_addr(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Calls a subroutine using the stack.
    fn call_subroutine(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the register is equal to the byte.
    fn skip_eq_byte(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the register isn't equal to the byte.
    fn skip_not_byte(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the register is equal to the register.
    fn skip_eq_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the register to the byte
    fn set_reg_byte(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Adds the byte to the register.
    fn add_byte(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the register to the register.
    fn set_reg_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Applies a bitwise OR operation onto the register with the register.
    fn or_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Applies a bitwise AND operation onto the register with the register.
    fn and_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Applies a bitwise XOR operation onto the register with the register.
    fn xor_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Adds the register to the register and sets the flag register in the case of a carry.
    fn add_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Subtracts the register from the register and sets the flag register in the case of a borrow.
    fn sub_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the flag register to the least significant bit and right shifts the register by one.
    fn right_shift(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the register to the register minus it and sets the flag register in the case of a
    /// borrow.
    fn rev_sub_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the flag register to the most significant bit and left shifts the register by one.
    fn left_shift(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the register isn't equal to the register by incrementing the
    /// pogram counter.
    fn skip_not_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the index register to the address.
    fn set_idx_addr(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the program counter to the address plus the first register.
    fn jp_add_addr(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the register to the result of a bitwise AND operation on a random number and the byte.
    fn rand_and_byte(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Draws the sprite located in the index register onto the screen, and the flag register is set
    /// if a pixel collision occurs; the location of the sprite is represented using the registers,
    /// and height is defined by the nibble.
    fn drw_sprite(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the key represented in the register is pressed.
    fn skip_eq_key(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Skips the next instruction if the key represented in the register isn't pressed.
    fn skip_not_key(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the register to the delay timer.
    fn set_reg_delay(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Waits until a key is pressed and released before setting the register to it.
    fn set_reg_key(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the delay timer to the register.
    fn set_delay_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the sound timer to the register.
    fn set_sound_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Adds the register to the index register.
    fn add_idx_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the index register to the font character represented by the register.
    fn set_idx_char(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the location in RAM represented by the index register to the binary-coded decimal
    /// representation of the register (hundreds, tens, and ones all in decimal).
    fn set_idx_bcd(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the location in RAM represented by the index register to the range of registers from
    /// the first to the register.
    fn set_idx_reg(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }

    /// Sets the range of registers from the first to the register to the location in RAM
    /// represented by the index register.
    fn set_reg_idx(&mut self) {
        todo!(
            "Still have to implement the {} instruction.",
            self.instruction
        );
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(&[]).expect("Empty ROM should've fit in the emulator's RAM.")
    }
}

/// Used to describe possibble errors caused by the emulator
#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum Chip8Error {
    #[error("ROM size exceeds the amount of RAM provided by the CHIP-8 emulator.")]
    RomTooBig,
    #[error("Instruction opcode is unknown.")]
    UnknownInstruction(Instruction),
    #[error("There aren't any more instructions to run.")]
    NoMoreInstructions,
}
