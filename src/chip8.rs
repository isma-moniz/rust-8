use rand::prelude::*;
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

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const FONT_ADDRESS: usize = 0x050;

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
    rng: ThreadRng,
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
            video_buffer: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            registers: [0; 16],
            keypad: [false; 16],
            opcode: 0,
            rng: rand::rng(),
        }
    }

    pub fn get_video_buffer(&self) -> &[u32; 64 * 32] {
        &self.video_buffer
    }

    pub fn get_keypad(&mut self) -> &mut [bool; 16] {
        &mut self.keypad
    }

    pub fn tick_clock(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
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
                    self.subn_registers(x, y);
                }
                0xE => {
                    self.shl_register(x);
                }
                _ => {}
            },
            0x9 => {
                self.sne(x, y);
            }
            0xA => {
                self.load_address_to_index(nnn);
            }
            0xB => {
                self.jump_v0(nnn);
            }
            0xC => {
                self.rnd_and_byte(x, nn);
            }
            0xD => {
                self.draw(x, y, n);
            }
            0xE => match nn {
                0x9E => {
                    self.skip_key(x);
                }
                0xA1 => {
                    self.skip_not_key(x);
                }
                _ => {}
            },
            0xF => match nn {
                0x07 => {
                    self.load_timer(x);
                }
                0x0A => {
                    self.load_key(x);
                }
                0x15 => {
                    self.set_delay(x);
                }
                0x18 => {
                    self.set_sound(x);
                }
                0x1E => {
                    self.add_to_index(x);
                }
                0x29 => {
                    self.ld_digit_to_index(x);
                }
                0x33 => {
                    self.bcd_to_index(x);
                }
                0x55 => {
                    self.store_to_index(x);
                }
                0x65 => {
                    self.read_from_index(x);
                }
                _ => {}
            },

            _ => {}
        }
    }

    // instruction set

    fn clear_screen(&mut self) {
        self.video_buffer = [0; VIDEO_WIDTH * VIDEO_HEIGHT];
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
        self.registers[v_x as usize] = self.registers[v_x as usize].wrapping_add(byte);
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

    fn sub_registers(&mut self, v_x: u8, v_y: u8) {
        let sub: u16 =
            (self.registers[v_x as usize] as u16).wrapping_sub(self.registers[v_y as usize] as u16);
        if self.registers[v_x as usize] > self.registers[v_y as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[v_x as usize] = (sub & 0x00FFu16) as u8;
    }

    fn subn_registers(&mut self, v_x: u8, v_y: u8) {
        let sub: u16 =
            (self.registers[v_y as usize] as u16).wrapping_sub(self.registers[v_y as usize] as u16);
        if self.registers[v_y as usize] > self.registers[v_x as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.registers[v_x as usize] = (sub & 0x00FFu16) as u8;
    }

    fn shr_register(&mut self, v_x: u8) {
        let least_significant_bit: u8 = v_x & 0x01u8;
        self.registers[0xF] = least_significant_bit;
        self.registers[v_x as usize] >>= 1;
    }

    fn shl_register(&mut self, v_x: u8) {
        let most_significant_bit: u8 = (v_x & 0xF0u8) >> 7;
        self.registers[0xF] = most_significant_bit;
        self.registers[v_x as usize] <<= 1;
    }

    fn sne(&mut self, v_x: u8, v_y: u8) {
        if self.registers[v_x as usize] != self.registers[v_y as usize] {
            self.pc += 2;
        }
    }

    fn load_address_to_index(&mut self, addr: u16) {
        self.index = addr;
    }

    fn jump_v0(&mut self, addr: u16) {
        self.pc = addr + self.registers[0 as usize] as u16;
    }

    fn rnd_and_byte(&mut self, v_x: u8, byte: u8) {
        let random_byte: u8 = self.rng.random();
        self.registers[v_x as usize] = random_byte & byte;
    }

    fn draw(&mut self, v_x: u8, v_y: u8, height: u8) {
        let x: u16 = self.registers[v_x as usize] as u16;
        let y: u16 = self.registers[v_y as usize] as u16;
        self.registers[0xF] = 0; // collision flag reset

        for row in 0..height {
            let sprite_byte = self.memory[self.index as usize + row as usize];
            for col in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                if sprite_pixel == 0 {
                    continue;
                }
                let x_pos = (x + col) % VIDEO_WIDTH as u16;
                let y_pos = (y + row as u16) % VIDEO_HEIGHT as u16;
                let buffer_index = (y_pos * VIDEO_WIDTH as u16 + x_pos) as usize;

                let screen_pixel = &mut self.video_buffer[buffer_index as usize];
                if *screen_pixel == 0xFFFFFFFF {
                    self.registers[0xF] = 1; // Collision detected
                }
                *screen_pixel ^= 0xFFFFFFFF; // Toggle pixel
            }
        }
    }

    fn skip_key(&mut self, v_x: u8) {
        let key = self.registers[v_x as usize];
        if self.keypad[key as usize] {
            self.pc += 2;
        }
    }

    fn skip_not_key(&mut self, v_x: u8) {
        let key = self.registers[v_x as usize];
        if !self.keypad[key as usize] {
            self.pc += 2;
        }
    }

    fn load_timer(&mut self, v_x: u8) {
        self.registers[v_x as usize] = self.delay_timer;
    }

    fn load_key(&mut self, v_x: u8) {
        let mut key: i8 = -1;
        for i in 0..15 {
            if self.keypad[i] {
                key = i as i8;
                break;
            }
        }
        if key == -1 {
            self.pc -= 2;
        } else {
            self.registers[v_x as usize] = key as u8;
        }
    }

    fn set_delay(&mut self, v_x: u8) {
        self.delay_timer = self.registers[v_x as usize];
    }

    fn set_sound(&mut self, v_x: u8) {
        self.sound_timer = self.registers[v_x as usize];
    }

    fn add_to_index(&mut self, v_x: u8) {
        self.index += self.registers[v_x as usize] as u16;
    }

    fn ld_digit_to_index(&mut self, v_x: u8) {
        self.index = FONT_ADDRESS as u16 + (5 * self.registers[v_x as usize]) as u16;
    }

    fn bcd_to_index(&mut self, v_x: u8) {
        let mut value: u8 = self.registers[v_x as usize];

        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;

        self.memory[self.index as usize + 1] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    fn store_to_index(&mut self, v_x: u8) {
        for i in 0..=v_x as usize {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    fn read_from_index(&mut self, v_x: u8) {
        for i in 0..=v_x as usize {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }
}
