use std::fs::File;
use std::io::Read;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    memory: Box<[u8; 4096]>,
    pc: u16,
    stack: [u16; 16],
    sp: u16,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
    video_buffer: [u32; 64 * 32],
    registers: [u8; 16],
    keypad: [bool; 16],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        // hardcoding the font into memory
        let mut memory = [0u8; 4096];
        let font_start = 0x050;
        for (i, byte) in FONT.iter().enumerate() {
            memory[font_start + i] = *byte;
        }
        // TODO: i'm putting all zeroes for now, need to check actual init values
        Self {
            memory: Box::new(memory),
            pc: 0x200,
            stack: [0u16; 16],
            sp: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            video_buffer: [0; 64 * 32],
            registers: [0; 16],
            keypad: [false; 16],
            opcode: 0,
        }
    }

    pub fn get_video_buffer(&self) -> &[u32; 64 * 32] {
        &self.video_buffer
    }

    pub fn tick_clock(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
    }

    pub fn load_rom(&mut self, file_name: &String) -> std::io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let start = 0x200;
        let end = start + buffer.len();
        if end > self.memory.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "Rom is too large to fit in memory!",
            ));
        }

        self.memory[start..end].copy_from_slice(&buffer);

        Ok(())
    }

    pub fn tick(&mut self) {
        // fetch
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;
        self.opcode = high_byte << 8 | low_byte;

        self.pc += 2;

        // decode and execute

        // we start by capturing and storing the nibbles (4 bit)
        let first_nibble: u16 = self.opcode >> 12; // the first nibble
        let x: u16 = (self.opcode >> 8) & 0x000F; // the second nibble
        let y: u16 = (self.opcode >> 4) & 0x000F; // the third nibble
        let n: u16 = self.opcode & 0x000F; // the fourth nibble
        let nn: u16 = self.opcode & 0x00FF; // second byte
        let nnn: u16 = self.opcode & 0x0FFF; // second, third, fourth nibbles

        match first_nibble {
            0x0 => match nn {
                0xE0 => {
                    // clear screen
                }
                0xEE => {
                    // RET
                }
                _ => {
                    // ignore the rest
                }
            },
            _ => {}
        }
    }
}
