mod chip8;
mod screen;

use chip8::Chip8;
use screen::Screen;

fn main() {
    let mut screen = Screen::new();
    let mut chip8 = Chip8::new();
}
