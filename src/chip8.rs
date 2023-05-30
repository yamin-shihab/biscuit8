use crate::{drivers::Drivers, instruction::Instruction};

// How many bytes to give to RAM
const RAM_SIZE: usize = 0x1000;

// Where to put the ROM in the RAM
const ROM_LOC: usize = 0x200;

// The sprites for all hexadecimal digits (stored at beginning of RAM)
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

// Struct used to represent the emulator
pub struct Chip8<T: Drivers> {
    ram: [u8; RAM_SIZE],
    v: [u8; 0x10],
    i: usize,
    pc: usize,
    dt: u8,
    st: u8,
    stack: Vec<usize>,
    drivers: T,
}

impl<T: Drivers> Chip8<T> {
    // Create an emulator from the given rom and set of drivers
    pub fn new(rom: &[u8], drivers: T) -> Result<Self, &str> {
        if rom.len() > RAM_SIZE - ROM_LOC {
            return Err("ROM size too big to fit into RAM");
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
            drivers,
        })
    }

    // Performs one iteration of the fetch-decode-execute cycle, and returns whether the instruction was executed or not, in the case of it being unknown or the last one
    pub fn cycle(&mut self) -> bool {
        self.fetch_instruction().map_or(false, |instruction| {
            self.pc += 2;
            self.decode_execute(instruction);
            true
        })
    }

    // Fetches the current instruction from the program counter if there is still one
    fn fetch_instruction(&self) -> Option<Instruction> {
        let first = self.ram.get(self.pc)?;
        let second = self.ram.get(self.pc + 1)?;
        Some(Instruction::new(u16::from_be_bytes([*first, *second])))
    }

    // Decodes and executes
    fn decode_execute(&mut self, instruction: Instruction) {
        match instruction.nibbles {
            _ => (),
        }
    }
}
