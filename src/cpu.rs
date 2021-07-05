use derive_more::{Add, Sub};

// DEBUG
#[allow(dead_code)]
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

// DEBUG
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NOP(),
    MOV(Reg, Immediate),
    MOVR(Reg, Reg),
    JMP(Reg),
    JE(Reg),
    JNE(Reg),
    JG(Reg),
    JL(Reg),
    CMP(Reg, Reg),
    PRINTR(Reg),
    PRINTV(Adr),
    VSTORE(Adr, Immediate),
    VLOAD(Adr),
    VSTORER(Adr, Reg),
    VLOADR(Adr, Adr),
    ADD(Reg, Reg),
    SUB(Reg, Reg),
    MUL(Reg, Reg),
    DIV(Reg, Reg),
    AND(Reg, Reg),
    OR(Reg, Reg),
    XOR(Reg, Reg),
    SHR(Reg, Immediate),
    SHL(Reg, Immediate),
    VPUSH(Immediate),
    VPUSHR(Reg),
    VPOP(Reg),
    CALL(Reg),
    RET(),
    HALT()
}