use std::rc::Rc;
use std::{env, process, u16, usize};
const MEMORY_MAX: usize = 1 << 16;
const MEMORY: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
const R_COUNT: usize = 10;
#[allow(non_camel_case_types)]
enum Registers {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC, /* The program counter */
    R_COND,
}
#[allow(non_camel_case_types)]
enum ConditionFlags {
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2,
}
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
enum OpCodes {
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP,   /* execute trap */
}
impl OpCodes {
    fn from_u16(value: u16) -> Option<OpCodes> {
        match value {
            0 => Some(OpCodes::OP_BR),
            1 => Some(OpCodes::OP_ADD),
            2 => Some(OpCodes::OP_LD),
            3 => Some(OpCodes::OP_ST),
            4 => Some(OpCodes::OP_JSR),
            5 => Some(OpCodes::OP_AND),
            6 => Some(OpCodes::OP_LDR),
            7 => Some(OpCodes::OP_STR),
            8 => Some(OpCodes::OP_RTI),
            9 => Some(OpCodes::OP_NOT),
            10 => Some(OpCodes::OP_LDI),
            11 => Some(OpCodes::OP_STI),
            12 => Some(OpCodes::OP_JMP),
            13 => Some(OpCodes::OP_RES),
            14 => Some(OpCodes::OP_LEA),
            _ => None,
        }
    }
}
use ConditionFlags::*;
use OpCodes::*;
use Registers::*;
fn mem_read(PC: usize) -> OpCodes {
    todo!("Mem read is not implemented");
}
fn read_image(arg: &str) -> bool {
    todo!("Read image is not implemented");
}
fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    if x >> (bit_count - 1) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}
fn update_flags(result: u16, reg: &mut [u16; R_COUNT]) {
    let r = result as usize;
    let r_cond = R_COND as usize;
    if reg[r] == 0 {
        reg[r_cond] = FL_ZRO as u16;
    } else if reg[r] >> 15 != 0
    /* a 1 in the left-most bit indicates that the result is negative*/
    {
        reg[r_cond] = FL_NEG as u16;
    } else {
        reg[r_cond] = FL_POS as u16;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("lc3vm [image-file1] ...");
        std::process::exit(2);
    }
    for arg in args {
        if !read_image(&arg) {
            println!("failed to load image: {}", arg);
            std::process::exit(1);
        }
    }
    let mut reg: [u16; R_COUNT] = [0; R_COUNT];
    reg[R_COND as usize] = FL_ZRO as u16;
    let PC_START = 0x3000;
    reg[R_PC as usize] = PC_START as u16;
    let mut running = true;
    while running {
        reg[R_PC as usize] += 1;
        let current_pos = reg[R_PC as usize];
        let instr = mem_read(current_pos.into());
        let instr_op = instr as u16 >> 12;
        let op = OpCodes::from_u16(instr_op);
        match op {
            Some(op) => match op {
                OP_ADD => {
                    let instr = instr as u16;
                    // Destination register
                    let r0 = instr >> 9 & 0x7;
                    // First operand(SR1)
                    let r1 = instr >> 6 & 0x7;
                    // Whether we are in immediate mode
                    let imm_flag = instr >> 5 & 0x1;

                    if imm_flag != 0 {
                        let imm5 = sign_extend(instr & 0x1F, 5);
                        reg[r0 as usize] = reg[r1 as usize] + imm5;
                    } else {
                        let r2 = instr & 0x7;
                        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
                    }
                    update_flags(r0, &mut reg);
                }
                OP_AND => {}
                OP_NOT => {}
                OP_BR => {}
                OP_JMP => {}
                OP_JSR => {}
                OP_LD => {}
                OP_LDI => {}
                OP_LDR => {}
                OP_LEA => {}
                OP_ST => {}
                OP_STI => {}
                OP_STR => {}
                OP_TRAP => {}
                OP_RES => {}
                OP_RTI => {}
            },
            None => {}
        }
    }
}
