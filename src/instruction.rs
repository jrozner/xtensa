use crate::decode::{
    extract_m, extract_n, extract_op0, extract_op1, extract_op2, extract_r, extract_rs, extract_s,
    extract_t, extract_imm12
};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Abs(Rrr),
    Add(Rrr),
    Addx2(Rrr),
    Addx4(Rrr),
    Addx8(Rrr),
    And(Rrr),
    Dsync(Rrr),
    Esync(Rrr),
    Extui(Rrr),
    Extw(Rrr),
    Isync(Rrr),
    Memw(Rrr),
    Moveqz(Rrr),
    Movgez(Rrr),
    Movltz(Rrr),
    Neg(Rrr),
    Nop(Rrr),
    Or(Rrr),
    Rsync(Rrr),
    Sll(Rrr),
    Slli(Rrr),
    Sra(Rrr),
    Srai(Rrr),
    Src(Rrr),
    Srl(Rrr),
    Srli(Rrr),
    Ssa8l(Rrr),
    Ssai(Rrr),
    Ssl(Rrr),
    Ssr(Rrr),
    Sub(Rrr),
    Subx2(Rrr),
    Subx4(Rrr),
    Subx8(Rrr),
    Xor(Rrr),
    Xsr(Rrr),
    Addi(Rri8),
    Addmi(Rri8),
    Ball(Rri8),
    Bany(Rri8),
    Bbc(Rri8),
    Bbci(Rri8),
    Bbs(Rri8),
    Bbsi(Rri8),
    Beq(Rri8),
    Beqi(Rri8), // TODO: this is actually BRI8
    Bge(Rri8),
    Bgei(Rri8), // TODO: this is actually BRI8
    Bgeu(Rri8),
    Bgeui(Rri8), // TODO: this is actually BRI8
    Blt(Rri8),
    Blti(Rri8), // TODO: this is actually BRI8
    Bltu(Rri8),
    Bltui(Rri8), // TODO: this is actually BRI8
    Bnall(Rri8),
    Bne(Rri8),
    Bnei(Rri8), // TODO: this is actually BRI8
    Bnone(Rri8),
    L8ui(Rri8),
    L16si(Rri8),
    L16ui(Rri8),
    L32i(Rri8),
    Movi(Rri8),
    S8i(Rri8),
    S16i(Rri8),
    S32i(Rri8),
    Beqz(Bri12),
    Bgez(Bri12),
    Bltz(Bri12),
    Bnez(Bri12),
    Call0(GenericInstruction),
    J(GenericInstruction),
    Callx0(GenericInstruction),
    Jx(GenericInstruction),
    Ret(GenericInstruction),
    Rsr(GenericInstruction),
    Wsr(GenericInstruction),
    L32r(GenericInstruction),
    Movsp(GenericInstruction),
    Rotw(GenericInstruction),
    Rfwo(GenericInstruction),
    Rfwu(GenericInstruction),
    Call4(GenericInstruction),
    Call8(GenericInstruction),
    Call12(GenericInstruction),
    Callx4(GenericInstruction),
    Callx8(GenericInstruction),
    Callx12(GenericInstruction),
    Retw(GenericInstruction),
    Entry(GenericInstruction),
    L32e(GenericInstruction),
    S32e(GenericInstruction),
    Addn(GenericInstruction),
    Addin(GenericInstruction),
    Illn(GenericInstruction),
    L32in(GenericInstruction),
    Movn(GenericInstruction),
    Nopn(GenericInstruction),
    Retn(GenericInstruction),
    Retwn(GenericInstruction),
    S32in(GenericInstruction),
    Beqzn(GenericInstruction),
    Bnezn(GenericInstruction),
    Movin(GenericInstruction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rrr {
    r: u32,
    s: u32,
    t: u32,
}

impl Rrr {
    pub fn new(r: u32, s: u32, t: u32) -> Rrr {
        Rrr { r, s, t }
    }

    pub fn r(&self) -> u32 {
        self.r
    }

    pub fn s(&self) -> u32 {
        self.s
    }

    pub fn t(&self) -> u32 {
        self.t
    }
}

impl From<u32> for Rrr {
    fn from(value: u32) -> Self {
        let r = extract_r(value);
        let s = extract_s(value);
        let t = extract_t(value);

        Rrr { r, s, t }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rri4 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Rri8 {
    s: u32,
    t: u32,
    imm8: u8,
}

impl Rri8 {
    pub fn new(s: u32, t: u32, imm8: u8) -> Rri8 {
        Rri8 { s, t, imm8 }
    }

    pub fn s(&self) -> u32 {
        self.s
    }

    pub fn t(&self) -> u32 {
        self.t
    }

    pub fn imm8(&self) -> u8 {
        self.imm8
    }
}

impl From<u32> for Rri8 {
    fn from(value: u32) -> Self {
        let s = extract_s(value);
        let t = extract_t(value);
        let imm8 = extract_op2(value) as u8;

        Rri8 { s, t, imm8 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ri16 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Rsr {}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {}

#[derive(Debug, Clone, PartialEq)]
pub struct Callx {}

#[derive(Debug, Clone, PartialEq)]
pub struct Bri8 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Bri12 {
    s: u32,
    imm12: u16,
}

impl Bri12 {
    pub fn new(s: u32, imm12: u16) -> Bri12 {
        Bri12{
            s,
            imm12,
        }
    }

    pub fn s(&self) -> u32 {
        self.s
    }

    pub fn imm12(&self) -> u16 {
        self.imm12
    }
}

impl From<u32> for Bri12 {
    fn from(value: u32) -> Self {
        let s = extract_s(value);
        let imm12 = extract_imm12(value);

        Bri12 { s, imm12 }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rrrn {}

#[derive(Debug, Clone, PartialEq)]
pub struct Ri7 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Ri6 {}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericInstruction {
    inst: String,
    source: Option<u32>,
    target: Option<u32>,
    result: Option<u32>,
}
