//! A [`biscuit8`] frontend using [`pixels`] for rendering, [`winit`] for window managemenet and
//! input, and [`rodio`] for audio, primarily provided by [`PixelsFrontend`]. Errors are also
//! represented by [`PixelsError`].

use biscuit8::{
    chip8::{Chip8, Chip8Error},
    cli::Args,
    input::{Keys, Layout},
};
use pixels::{Error, Pixels, SurfaceTexture, TextureError};
use std::process;
use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// The width of the screen.
const WIDTH: u32 = 64;

/// The height of the screen.
const HEIGHT: u32 = 32;

/// A frontend that uses [`pixels`] for rendering, [`winit`] for window managemenet and input, and
/// [`rodio`] for audio.
#[derive(Debug)]
pub struct PixelsFrontend {
    chip8: Chip8,
    layout: Layout,
    keys: Keys,
    event_loop: Option<EventLoop<()>>,
    window: Window,
    pixels: Pixels,
}

impl PixelsFrontend {
    /// Constructs a new [`pixels`] frontend using the provided emulator instance and ROM name.
    fn new(chip8: Chip8, rom: &str, layout: Layout) -> Result<Self, PixelsError> {
        let event_loop = EventLoop::new();
        let window = {
            let size = PhysicalSize::new(WIDTH, HEIGHT);
            WindowBuilder::new()
                .with_title(format!("{} - biscuit8-pixels", rom))
                .with_min_inner_size(size)
                .build(&event_loop)?
        };
        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        Ok(Self {
            chip8,
            layout,
            keys: Keys::new(),
            event_loop: Some(event_loop),
            window,
            pixels,
        })
    }

    /// The main loop managed by [`winit`]; almost everything happens here.
    fn main_loop(mut self) {
        // The event loop is an option so that methods can be called from within the move closure.
        let event_loop = self
            .event_loop
            .take()
            .expect("Event loop should've been initialized.");

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            if let Err(err) = self.event_handler(event, control_flow) {
                eprintln!("Error while running pixels frontend: {}", err);
                control_flow.set_exit_with_code(1);
            }
        });
    }

    /// Handles [`winit`] events (window management, logic, rendering).
    fn event_handler(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) -> Result<(), PixelsError> {
        match event {
            Event::WindowEvent { event, .. } => self.window_event_handler(event, control_flow)?,
            Event::MainEventsCleared => self.instruction_cycle(control_flow),
            Event::RedrawRequested(_) => self.pixels.render()?,
            _ => (),
        }
        Ok(())
    }

    /// Handles [`winit`] window events (scale, window state, input).
    fn window_event_handler(
        &mut self,
        event: WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> Result<(), PixelsError> {
        match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::Resized(size) => self.pixels.resize_surface(size.width, size.height)?,
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self
                .pixels
                .resize_surface(new_inner_size.width, new_inner_size.height)?,
            WindowEvent::KeyboardInput { input, .. } => self.input_handler(input),
            _ => (),
        }
        Ok(())
    }

    /// Handles keyboard input.
    fn input_handler(&mut self, input: KeyboardInput) {
        let Some(keycode) = input.virtual_keycode else {
            return;
        };
        let Some(key) = (match self.layout {
            Layout::Qwerty => Self::qwerty_keycode_to_key(keycode),
            Layout::Colemak => Self::colemak_keycode_to_key(keycode),
        }) else {
            return;
        };
        match input.state {
            ElementState::Pressed => self.keys.press_key(key),
            ElementState::Released => self.keys.release_key(key),
        }
    }

    /// Converts [`winit`]'s [`VirtualKeyCode`] to a numeric CHIP-8 key using QWERTY.
    fn qwerty_keycode_to_key(keycode: VirtualKeyCode) -> Option<usize> {
        Some(match keycode {
            VirtualKeyCode::Key1 => 0x1,
            VirtualKeyCode::Key2 => 0x2,
            VirtualKeyCode::Key3 => 0x3,
            VirtualKeyCode::Key4 => 0xC,
            VirtualKeyCode::Q => 0x4,
            VirtualKeyCode::W => 0x5,
            VirtualKeyCode::E => 0x6,
            VirtualKeyCode::R => 0xD,
            VirtualKeyCode::A => 0x7,
            VirtualKeyCode::S => 0x8,
            VirtualKeyCode::D => 0x9,
            VirtualKeyCode::F => 0xE,
            VirtualKeyCode::Z => 0xA,
            VirtualKeyCode::X => 0x0,
            VirtualKeyCode::C => 0xB,
            VirtualKeyCode::V => 0xF,
            _ => return None,
        })
    }

    /// Converts [`winit`]'s [`VirtualKeyCode`] to a numeric CHIP-8 key using Colemak.
    fn colemak_keycode_to_key(keycode: VirtualKeyCode) -> Option<usize> {
        Some(match keycode {
            VirtualKeyCode::Key1 => 0x1,
            VirtualKeyCode::Key2 => 0x2,
            VirtualKeyCode::Key3 => 0x3,
            VirtualKeyCode::Key4 => 0xC,
            VirtualKeyCode::Q => 0x4,
            VirtualKeyCode::W => 0x5,
            VirtualKeyCode::F => 0x6,
            VirtualKeyCode::P => 0xD,
            VirtualKeyCode::A => 0x7,
            VirtualKeyCode::R => 0x8,
            VirtualKeyCode::S => 0x9,
            VirtualKeyCode::T => 0xE,
            VirtualKeyCode::Z => 0xA,
            VirtualKeyCode::X => 0x0,
            VirtualKeyCode::C => 0xB,
            VirtualKeyCode::V => 0xF,
            _ => return None,
        })
    }

    /// Updates the emulator and gets the frontend to act accordingly.
    fn instruction_cycle(&mut self, control_flow: &mut ControlFlow) {
        let Err(err) = self.chip8.instruction_cycle() else {
            return;
        };
        match err {
            Chip8Error::NoMoreInstructions => {
                println!("Successfully finished executing ROM.");
                control_flow.set_exit();
            }
            Chip8Error::UnknownInstruction(instruction) => {
                eprintln!("Unknown instruction opcode: {}.", instruction);
            }
            _ => (),
        }
    }
}

/// The ways this frontend can cause an error and fail.
#[derive(Debug, Error)]
pub enum PixelsError {
    #[error("Pixels error: {0}")]
    Pixels(#[from] Error),
    #[error("Texture error: {0}.")]
    Texture(#[from] TextureError),
    #[error("Winit OS error: {0}.")]
    WinitOs(#[from] OsError),
}

/// Gets the ROM from the given path, creates an emulator using the chosen frontend, and starts the
/// main instruction loop with some options/settings.
fn main() {
    let args = argh::from_env::<Args>();
    let chip8 = args.chip8().unwrap_or_else(|err| {
        eprintln!(
            "Error while setting up CHIP-8 emulator with ROM file \"{}\": {}",
            args.path.to_string_lossy(),
            err
        );
        process::exit(1)
    });

    let frontend = PixelsFrontend::new(chip8, &args.path.to_string_lossy(), args.layout)
        .unwrap_or_else(|err| {
            eprintln!("Error while setting up pixels frontend: {}", err);
            process::exit(1)
        });
    frontend.main_loop();
}