//! Provides intermediary constructs for output between a frontend and the
//! backend.

/// The default width of the emulator's screen.
pub const WIDTH: usize = 64;

/// The default height of the emulator's screen.
pub const HEIGHT: usize = 32;

/// Represents the screen of the emulator.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Screen {
    raw: [bool; WIDTH * HEIGHT],
}

impl Screen {
    /// Initializes a new screen.
    pub fn new() -> Self {
        Self {
            raw: [false; WIDTH * HEIGHT],
        }
    }

    /// Draws the given sprite at the specified location. Returns true if a pixel is
    /// erased.
    pub fn draw_sprite(&mut self, sprite: &[u8], mut x: usize, mut y: usize) -> bool {
        x %= WIDTH;
        y %= HEIGHT;
        let mut erased = false;
        for (i, row) in sprite.iter().enumerate() {
            if y + i >= HEIGHT {
                break;
            }
            for j in 0..8 {
                if x + j >= WIDTH {
                    break;
                }
                let bit = (row & 0b10000000 >> j) << j;
                let pos = (y + i) * WIDTH + x + j;
                let pixel = self.raw[pos];
                self.raw[pos] ^= bit != 0;
                if pixel && !self.raw[pos] {
                    erased = true;
                }
            }
        }
        erased
    }

    /// Clears the screen.
    pub fn clear(&mut self) {
        self.raw.fill(false)
    }

    /// Returns true if the provided position has a pixel, and false otherwise.
    pub fn pixel(&self, x: usize, y: usize) -> bool {
        self.raw[y * WIDTH + x]
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
