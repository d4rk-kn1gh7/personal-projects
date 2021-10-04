#[derive(Debug, PartialEq)]
pub enum Opcode {
    QUIT,
    PUSH,
    POP,
    LD8,
    LD16,
    MOV,
    ADD,
    SUB,
    MULT,
    DIV,
    MOD,
    LS,
    RS,
    EQ,
    GT,
    LT,
    JMP,
    JE,
    JNE,
    JF,
    JB,
    READ,
    WRITE,
    ILL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0x0 => return Opcode::QUIT,
            0x10 => return Opcode::PUSH,
            0x11 => return Opcode::POP,
            0x12 => return Opcode::LD8,
            0x13 => return Opcode::LD16,
            0x14 => return Opcode::MOV,
            0x20 => return Opcode::ADD,
            0x21 => return Opcode::SUB,
            0x22 => return Opcode::MULT,
            0x23 => return Opcode::DIV,
            0x24 => return Opcode::MOD,
            0x25 => return Opcode::LS,
            0x26 => return Opcode::RS,
            0x30 => return Opcode::EQ,
            0x31 => return Opcode::GT,
            0x32 => return Opcode::LT,
            0x40 => return Opcode::JMP,
            0x41 => return Opcode::JE,
            0x42 => return Opcode::JNE,
            0x43 => return Opcode::JF,
            0x44 => return Opcode::JB,
            0x51 => return Opcode::READ,
            0x52 => return Opcode::WRITE,
            _ => return Opcode::ILL
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::QUIT => return 0x0,
            Opcode::PUSH => return 0x10,
            Opcode::POP => return 0x11,
            Opcode::LD8 => return 0x12,
            Opcode::LD16 => return 0x13,
            Opcode::MOV => return 0x14,
            Opcode::ADD => return 0x20,
            Opcode::SUB => return 0x21,
            Opcode::MULT => return 0x22,
            Opcode::DIV => return 0x23,
            Opcode::MOD => return 0x24,
            Opcode::LS => return 0x25,
            Opcode::RS => return 0x26,
            Opcode::EQ => return 0x30,
            Opcode::GT => return 0x31,
            Opcode::LT => return 0x32,
            Opcode::JMP => return 0x40,
            Opcode::JE => return 0x41,
            Opcode::JNE => return 0x42,
            Opcode::JF => return 0x43,
            Opcode::JB => return 0x44,
            Opcode::READ => return 0x51,
            Opcode::WRITE => return 0x52,
            Opcode::ILL => return 0xff
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v {
            "QUIT" => return Opcode::QUIT,
            "PUSH" => return Opcode::PUSH,
            "POP" => return Opcode::POP,
            "LD8" => return Opcode::LD8,
            "LD16" => return Opcode::LD16,
            "MOV" => return Opcode::MOV,
            "ADD" => return Opcode::ADD,
            "SUB" => return Opcode::SUB,
            "MULT" => return Opcode::MULT,
            "DIV" => return Opcode::DIV,
            "MOD" => return Opcode::MOD,
            "LS" => return Opcode::LS,
            "RS" => return Opcode::RS,
            "EQ" => return Opcode::EQ,
            "GT" => return Opcode::GT,
            "LT" => return Opcode::LT,
            "JMP" => return Opcode::JMP,
            "JE" => return Opcode::JE,
            "JNE" => return Opcode::JNE,
            "JF" => return Opcode::JF,
            "JB" => return Opcode::JB,
            "READ" => return Opcode::READ,
            "WRITE" => return Opcode::WRITE,
            _ => return Opcode::ILL
        }
    }
}