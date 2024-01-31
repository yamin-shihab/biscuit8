//! Provides intermediary constructs for input between a frontend and the
//! backend.

/// This represents any keys for input currently held down or released.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Keys {
    raw: u16,
    last_pressed: Option<u8>,
}

impl Keys {
    /// Constructs a new set of keys.
    pub const fn new() -> Self {
        Self { raw: 0, last_pressed: None }
    }

    /// Presses the specified key.
    pub fn press_key(&mut self, key: u8) {
        self.raw |= 1 << key;
        self.last_pressed = Some(key);
    }

    /// Releases the specified key.
    pub fn release_key(&mut self, key: u8) {
        self.raw &= !(1 << key);
    }

    /// Resets the last pressed key. This should be done at every tick.
    pub fn reset_last_pressed(&mut self) {
        self.last_pressed = None;
    }

    /// Returns whether the specified key is currently being pressed or not.
    pub const fn key_pressed(&self, key: u8) -> bool {
        (self.raw & (1 << key)) != 0
    }

    /// Returns the last key that was pressed.
    pub const fn last_pressed(&self) -> Option<u8> {
        self.last_pressed
    }
}
