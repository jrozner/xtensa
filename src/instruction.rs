use crate::decode::{
    extract_m, extract_n, extract_op0, extract_op1, extract_op2, extract_r, extract_rs, extract_s,
    extract_t,
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
    Addi(GenericInstruction),
    Addmi(GenericInstruction),
    Ball(GenericInstruction),
    Bany(GenericInstruction),
    Bbc(GenericInstruction),
    Bbci(GenericInstruction),
    Bbs(GenericInstruction),
    Bbsi(GenericInstruction),
    Beq(GenericInstruction),
    Beqi(GenericInstruction),
    Bge(GenericInstruction),
    Bgei(GenericInstruction),
    Bgeu(GenericInstruction),
    Bgeui(GenericInstruction),
    Blt(GenericInstruction),
    Blti(GenericInstruction),
    Bltu(GenericInstruction),
    Bltui(GenericInstruction),
    Bnall(GenericInstruction),
    Bne(GenericInstruction),
    Bnei(GenericInstruction),
    Bnone(GenericInstruction),
    L8ui(GenericInstruction),
    L16si(GenericInstruction),
    L16ui(GenericInstruction),
    L32i(GenericInstruction),
    Movi(GenericInstruction),
    S8i(GenericInstruction),
    S16i(GenericInstruction),
    S32i(GenericInstruction),
    Beqz(GenericInstruction),
    Bgez(GenericInstruction),
    Bltz(GenericInstruction),
    Bnez(GenericInstruction),
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
pub struct Rri8 {}

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
pub struct Bri12 {}

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
