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
            memory: Box::new([0u8; 4096]),
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
        let first_nibble: u8 = (self.opcode >> 12) as u8; // the first nibble
        let x: u8 = ((self.opcode >> 8) & 0x000Fu16) as u8; // the second nibble
        let y: u8 = ((self.opcode >> 4) & 0x000Fu16) as u8; // the third nibble
        let n: u8 = (self.opcode & 0x000Fu16) as u8; // the fourth nibble
        let nn: u8 = (self.opcode & 0x00FFu16) as u8; // second byte
        let nnn: u16 = (self.opcode & 0x0FFFu16) as u16; // second, third, fourth nibbles

        match first_nibble {
            0x0 => match nn {
                0xE0 => {
                    self.clear_screen();
                }
                0xEE => {
                    self.ret();
                }
                _ => {
                    // ignore the rest
                }
            },
            0x1 => {
                self.jump(nnn);
            }
            0x2 => {
                self.call(nnn);
            }
            0x3 => {
                self.skip_if_equals_byte(x, nn);
            }
            0x4 => {
                self.skip_if_not_equals_byte(x, nn);
            }
            0x5 => {
                self.skip_if_equals_registers(x, y);
            }
            0x6 => {
                self.load(x, nn);
            }
            0x7 => {
                self.add_to_register(x, nn);
            }
            0x8 => match n {
                0x0 => {
                    self.load_register(x, y);
                }
                0x1 => {
                    self.or_registers(x, y);
                }
                0x2 => {
                    self.and_registers(x, y);
                }
                0x3 => {
                    self.xor_registers(x, y);
                }
                0x4 => {
                    self.add_registers(x, y);
                }
                0x5 => {
                    self.sub_registers(x, y);
                }
                0x6 => {
                    self.shr_register(x);
                }
                0x7 => {
                    self.sub_registers(x, y);
                }
                0xE => {
                    self.shl_register(x);
                }
                _ => {}
            },
            _ => {}
        }
    }

    // instruction set

    fn clear_screen(&mut self) {
        self.video_buffer = [0; 64 * 32];
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn call(&mut self, address: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn skip_if_equals_byte(&mut self, v_x: u8, byte: u8) {
        if self.registers[v_x as usize] == byte {
            self.pc += 2;
        }
    }

    fn skip_if_not_equals_byte(&mut self, v_x: u8, byte: u8) {
        if self.registers[v_x as usize] != byte {
            self.pc += 2;
        }
    }

    fn skip_if_equals_registers(&mut self, v_x: u8, v_y: u8) {
        if self.registers[v_x as usize] == self.registers[v_y as usize] {
            self.pc += 2;
        }
    }

    fn load(&mut self, v_x: u8, byte: u8) {
        self.registers[v_x as usize] = byte;
    }

    fn add_to_register(&mut self, v_x: u8, byte: u8) {
        self.registers[v_x as usize] += byte;
    }

    fn load_register(&mut self, v_x: u8, v_y: u8) {
        self.registers[v_x as usize] = self.registers[v_y as usize];
    }

    fn or_registers(&mut self, v_x: u8, v_y: u8) {
        self.registers[v_x as usize] |= self.registers[v_y as usize];
    }

    fn and_registers(&mut self, v_x: u8, v_y: u8) {
        self.registers[v_x as usize] &= self.registers[v_y as usize];
    }

    fn xor_registers(&mut self, v_x: u8, v_y: u8) {
        self.registers[v_x as usize] ^= self.registers[v_y as usize];
    }

    fn add_registers(&mut self, v_x: u8, v_y: u8) {
        let sum: u16 = self.registers[v_x as usize] as u16 + self.registers[v_y as usize] as u16;
        if sum > 255u16 {
            self.registers[0xF] = 1; // overflow flag
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[v_x as usize] = (sum & 0x00FFu16) as u8;
    }
}
