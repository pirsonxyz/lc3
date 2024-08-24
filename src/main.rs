use std::{u16, usize};

const MEMORY_MAX: usize = 1 << 16;
const MEMORY: [u16; MEMORY_MAX ] = [0; MEMORY_MAX]; 
const R_COUNT: usize = 10;
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
const REG: [u16; R_COUNT] = [0; R_COUNT];
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
    OP_TRAP    /* execute trap */
}
fn main() {
    println!("Hello, world!");
}
