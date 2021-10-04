# RVM

A small register-based virtual machine written entirely from scratch in Rust.

Registers are numbered R0-R7, and each is 32 bits in size.

The vm also contains a stack (a 32 bit vector), and a few special registers:
+ pc - program counter
+ sp - stack pointer
+ bp - base pointer
+ eq - equality flag

## List of opcodes

| Opcode Name | Argument 1 | Argument 2 | Description |
|:-----------:|:----------:|:----------:|:-----------:|
| PUSH | Register | - | Pushes the value stored in a register onto the stack |
| POP | Register | - | Pops the value on the top of the stack into a register |
| LD8 | Register | 8-bit number | Load a 8 bit value into a register |
| LD16 | Register | 16-bit number | Load a 16 bit value into a register |
| MOV | Register | Register | Move value stored in second register to first register |
| ADD | Register | Register | Add values in 2 registers, result stored in first one |
| SUB | Register | Register | Subtract values in 2 registers, result stored in first one |
| MULT | Register | Register | Multiply values in 2 registers, result stored in first one |
| DIV | Register | Register | Divide values in 2 registers, result stored in first one |
| MOD | Register | Register | Modulus of values in 2 registers, result stored in first one |
| LS | Register | 8-bit value | Left shift the value in a register by 8-bit value |
| RS | Register | 8-bit value | Right shift the value in a register by 8-bit value |
| EQ | Register | Register | Set equality flag if values in 2 registers are equal |
| GT | Register | Register | Set equality flag if value in reg1 > reg2 |
| LT | Register | Register | Set equality flag if value in reg1 < reg2 |
| JMP | 8-bit value | - | Set the PC to the given value |
| JE | 8-bit value | - | Set the PC to the given value if equality flag is set |
| JNE | 8-bit value | - | Set the PC to the given value if equality flag is not set |
| JF | 8-bit value | - | Relative jump forwards by value (add pc) |
| JB | 8-bit value | - | Relative jump backwards by value (subtract pc) |
| READ | Register | - | Reads a 32 bit integer into a register from stdin |
| WRITE | Register | - | Writes the value in a register to stdout |

## Running
You can run the project with `cargo run <path-to-asm-file>`
A sample asm file is [test.asm](test.asm)
