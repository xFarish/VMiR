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
            }

            _ => cpu::Immediate::None()
        }
    }
}