//! Provides intermediary constructs for input between a frontend and the
//! backend.

/// This represents any keys for input currently held down or released.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Keys {
    raw: u16,
}

impl Keys {
    /// Constructs a new set of keys.
    pub const fn new() -> Self {
        Self { raw: 0 }
    }

    /// Presses the specified key.
    pub fn press_key(&mut self, key: usize) {
        self.raw |= 1 << key
    }

    /// Releases the specified key.
    pub fn release_key(&mut self, key: usize) {
        self.raw &= !(1 << key)
    }

    /// Returns whether the specified key is currently being pressed or not.
    pub const fn key_pressed(&self, key: usize) -> bool {
        (self.raw & (1 << key)) != 0
    }
}
