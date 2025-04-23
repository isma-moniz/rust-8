extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub struct Screen {
    pub sdl_context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Rust-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        Screen {
            sdl_context,
            canvas,
        }
    }

    fn draw(&mut self, buffer: &[u32; 64 * 32]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        for (i, pixel) in buffer.iter().enumerate() {
            if *pixel != 0 {
                let x = (i % 64) as i32 * 10;
                let y = (i / 64) as i32 * 10;
                self.canvas.set_draw_color(Color::WHITE);
                let _ = self.canvas.fill_rect(Rect::new(x, y, 10, 10));
            }
        }

        self.canvas.present();
    }

    pub fn display_loop(&mut self, buffer: &[u32; 64 * 32]) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        scancode: Some(Scancode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {
                        println!("No match for: {:?}", event);
                    }
                }
            }

            self.draw(buffer);
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
