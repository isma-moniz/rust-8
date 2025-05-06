extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Screen {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem
            .window("Rust-8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        Screen {
            sdl_context,
            canvas,
            event_pump,
        }
    }

    pub fn draw(&mut self, buffer: &[u32; 64 * 32]) {
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

    pub fn process_input(&mut self, keys: &mut [bool; 16]) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    return false; // quit
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Num1),
                    ..
                } => {
                    keys[1] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Num2),
                    ..
                } => {
                    keys[2] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Num3),
                    ..
                } => {
                    keys[3] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Num4),
                    ..
                } => {
                    keys[12] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Q),
                    ..
                } => {
                    keys[4] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::W),
                    ..
                } => {
                    keys[5] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::E),
                    ..
                } => {
                    keys[6] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::R),
                    ..
                } => {
                    keys[13] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::A),
                    ..
                } => {
                    keys[7] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::S),
                    ..
                } => {
                    keys[8] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::D),
                    ..
                } => {
                    keys[9] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::F),
                    ..
                } => {
                    keys[14] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::Z),
                    ..
                } => {
                    keys[10] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::X),
                    ..
                } => {
                    keys[0] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::C),
                    ..
                } => {
                    keys[11] = true;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::V),
                    ..
                } => {
                    keys[15] = true;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Num1),
                    ..
                } => {
                    keys[1] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Num2),
                    ..
                } => {
                    keys[2] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Num3),
                    ..
                } => {
                    keys[3] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Num4),
                    ..
                } => {
                    keys[12] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Q),
                    ..
                } => {
                    keys[4] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::W),
                    ..
                } => {
                    keys[5] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::E),
                    ..
                } => {
                    keys[6] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::R),
                    ..
                } => {
                    keys[13] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::A),
                    ..
                } => {
                    keys[7] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::S),
                    ..
                } => {
                    keys[8] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::D),
                    ..
                } => {
                    keys[9] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::F),
                    ..
                } => {
                    keys[14] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::Z),
                    ..
                } => {
                    keys[10] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::X),
                    ..
                } => {
                    keys[0] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::C),
                    ..
                } => {
                    keys[11] = false;
                }
                Event::KeyUp {
                    scancode: Some(Scancode::V),
                    ..
                } => {
                    keys[15] = false;
                }

                _ => {}
            }
        }
        true
    }
}
