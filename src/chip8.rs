use crate::instruction::Instruction;

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
    // Create an emulator from the given rom
    pub fn new(rom: &[u8]) -> Result<Self, &str> {
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
            instruction: Instruction::new(0),
        })
    }

    // Performs one iteration of the fetch-decode-execute cycle, and returns whether the instruction was executed or not, in the case of it being the last one
    pub fn cycle(&mut self) -> bool {
        self.fetch_instruction().map_or(false, |instruction| {
            self.instruction = instruction;
            self.pc += 2;
            self.decode_execute();
            true
        })
    }

    // Fetches the current instruction from the program counter if there is still one
    fn fetch_instruction(&self) -> Option<Instruction> {
        let first = self.ram.get(self.pc)?;
        let second = self.ram.get(self.pc + 1)?;
        Some(Instruction::new(u16::from_be_bytes([*first, *second])))
    }

    // Decodes the current instruction and executes the appropriate method
    fn decode_execute(&mut self) {
        match self.instruction.nibbles {
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
            _ => eprintln!("Unknown opcode: {:#X}", self.instruction.raw_instruction),
        }
    }

    // Clears the screen
    fn clr_screen(&mut self) {
        todo!();
    }

    // Returns from the current subroutine using the stack
    fn ret_subroutine(&mut self) {
        todo!();
    }

    // Jumps to the given address
    fn jp_addr(&mut self) {
        todo!();
    }

    // Calls a subroutine using the stack
    fn call_subroutine(&mut self) {
        todo!();
    }

    // Skips the next instruction if register is equal to byte
    fn skip_eq_byte(&mut self) {
        todo!();
    }

    // Skips the next instruction if register isn't equal to byte
    fn skip_not_byte(&mut self) {
        todo!();
    }

    // Skips the next instruction if register is equal to register
    fn skip_eq_reg(&mut self) {
        todo!();
    }

    // Sets register to byte
    fn set_reg_byte(&mut self) {
        todo!();
    }

    // Adds byte to register
    fn add_byte(&mut self) {
        todo!();
    }

    // Sets register to register
    fn set_reg_reg(&mut self) {
        todo!();
    }

    // Applies bitwise OR on register with register
    fn or_reg(&mut self) {
        todo!();
    }

    // Applies bitwise AND on register with register
    fn and_reg(&mut self) {
        todo!();
    }

    // Applies bitwise XOR on register with register
    fn xor_reg(&mut self) {
        todo!();
    }

    // Adds register to register, and sets the flag register in the case of a carry
    fn add_reg(&mut self) {
        todo!();
    }

    // Subtracts register from register, and sets the flag register in the case of a borrow
    fn sub_reg(&mut self) {
        todo!();
    }

    // Sets the flag register to the least significant bit and right shifts register by one
    fn right_shift(&mut self) {
        todo!();
    }

    // Sets register to register minus it, and sets the flag register in the case of a borrow
    fn rev_sub_reg(&mut self) {
        todo!();
    }

    // Sets the flag register to the most significant bit and left shifts register by one
    fn left_shift(&mut self) {
        todo!();
    }

    // Skips the next instruction if register isn't equal to register by incrementing the pogram counter
    fn skip_not_reg(&mut self) {
        todo!();
    }

    // Sets the index register to address
    fn set_idx_addr(&mut self) {
        todo!();
    }

    // Sets the program counter to address plus the first register
    fn jp_add_addr(&mut self) {
        todo!();
    }

    // Sets register to the result of bitwise AND on random number and byte
    fn rand_and_byte(&mut self) {
        todo!();
    }

    // Draws sprite located in index register onto screen, and the flag register is set if a pixel collision occurs; the location of the sprite is represented using registers, and height is given by nibble
    fn drw_sprite(&mut self) {
        todo!();
    }

    // Skips the next instruction if the key represented in register is pressed
    fn skip_eq_key(&mut self) {
        todo!();
    }

    // Skips the next instruction if the key represented in register isn't pressed
    fn skip_not_key(&mut self) {
        todo!();
    }

    // Sets register to the delay timer
    fn set_reg_delay(&mut self) {
        todo!();
    }

    // Waits until a key is pressed and released before setting register to it
    fn set_reg_key(&mut self) {
        todo!();
    }

    // Sets the delay timer to register
    fn set_delay_reg(&mut self) {
        todo!();
    }

    // Sets the sound timer to register
    fn set_sound_reg(&mut self) {
        todo!();
    }

    // Adds register to the index register
    fn add_idx_reg(&mut self) {
        todo!();
    }

    // Sets the index register to the font character represented by register
    fn set_idx_char(&mut self) {
        todo!();
    }

    // Sets the location in RAM represented by the index register to the binary-coded decimal representation of register (hundreds, tens, and ones all in decimal)
    fn set_idx_bcd(&mut self) {
        todo!();
    }

    // Sets the location in RAM represented by the index register to the range of registers from the first to register
    fn set_idx_reg(&mut self) {
        todo!();
    }

    // Sets the range of registers from the first to register to the location in RAM represented by the index register
    fn set_reg_idx(&mut self) {
        todo!();
    }
}
