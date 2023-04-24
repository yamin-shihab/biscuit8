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
    vx: [u8; 0x10],
    i: usize,
    pc: usize,
    stack: Vec<usize>,
    dt: u8,
    st: u8,
    keys: u16,
    display: [u64; 32],
}

impl Chip8 {
    // Create a new instance off an emulator
    pub fn new(rom: &[u8]) -> Result<Self, &str> {
        if rom.len() > RAM_SIZE - ROM_LOC {
            return Err("ROM size too big to fit into RAM");
        }

        let mut ram = [0; RAM_SIZE];
        ram[..FONT_SPRITES.len()].copy_from_slice(&FONT_SPRITES);
        ram[ROM_LOC..rom.len() + ROM_LOC].copy_from_slice(rom);

        Ok(Self {
            ram,
            vx: [0; 0x10],
            i: 0,
            pc: ROM_LOC,
            stack: Vec::new(),
            dt: 0,
            st: 0,
            keys: 0,
            display: [0; 32],
        })
    }
}
