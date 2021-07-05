use std::convert::TryInto;
use std::mem;
use super::cpu;

pub struct VM {
    ip: cpu::Adr,
    flag_eq: bool,
    flag_gt: bool,
    reg: [cpu::Immediate; 8],
    code: Vec<u8>,
    stack: Vec<cpu::Immediate>,
    data: Vec<cpu::Immediate>,
    is_exec: bool
}

impl VM {
    // DEBUG
    #[allow(dead_code)]
    pub fn new(code: Vec<u8>, heap_capacity: usize) -> VM {
        VM {
            ip: 0,
            flag_eq: false,
            flag_gt: false,
            reg: [cpu::Immediate::U8(0); 8],
            code: code,
            stack: Vec::new(),
            data: vec![cpu::Immediate::U8(0); heap_capacity],
            is_exec: false
        }
    }

    pub fn decode_immediate(&mut self) -> cpu::Immediate {
        self.ip += 1;

        match self.code[self.ip] {
            0 => {
                self.ip += 1;
                cpu::Immediate::U8(self.code[self.ip] as u8)
            },

            1 => {
                self.ip += 1;
                cpu::Immediate::I8(self.code[self.ip] as i8)
            },

            2 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = u16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::U16(value)
            },

            3 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i16::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::I16(value)
            },

            4 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = u32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::U32(value)
            },

            5 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::I32(value)
            },

            6 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = u64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::U64(value)
            },

            7 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = i64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::I64(value)
            },

            8 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = f32::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::F32(value)
            },

            9 => {
                self.ip += 1;
                let size = mem::size_of::<i16>();
                let value = f64::from_le_bytes(self.code[self.ip..][..size].try_into().unwrap());
                self.ip += 1 - size;

                cpu::Immediate::F64(value)
            },

            _ => {
                cpu::Immediate::None()
            }
        }
    }

    pub fn decode(&mut self) -> cpu::Instruction {
        match self.code[self.ip] {
            0 => {
                cpu::Instruction::NOP()
            },

            1 => {
                self.ip += 1;
                let register = self.code[self.ip] as cpu::Reg;
                let var = self.decode_immediate();

                cpu::Instruction::MOV(register, var)
            },

            2 => {
                let register = (self.code[self.ip + 1] as cpu::Reg, self.code[self.ip + 2] as cpu::Reg);
                self.ip += 2;

                cpu::Instruction::MOVR(register.0, register.1)
            },

            3 => {
                self.ip += 1;
                let register = self.code[self.ip] as cpu::Reg;

                cpu::Instruction::JMP(register)
            }
            
            _ => {
                cpu::Instruction::NOP()
            }
        }
    }
}