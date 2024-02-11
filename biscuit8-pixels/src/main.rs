//! A [`biscuit8`] frontend using [`pixels`] for rendering, [`winit`] for window
//! management and input, and [`rodio`] for audio, primarily provided by
//! [`PixelsFrontend`]. Errors are also represented by [`PixelsFrontendError`].

use biscuit8::{
    args::{self, Args, ArgsError, Layout},
    chip8::{Chip8, Chip8Error},
    keys::Keys,
    screen::{self, Screen},
};
use pixels::{wgpu::Color, Error, Pixels, PixelsBuilder, SurfaceTexture, TextureError};
use rodio::{source::SineWave, OutputStream, PlayError, Sink, StreamError};
use std::process::ExitCode;
use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    error::{EventLoopError, OsError},
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    window::{Window, WindowBuilder},
};

/// A frontend that uses [`pixels`] for rendering, [`winit`] for window
/// managemenet and input, and [`rodio`] for audio.
pub struct PixelsFrontend {
    chip8: Chip8,
    keys: Keys,
    layout: Layout,
    bg: [u8; 3],
    fg: [u8; 3],
    event_loop: Option<EventLoop<()>>,
    window: Window,
    pixels: Pixels,
    sink: Sink,
    _stream: OutputStream,
}

impl PixelsFrontend {
    /// Constructs a new [`pixels`] frontend using the provided emulator instance,
    /// keyboard layout, colors, and ROM name.
    pub fn new(
        chip8: Chip8,
        layout: Layout,
        bg: [u8; 3],
        fg: [u8; 3],
        rom: &str,
    ) -> Result<Self, PixelsFrontendError> {
        let event_loop = EventLoop::new()?;
        let window = {
            let size = PhysicalSize::new(screen::WIDTH as u32, screen::HEIGHT as u32);
            WindowBuilder::new()
                .with_title(format!("{} - biscuit8-pixels", rom))
                .with_min_inner_size(size)
                .build(&event_loop)?
        };
        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            let clear_color = Color {
                r: (fg[0] as f64) / 255.0,
                g: (fg[1] as f64) / 255.0,
                b: (fg[2] as f64) / 255.0,
                a: 1.0,
            };
            PixelsBuilder::new(screen::WIDTH as u32, screen::HEIGHT as u32, surface_texture)
                .clear_color(clear_color)
                .build()?
        };
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let source = SineWave::new(700.0);
        sink.append(source);
        sink.pause();

        Ok(Self {
            chip8,
            fg,
            bg,
            layout,
            keys: Keys::new(),
            event_loop: Some(event_loop),
            window,
            pixels,
            sink,
            _stream,
        })
    }

    /// The main loop managed by [`winit`]; almost everything happens here.
    pub fn main_loop(mut self) -> Result<(), PixelsFrontendError> {
        let event_loop = self
            .event_loop
            .take()
            .expect("Event loop should've been initialized.");
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
            if let Err(err) = self.event_handler(event) {
                eprintln!("{}", err);
                elwt.exit();
            }
        })?;
        Ok(())
    }

    /// Handles [`winit`] events (window management, logic, rendering).
    fn event_handler(&mut self, event: Event<()>) -> Result<(), PixelsFrontendError> {
        match event {
            Event::WindowEvent { event, .. } => self.window_event_handler(event),
            Event::AboutToWait => self.instruction_cycle(),
            _ => Ok(()),
        }
    }

    /// Handles [`winit`] window events (scale, window state, input).
    fn window_event_handler(&mut self, event: WindowEvent) -> Result<(), PixelsFrontendError> {
        match event {
            WindowEvent::Resized(size) => self.pixels.resize_surface(size.width, size.height)?,
            WindowEvent::CloseRequested => return Err(PixelsFrontendError::WindowClose),
            WindowEvent::KeyboardInput { event, .. } => self.key_handler(event),
            WindowEvent::ScaleFactorChanged { .. } => {
                let size = self.window.inner_size();
                self.pixels.resize_surface(size.width, size.height)?
            }
            WindowEvent::RedrawRequested => self.pixels.render()?,
            _ => (),
        }
        Ok(())
    }

    /// Handles keyboard input.
    fn key_handler(&mut self, key_event: KeyEvent) {
        let Key::Character(character) = key_event.logical_key else {
            return;
        };
        let Some(key) = (match self.layout {
            Layout::Qwerty => Self::qwerty_character_to_key(&character),
            Layout::Colemak => Self::colemak_character_to_key(&character),
        }) else {
            return;
        };
        if key_event.state.is_pressed() {
            self.keys.press_key(key);
        } else {
            self.keys.release_key(key);
        }
    }

    /// Converts [`winit`]'s string character representation into a numeric
    /// CHIP-8 key using QWERTY.
    fn qwerty_character_to_key(character: &str) -> Option<u8> {
        Some(match character {
            "1" => 0x1,
            "2" => 0x2,
            "3" => 0x3,
            "4" => 0xC,
            "q" => 0x4,
            "w" => 0x5,
            "e" => 0x6,
            "r" => 0xD,
            "a" => 0x7,
            "s" => 0x8,
            "d" => 0x9,
            "f" => 0xE,
            "z" => 0xA,
            "x" => 0x0,
            "c" => 0xB,
            "v" => 0xF,
            _ => return None,
        })
    }

    /// Converts [`winit`]'s string character representation into a numeric
    /// CHIP-8 key using Colemak.
    fn colemak_character_to_key(character: &str) -> Option<u8> {
        Some(match character {
            "1" => 0x1,
            "2" => 0x2,
            "3" => 0x3,
            "4" => 0xC,
            "q" => 0x4,
            "w" => 0x5,
            "f" => 0x6,
            "p" => 0xD,
            "a" => 0x7,
            "r" => 0x8,
            "s" => 0x9,
            "t" => 0xE,
            "z" => 0xA,
            "x" => 0x0,
            "c" => 0xB,
            "v" => 0xF,
            _ => return None,
        })
    }

    /// Updates the emulator and gets the frontend to act accordingly.
    fn instruction_cycle(&mut self) -> Result<(), PixelsFrontendError> {
        let output = self.chip8.instruction_cycle(self.keys)?;
        if let Some(screen) = output.0 {
            self.draw_screen(screen);
        }
        self.beep(output.1);
        self.keys.reset_last_pressed();
        Ok(())
    }

    /// Draws the provided screen to the pixels buffer.
    fn draw_screen(&mut self, screen: Screen) {
        let frame = self.pixels.frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % screen::WIDTH;
            let y = i / screen::WIDTH;
            if screen.pixel(x, y) {
                pixel[0..3].copy_from_slice(&self.fg);
                pixel[3] = 255;
            } else {
                pixel[0..3].copy_from_slice(&self.bg);
                pixel[3] = 255;
            }
        }
        self.window.request_redraw();
    }

    /// Makes a beeping noise using [`rodio`].
    fn beep(&self, beep: bool) {
        if beep {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }
}

#[derive(Debug, Error)]
pub enum PixelsFrontendError {
    #[error("{0}")]
    Args(#[from] ArgsError),
    #[error("{0}")]
    EventLoop(#[from] EventLoopError),
    #[error("{0}")]
    Os(#[from] OsError),
    #[error("{0}")]
    Pixels(#[from] Error),
    #[error("{0}")]
    Stream(#[from] StreamError),
    #[error("Window close requested.")]
    WindowClose,
    #[error("{0}")]
    Texture(#[from] TextureError),
    #[error("{0}")]
    Chip8(#[from] Chip8Error),
    #[error("{0}")]
    PlayError(#[from] PlayError),
}

/// Same old "exciting" entry point.
fn main() -> ExitCode {
    if let Err(err) = main_loop() {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Gets the ROM from the given path and starts the main instruction loop with
/// some options/settings.
fn main_loop() -> Result<(), PixelsFrontendError> {
    let args = argh::from_env::<Args>();
    let chip8 = args.chip8()?;
    let frontend = PixelsFrontend::new(
        chip8,
        args.layout,
        args::hex_to_rgb(args.bg)?,
        args::hex_to_rgb(args.fg)?,
        &args.path.to_string_lossy(),
    )?;
    frontend.main_loop()?;
    Ok(())
}
