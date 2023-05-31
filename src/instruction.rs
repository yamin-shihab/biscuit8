// Used to represent an instruction (opcode and values)
pub struct Instruction {
    pub nibbles: (u8, u8, u8, u8),
    pub raw_instruction: u16,
}

impl Instruction {
    // Create an instruction from the given 16 bits
    pub fn new(raw_instruction: u16) -> Self {
        Self {
            nibbles: (
                ((raw_instruction & 0xF000) >> 12) as u8,
                ((raw_instruction & 0x0F00) >> 8) as u8,
                ((raw_instruction & 0x00F0) >> 4) as u8,
                (raw_instruction & 0x000F) as u8,
            ),
            raw_instruction,
        }
    }

    // Returns the second 4 bits (a register)
    pub fn x(&self) -> usize {
        self.nibbles.1 as usize
    }

    // Returns the third 4 bits (a register)
    pub fn y(&self) -> usize {
        self.nibbles.2 as usize
    }

    // Returns the last 4 bits (a size)
    pub fn n(&self) -> usize {
        self.nibbles.3 as usize
    }

    // Returns the last 8 bits (a constant)
    pub fn nn(&self) -> u8 {
        self.raw_instruction.to_be_bytes()[1]
    }

    // Returns the last 12 bits (a memory address)
    pub fn nnn(&self) -> usize {
        (self.raw_instruction & 0x0FFF) as usize
    }
}
