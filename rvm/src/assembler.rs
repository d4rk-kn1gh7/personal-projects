use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::instruction::Opcode;

#[derive(Debug)]
pub struct Asm {
    bytecode: Vec<u8>
}

impl Asm {
    pub fn new() -> Asm {
        Asm {
            bytecode: vec![]
        }
    }

    fn get_reg(&mut self, data: &str) {
        assert!(&data[0..1] == "R", "Not a valid register!");
        let num: &str = &data[1..].replace(",", "");
        self.bytecode.push(num.parse::<u8>().unwrap());
    }

    fn get_num(&mut self, data: &str, size: usize) {
        let number: u16;
        if data.len() > 2 && &data[..2] == "0x" {
            number = u16::from_str_radix(data.trim_start_matches("0x"), 16).unwrap();
        }
        else {
            number = data.parse().unwrap();
        }
        match size {
            1 => {
                self.bytecode.push(number as u8);
            }
            2 => {
                self.bytecode.push(((number >> 8) & 0xff) as u8);
                self.bytecode.push((number & 0xff) as u8);
            }
            _ => {
            }
        }
    }

    fn update_bytecode(&mut self, data: Vec<&str>) {
        let op: Opcode = Opcode::from(data[0]);
        match op {
            Opcode::PUSH | Opcode::POP | Opcode::READ | Opcode::WRITE => { // Single register
                self.bytecode.push(u8::from(op));
                self.get_reg(data[1]);
            },
            Opcode::LD8 | Opcode::LS | Opcode::RS => { // Single register, 8 bit value
                self.bytecode.push(u8::from(op));
                self.get_reg(data[1]);
                self.get_num(data[2], 1);
            },
            Opcode::LD16 => { // Single register, 16 bit value
                self.bytecode.push(u8::from(op));
                self.get_reg(data[1]);
                self.get_num(data[2], 2);
            },
            Opcode::MOV | Opcode::ADD | Opcode::SUB |
            Opcode::MULT | Opcode::DIV | Opcode::MOD |
            Opcode::EQ | Opcode::GT | Opcode::LT => { // Two registers
                self.bytecode.push(u8::from(op));
                self.get_reg(data[1]);
                self.get_reg(data[2]);
            },
            Opcode::JMP | Opcode::JE | Opcode::JNE |
            Opcode::JF | Opcode::JB => { // Single 8 bit value
                self.bytecode.push(u8::from(op));
                self.get_num(data[1], 1);
            },
            Opcode::QUIT => {
                self.bytecode.push(u8::from(op));
            }
            _ => {
                self.bytecode.push(u8::from(Opcode::ILL));
            }
        }
    }

    pub fn parse_to_bytecode(&mut self, path: &str) -> Vec<u8> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line2: String = line.unwrap();
            let data: Vec<&str> = line2.split_whitespace().collect();
            self.update_bytecode(data);
        }
        return self.bytecode.clone();
    }
}
