const MEMORY_MAX: u16 = u16::MAX;

pub enum Opcode {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP    /* execute trap */
}

#[derive(Debug)]
pub struct InvalidConversion;
impl TryFrom<u16> for Opcode {
    type Error = InvalidConversion;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::BR),
            2 => Ok(Opcode::ADD),    /* add  */
            3 => Ok(Opcode::LD),     /* load */
            4 => Ok(Opcode::ST),     /* store */
            5 => Ok(Opcode::JSR),    /* jump register */
            6 => Ok(Opcode::AND),    /* bitwise and */
            7 => Ok(Opcode::LDR),    /* load register */
            8 => Ok(Opcode::STR),    /* store register */
            9 => Ok(Opcode::RTI),    /* unused */
            10 => Ok(Opcode::NOT),    /* bitwise not */
            11 => Ok(Opcode::LDI),    /* load indirect */
            12 => Ok(Opcode::STI),    /* store indirect */
            13 => Ok(Opcode::JMP),    /* jump */
            14 => Ok(Opcode::RES),    /* reserved (unused) */
            15 => Ok(Opcode::LEA),    /* load effective address */
            16 => Ok(Opcode::TRAP),    /* execute trap */
            _ => panic!("No such opcode")
        }
    }
}

pub enum Flag {
    Pos = 1 << 0,
    Zer = 1 << 1,
    Neg = 1 << 2
}

pub struct VM {
    pub memory: Vec<u16>,

    // LC3 uses 8 registers, each 16 bits wide for handling data.
    pub registers: Vec<u16>,

    // Program counter
    pub pc: u16,

    // Conditional register
    pub conditional: Flag,
}

impl VM {
    pub fn init() -> VM {
        return VM {
            memory: Vec::with_capacity((MEMORY_MAX as usize) + 1),
            registers: vec![0, 0, 0, 0, 0, 0, 0, 0],
            pc: 0x3000,
            conditional: Flag::Zer,
        }
    }

    pub fn cleanup(self: &mut Self) {
        self.memory.clear();
        self.registers.clear();
        self.pc = 0;
        self.conditional = Flag::Zer;
    }
}

