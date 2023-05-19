// Used to represent an instruction (opcode and values)
pub struct Instruction {
    nibbles: (u8, u8, u8, u8),
    x: usize,
    y: usize,
    n: usize,
    nn: u8,
    nnn: usize,
}

impl Instruction {
    // Create an instruction from the given two bytes (16 bits)
    pub fn new(raw_instruction: u16) -> Self {
        let nibbles = (
            ((raw_instruction & 0xF000) >> 12) as u8,
            ((raw_instruction & 0x0F00) >> 8) as u8,
            ((raw_instruction & 0x00F0) >> 4) as u8,
            (raw_instruction & 0x000F) as u8,
        );

        Self {
            nibbles,
            x: nibbles.1 as usize,
            y: nibbles.2 as usize,
            n: nibbles.3 as usize,
            nn: raw_instruction.to_be_bytes()[1],
            nnn: (raw_instruction & 0x0FFF) as usize,
        }
    }

    // Matches against the nibbles of the instruction and executes the appropriate function
    pub fn execute(&self) {
        match self.nibbles {
            _ => (),
        }
    }
}
