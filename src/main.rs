mod chip8;
mod screen;

use chip8::Chip8;
use screen::Screen;
use std::env;
use std::time::{Duration, Instant};

const IPS: u32 = 700;
const CLOCK_RATE: u8 = 60;
const NANOS_PER_INSTRUCTION: Duration = Duration::from_nanos(1_000_000_000 / IPS as u64);
const NANOS_PER_CLOCK: Duration = Duration::from_nanos(1_000_000_000 / CLOCK_RATE as u64);

fn main() {
    let mut screen = Screen::new();
    let mut chip8 = Chip8::new();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    chip8
        .load_rom(file_path)
        .expect("File {file_path} not found.");

    let mut last_instruction_time = Instant::now();
    let mut last_clock_time = Instant::now();

    'running: loop {
        if !screen.process_input(chip8.get_keypad()) {
            break 'running;
        }

        if last_instruction_time.elapsed() >= NANOS_PER_INSTRUCTION {
            chip8.tick();
            last_instruction_time += NANOS_PER_INSTRUCTION;
        }

        if last_clock_time.elapsed() >= NANOS_PER_CLOCK {
            chip8.tick_clock();
            last_clock_time += NANOS_PER_INSTRUCTION;
        }

        screen.draw(chip8.get_video_buffer());

        std::thread::sleep(Duration::from_micros(100)); // a little cpu nap
    }
}
