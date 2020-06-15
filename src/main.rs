use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::hash_map::DefaultHasher;
use std::io::{stdout, Write, BufReader};
use crossterm::{execute, terminal};
use crossterm::style::Colorize;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;
use std::{env, fs};
use rodio::Source;

enum Color {
    White,
    Red,
    Green,
    Blue,
}

struct Interpreter {
    memory: Vec<u8>,
    register: u8,
    pointer: usize,
    ins_pointer: usize,
    code: Vec<String>,
    color: Color,
}

impl Interpreter {
    fn new(code_text: &str) -> Self {
        Self {
            memory: vec!(0, 0, 0, 0),
            register: 0,
            pointer: 0,
            ins_pointer: 0,
            code: code_text.split_ascii_whitespace().map(|i| String::from(i)).collect(),
            color: Color::White,
        }
    }

    fn do_ins(&mut self) {
        if self.ins_pointer == self.code.len() {
            return;
        }
        match &self.code[self.ins_pointer][..] {
            "bruH" => {
                self.pointer += 1;
                if self.pointer >= self.memory.len() {
                    self.memory.push(0);
                }
                self.ins_pointer += 1;
            },
            "Bruh" => {
                if self.pointer == 0 {
                    panic!("Memory pointer went into negitives.")
                }
                self.pointer -= 1;
                self.ins_pointer += 1;
            },
            "brUh" => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                self.ins_pointer += 1;
            },
            "bRuh" => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                self.ins_pointer += 1;
            },
            "BruH" => {
                self.register = self.register.wrapping_add(self.memory[self.pointer]);
                self.ins_pointer += 1;
            },
            "BrUH" => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(self.register);
                self.ins_pointer += 1;
            },
            "BRuH" => {
                self.register = self.register.wrapping_add(Self::get_random());
                self.ins_pointer += 1;
            },
            "BrUh" => {
                self.register = 0;
                self.ins_pointer += 1;
            },
            "bruh" => {
                if let Ok(new_pointer) = self.find_matching() {
                    if self.register == 0 {
                        self.ins_pointer = new_pointer + 1;
                    } else {
                        self.ins_pointer += 1;
                    }
                } else {
                    panic!("No matching BRUH.");
                }
            },
            "BRUH" => {
                if let Ok(new_pointer) = self.find_matching() {
                    if self.register != 0 {
                        self.ins_pointer = new_pointer;
                    } else {
                        self.ins_pointer += 1;
                    }
                } else {
                    panic!("No matching bruh.");
                }
            },
            "bRUh" => {
                self.memory[self.pointer] = Self::read_key_press();
                self.ins_pointer += 1;
            },
            "bRUH" => {
                self.print_with_color(format!("{}", self.memory[self.pointer]));
                self.ins_pointer += 1;
            },
            "BRUh" => {
                self.print_with_color(format!("{}", self.memory[self.pointer] as char));
                self.ins_pointer += 1;
            },
            "bRuH" => {
                match self.memory[self.pointer] {
                    0 => self.color = Color::White,
                    1 => self.color = Color::Red,
                    2 => self.color = Color::Green,
                    3 => self.color = Color::Blue,
                    _ => panic!("Unknown color: {}", self.memory[self.pointer]),
                }
                self.ins_pointer += 1;
            },
            "BRuh" => {
                execute!(stdout(), terminal::Clear(terminal::ClearType::All)).expect("Clear screen faild.");
                self.ins_pointer += 1;
            },
            "brUH" => {
                let device = rodio::default_output_device().expect("Rodio faild to bind");
                let audio = rodio::Decoder::new(BufReader::new(fs::File::open("sound.mp3").expect("Faild to get sound file."))).expect("Faild to make audio.");
                rodio::play_raw(&device, audio.convert_samples());
                self.ins_pointer += 1;
            },
            _ => panic!("Unknown command: {}.", self.code[self.ins_pointer])
        }
    }

    fn print_with_color(&self, text: String) {
        match self.color {
            Color::White => print!("{}", text.white()),
            Color::Red => print!("{}", text.red()),
            Color::Green => print!("{}", text.green()),
            Color::Blue => print!("{}", text.blue()),
        }
        stdout().flush().expect("Stdout flush faild.");
    }

    fn read_key_press() -> u8 {
        let device_state = DeviceState::new();
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();

            if keys.len() > 0 {
                match keys[0] {
                    Keycode::Key0 => return 0,
                    Keycode::Key1 => return 1,
                    Keycode::Key2 => return 2,
                    Keycode::Key3 => return 3,
                    Keycode::Key4 => return 4,
                    Keycode::Key5 => return 5,
                    Keycode::Key6 => return 6,
                    Keycode::Key7 => return 7,
                    Keycode::Key8 => return 8,
                    Keycode::Key9 => return 9,
                    Keycode::A => return 'a' as u8,
                    Keycode::B => return 'b' as u8,
                    Keycode::C => return 'c' as u8,
                    Keycode::D => return 'd' as u8,
                    Keycode::E => return 'e' as u8,
                    Keycode::F => return 'f' as u8,
                    Keycode::G => return 'g' as u8,
                    Keycode::H => return 'h' as u8,
                    Keycode::I => return 'i' as u8,
                    Keycode::J => return 'j' as u8,
                    Keycode::K => return 'k' as u8,
                    Keycode::L => return 'l' as u8,
                    Keycode::M => return 'm' as u8,
                    Keycode::N => return 'n' as u8,
                    Keycode::O => return 'o' as u8,
                    Keycode::P => return 'p' as u8,
                    Keycode::Q => return 'q' as u8,
                    Keycode::R => return 'r' as u8,
                    Keycode::S => return 's' as u8,
                    Keycode::T => return 't' as u8,
                    Keycode::U => return 'u' as u8,
                    Keycode::V => return 'v' as u8,
                    Keycode::W => return 'w' as u8,
                    Keycode::X => return 'x' as u8,
                    Keycode::Y => return 'y' as u8,
                    Keycode::Z => return 'z' as u8,
                    _ => {}
                }
            }
        }
    }

    fn find_matching(&self) -> Result<usize, &str> {
        if self.code[self.ins_pointer] == "bruh" {
            for i in self.ins_pointer..self.code.len() {
                if self.code[i] == "BRUH" {
                    return Ok(i)
                }
            }
            return Err("No matching BRUH.")
        } else {
            for i in (0..self.ins_pointer).rev() {
                if self.code[i] == "bruh" {
                    return Ok(i)
                }
            }
            return Err("No matching bruh.")
        }
    }

    fn get_random() -> u8 {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        hasher.finish() as u8
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code_text = fs::read_to_string(&args[1]).unwrap();
    let mut interpreter = Interpreter::new(&code_text);

    loop {
        interpreter.do_ins();
    }
}
