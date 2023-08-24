use crate::{chip8::Chip8, drv::Drv};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// Screen dimensions
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

// Drivers that use pixels and winit for graphics, input, and window creation, rodio for audio, and fastrand for randomness
pub struct PixelsDrv {
    chip8: Chip8,
    event_loop: Option<EventLoop<()>>,
    window: Window,
    pixels: Pixels,
}

impl PixelsDrv {
    fn event_handler(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => self.window_event_handler(event, control_flow),
            Event::MainEventsCleared => self.cycle(control_flow),
            Event::RedrawRequested(_) => self.pixels.render().expect("Error rendering"),
            _ => (),
        }
    }

    fn window_event_handler(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => self
                .pixels
                .resize_surface(size.width, size.height)
                .expect("Error resizing pixels surface"),
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::KeyboardInput { input, .. } => self.input_handler(input),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => self
                .pixels
                .resize_surface(new_inner_size.width, new_inner_size.height)
                .expect("Error resizing pixels surface"),
            _ => (),
        }
    }

    fn input_handler(&mut self, input: KeyboardInput) {}

    fn cycle(&mut self, control_flow: &mut ControlFlow) {
        if !self.chip8.cycle() {
            println!("Successfully finished executing program");
            control_flow.set_exit();
        }
        //self.window.request_redraw();
    }
}

impl Drv for PixelsDrv {
    fn new(chip8: Chip8) -> Self {
        let event_loop = EventLoop::new();
        let window = {
            let size = PhysicalSize::new(WIDTH, HEIGHT);
            WindowBuilder::new()
                .with_title("BISCUIT-8")
                .with_min_inner_size(size)
                .build(&event_loop)
                .expect("Error creating window")
        };
        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Error creating pixels framebuffer")
        };

        Self {
            chip8,
            event_loop: Some(event_loop),
            window,
            pixels,
        }
    }

    fn instruction_loop(mut self) {
        let event_loop = self
            .event_loop
            .expect("Event loop should've been initialized");
        self.event_loop = None;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            self.event_handler(event, control_flow);
        });
    }
}
