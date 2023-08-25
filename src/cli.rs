//! This lets you use `argh` to parse given CLI arguments and return a struct containing things like
//! the chosen frontend, path to the ROM, and other options/settings.

use argh::FromArgs;
use std::path::PathBuf;

/// A CHIP-8 emulator with support for multiple frontends and options.
#[derive(Clone, Debug, FromArgs)]
pub struct Args {
    /// which frontend to use
    #[argh(option, short = 'f')]
    pub frontend: String,
    /// path of the ROM to execute
    #[argh(positional)]
    pub path: PathBuf,
}
