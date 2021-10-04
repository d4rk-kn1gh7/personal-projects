use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [u32; 8],
    pc: usize,
    sp: usize,
    bp: usize,
    eq: bool,
    stack: Vec<u32>,
    pub instructions: Vec<u8>
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 8],
            pc: 0,
            sp: 0,
            bp: 0,
            eq: false,
            stack: vec![0; 0x100],
            instructions: vec![]
        }
    }

    fn parse_operands(&mut self, size: u32) -> u16 {
        match size {
            1 => { //Size of operand = 1 byte
                self.pc += 1;
                return self.instructions[self.pc - 1] as u16;
            },
            2 => { //Size of operand = 2 bytes
                self.pc += 2;
                return ((self.instructions[self.pc - 2] as u16) << 8) | self.instructions[self.pc - 1] as u16;
            },
            _ => {
                println!("Something went wrong!!");
                return 0;
            }
        }
    }

    fn get_instruction(&mut self) -> Opcode {
        self.pc += 1;
        return Opcode::from(self.instructions[self.pc - 1]);
    }

    fn execute(&mut self, op: Opcode) {
        match op {
            Opcode::QUIT => {
                println!("Bye!");
                return;
            },
            Opcode::PUSH => { // Push the value stored in a register onto the stack
                assert!(self.sp != 0xff, "Stack is full!");
                self.stack[self.sp] = self.registers[self.parse_operands(1) as usize];
                self.sp += 1;
            },
            Opcode::POP => { // Pop the value on the top of the stack into a register
                assert!(self.sp != 0, "Stack is empty!");
                self.sp -= 1;
                self.registers[self.parse_operands(1) as usize] = self.stack[self.sp];
                self.stack[self.sp] = 0;
            },
            Opcode::LD8 => { // Load a 8 bit value into a register
                let reg = self.parse_operands(1) as usize;
                let number = self.parse_operands(1) as u16;
                self.registers[reg] = number as u32;
            },
            Opcode::LD16 => { // Load a 16 bit value into a register
                let reg = self.parse_operands(1) as usize;
                let number = self.parse_operands(2) as u16;
                self.registers[reg] = number as u32;
            },
            Opcode::MOV => { // Move value stored in second to first register
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg2];
            },
            Opcode::ADD => { // Add 2 registers, result stored in first one
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] + self.registers[reg2];
            },
            Opcode::SUB => { // Subtract 2 registers, result stored in first one
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] - self.registers[reg2];
            },
            Opcode::MULT => { // Multiply 2 registers, result stored in first one
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] * self.registers[reg2];
            },
            Opcode::DIV => { // Divide 2 registers, result stored in first one
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] / self.registers[reg2];
            },
            Opcode::MOD => { // Modulus of 2 registers, result stored in first one
                let reg1 = self.parse_operands(1) as usize;
                let reg2 = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] % self.registers[reg2];
            },
            Opcode::LS => { // Left shift the value in a register
                let reg1 = self.parse_operands(1) as usize;
                let val = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] << val;
            },
            Opcode::RS => { // Right shift the value in a register
                let reg1 = self.parse_operands(1) as usize;
                let val = self.parse_operands(1) as usize;
                self.registers[reg1] = self.registers[reg1] >> val;
            },
            Opcode::EQ => { // Set equality flag if 2 registers are equal
                let val1 = self.registers[self.parse_operands(1) as usize];
                let val2 = self.registers[self.parse_operands(1) as usize];
                self.eq = val1 == val2;
            },
            Opcode::GT => { // Set equality flag if reg1 > reg2
                let val1 = self.registers[self.parse_operands(1) as usize];
                let val2 = self.registers[self.parse_operands(1) as usize];
                self.eq = val1 > val2;
            },
            Opcode::LT => { // Set equality flag if reg1 < reg2
                let val1 = self.registers[self.parse_operands(1) as usize];
                let val2 = self.registers[self.parse_operands(1) as usize];
                self.eq = val1 < val2;
            },
            Opcode::JMP => { // Set the PC to the given value
                self.pc = self.parse_operands(1) as usize;
            },
            Opcode::JE => { // Set PC if equality flag is set
                let var = self.parse_operands(1) as usize;
                if self.eq {
                    self.pc = var;
                }
            },
            Opcode::JNE => { // Set PC if equality flag is not set
                let var = self.parse_operands(1) as usize;
                if !self.eq {
                    self.pc = var;
                }
            },
            Opcode::JF => { // Relative jump forwards
                self.pc += self.parse_operands(1) as usize;
            },
            Opcode::JB => { // Relative jump backwards
                self.pc -= self.parse_operands(1) as usize;
            },
            Opcode::READ => { // Reads a 32 bit integer into a register from stdin
                let reg = self.parse_operands(1) as usize;
                let number: u32;
                let data: String = read!("{}\n");
                if data.len() > 2 && &data[..2] == "0x" {
                    number = u32::from_str_radix(data.trim_start_matches("0x"), 16).unwrap();
                }
                else {
                    number = data.parse().unwrap();
                }
                self.registers[reg] = number;
            },
            Opcode::WRITE => { // Writes the value in a register to stdout
                let reg = self.parse_operands(1) as usize;
                println!("{}", self.registers[reg]);
            },
            _ => {
                println!("{:#02x} -> Unrecognized opcode!", self.instructions[self.pc - 1]);
                return;
            }
        }
    } 

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.instructions.len() {
                break;
            }
            let next_op: Opcode = self.get_instruction();
            self.execute(next_op);
        }
    }

    pub fn print_reg_state(&mut self) {
        println!("Register state:");
        for i in 0..8 {
            println!("R{} -> {:#02x}", i, self.registers[i]);
        }
        println!("PC -> {:#02x}", self.pc);
        println!("SP -> {:#02x}", self.sp);
        println!("EQ -> {}", self.eq);
    }
}
