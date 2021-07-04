use std::convert::TryInto;
use derive_more::*;
use std::mem;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Add, Sub)]
pub enum Immediate {
    None(),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64)
}

pub type Reg = usize;
pub type Adr = usize;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NOP(),
    MOV(Reg, Immediate),
    MOVR(Reg, Reg),
    JMP(Reg),
    JE(Reg),
    JNE(Reg),
    JG(Reg)
}