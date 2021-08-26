use byteorder::{ByteOrder, LittleEndian};

const OP0_MASK: u32 = 0b1111;
const OP1_MASK: u32 = 0b1111_0000_0000_0000_0000;
const OP2_MASK: u32 = 0b1111_0000_0000_0000_0000_0000;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {}

#[inline]
fn extract_op0(value: u32) -> u32 {
    value & 0b1111
}

#[inline]
fn extract_op1(value: u32) -> u32 {
    (value & 0xf0000) >> 16
}

#[inline]
fn extract_op2(value: u32) -> u32 {
    (value & 0xf00000) >> 20
}

#[inline]
fn extract_n(value: u32) -> u32 {
    (value & 0x30) >> 4
}

#[inline]
fn extract_m(value: u32) -> u32 {
    (value & 0xc0) >> 6
}

#[inline]
fn extract_r(value: u32) -> u32 {
    (value & 0xf000) >> 12
}

#[inline]
fn extract_s(value: u32) -> u32 {
    (value & 0xf00) >> 8
}

#[inline]
fn extract_t(value: u32) -> u32 {
    (value & 0xf0) >> 4
}

#[inline]
fn extract_rs(value: u32) -> u32 {
    (value & 0xff00) >> 8
}

pub fn decode(input: &[u8]) -> Option<Instruction> {
    if input.len() < 3 {
        return None;
    }

    let value = LittleEndian::read_u24(&input);

    let op0 = extract_op0(value);

    match op0 {
        0b0000 => parse_qrst(value),
        0b0001 => unimplemented!(),   // l32r
        0b0010 => parse_lsai(value),    // lsai
        0b0011 => parse_lsci(value),  // lsci
        0b0100 => parse_mac16(value), // mac16
        0b0101 => unimplemented!(),   // calln
        0b0110 => unimplemented!(),   // si
        0b0111 => unimplemented!(),   // b
        0b1000 => unimplemented!(),   // l32i.n
        0b1001 => unimplemented!(),   // s32i.n
        0b1010 => unimplemented!(),   // add.n
        0b1011 => unimplemented!(),   // addi.n
        0b1100 => unimplemented!(),   // st2
        0b1101 => unimplemented!(),   // st3
        _ => None,
    }
}

fn parse_qrst(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => parse_rst0(value),          // rst0
        0b0001 => unimplemented!(),          // rst1
        0b0010 => unimplemented!(),          // rst2
        0b0011 => unimplemented!(),          // rst3
        0b0100 | 0b0101 => unimplemented!(), // extui
        0b0110 => unimplemented!(),          // cust0
        0b0111 => unimplemented!(),          // cust1
        0b1000 => unimplemented!(),          // lscx
        0b1001 => unimplemented!(),          // lsc4
        0b1010 => unimplemented!(),          // fp0
        0b1011 => unimplemented!(),          // fp1
        _ => None,
    }
}

fn parse_rst0(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => parse_st0(value), // st0
        0b0001 => unimplemented!(), // and
        0b0010 => unimplemented!(), // or
        0b0011 => unimplemented!(), // xor
        0b0100 => parse_st1(value), // st1
        0b0101 => parse_tlb(value), // tlb
        0b0110 => parse_rt0(value), // rt0
        0b1000 => unimplemented!(), // add
        0b1001 => unimplemented!(), // addx2
        0b1010 => unimplemented!(), // addx4
        0b1011 => unimplemented!(), // addx8
        0b1100 => unimplemented!(), // sub
        0b1101 => unimplemented!(), // subx2
        0b1110 => unimplemented!(), // subx4
        0b1111 => unimplemented!(), // subx8
        _ => None,
    }
}

fn parse_st0(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => parse_snm0(value), // snm0
        0b0001 => unimplemented!(), // movsp
        0b0010 => parse_sync(value), // sync
        0b0011 => parse_rfei(value), // rfei
        0b0100 => unimplemented!(), // break
        0b0101 => unimplemented!(), // syscall
        0b0110 => unimplemented!(), // rsil
        0b0111 => unimplemented!(), // waiti
        0b1000 => unimplemented!(), // any4
        0b1001 => unimplemented!(), // all4
        0b1010 => unimplemented!(), // any8
        0b1011 => unimplemented!(), // all8
        _ => None,
    }
}

fn parse_snm0(value: u32) -> Option<Instruction> {
    let m = extract_m(value);

    match m {
        0b00 => unimplemented!(), // ill
        0b10 => parse_jr(value), // jr
        0b11 => parse_callx(value), // callx
        _ => None,
    }
}

fn parse_jr(value: u32) -> Option<Instruction> {
    let n = extract_n(value);

    match n {
        0b00 => unimplemented!(), // ret
        0b01 => unimplemented!(), // retw
        0b10 => unimplemented!(), // jx
        _ => None,
    }
}

fn parse_callx(value: u32) -> Option<Instruction> {
    let n = extract_n(value);

    match n {
        0b00 => unimplemented!(), // callx
        0b01 => unimplemented!(), // callx4
        0b10 => unimplemented!(), // callx8
        0b11 => unimplemented!(), // callx12
        _ => None,
    }
}

fn parse_sync(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => unimplemented!(), // isync
        0b0001 => unimplemented!(), // rsync
        0b0010 => unimplemented!(), // esync
        0b0011 => unimplemented!(), // dsync
        0b1000 => unimplemented!(), // excw
        0b1100 => unimplemented!(), // memw
        0b1101 => unimplemented!(), // extw
        _ => None,
    }
}

fn parse_rfei(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => parse_rfet(value), // rfet
        0b0001 => unimplemented!(), // rfi
        0b0010 => unimplemented!(), // rfme
        _ => None,
    }
}

fn parse_rfet(value: u32) -> Option<Instruction> {
    let s = extract_s(value);

    match s {
        0b0000 => unimplemented!(), // rfe
        0b0001 => unimplemented!(), // rfue
        0b0010 => unimplemented!(), // rfde
        0b0100 => unimplemented!(), // rfwo
        0b0101 => unimplemented!(), // rfwu
        _ => None,
    }
}

fn parse_st1(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(), // ssr
        0b0001 => unimplemented!(), // ssl
        0b0010 => unimplemented!(), // ssa8l
        0b0011 => unimplemented!(), // ssa8b
        0b0100 => unimplemented!(), // ssai
        0b0110 => unimplemented!(), // rer
        0b0111 => unimplemented!(), // wer
        0b1000 => unimplemented!(), // rotw
        0b1110 => unimplemented!(), // nsa
        0b1111 => unimplemented!(), // nsau
        _ => None,
    }
}

fn parse_tlb(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0011 => unimplemented!(), // ritlb0
        0b0100 => unimplemented!(), // iitlb
        0b0101 => unimplemented!(), // pitlb
        0b0110 => unimplemented!(), // witlb
        0b0111 => unimplemented!(), // tilb1
        0b1011 => unimplemented!(), // rdtlb0
        0b1100 => unimplemented!(), // idtlb
        0b1101 => unimplemented!(), // pdtlb
        0b1110 => unimplemented!(), // tdtlb
        0b1111 => unimplemented!(), // rdtlb1
        _ => None,
    }
}

fn parse_rt0(value: u32) -> Option<Instruction> {
    let s = extract_s(value);

    match s {
        0b0000 => unimplemented!(), // neg
        0b0001 => unimplemented!(), // abs
        _ => None,
    }
}

// CONTINUE rst1
fn parse_lsai(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(), // l8ui
        0b0001 => unimplemented!(), // l16ui
        0b0010 => unimplemented!(), // l32i
        0b0100 => unimplemented!(), // s8i
        0b0101 => unimplemented!(), // s16i
        0b0110 => unimplemented!(), // s32i
        0b0111 => unimplemented!(), // cache
        0b1001 => unimplemented!(), // l16si
        0b1010 => unimplemented!(), // movi
        0b1011 => unimplemented!(), // l32ai
        0b1100 => unimplemented!(), // addi
        0b1101 => unimplemented!(), // addmi
        0b1110 => unimplemented!(), // s32c1i
        0b1111 => unimplemented!(), // s32rl
        _ => None,
    }
}

fn parse_lsci(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(), // lsi
        0b0100 => unimplemented!(), // ssi
        0b1000 => unimplemented!(), // lsiu
        0b1100 => unimplemented!(), // ssiu
        _ => None,
    }
}

fn parse_mac16(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // macid
        0b0001 => unimplemented!(), // maccd
        0b0010 => unimplemented!(), // macdd
        0b0011 => unimplemented!(), // macad
        0b0100 => unimplemented!(), // macia
        0b0101 => unimplemented!(), // macca
        0b0110 => unimplemented!(), // macda
        0b0111 => unimplemented!(), // macaa
        0b1000 => unimplemented!(), // macci
        0b1001 => unimplemented!(), // macc
        _ => None,
    }
}

fn parse_(value: u32) -> Option<Instruction> {
    let op0 = extract_(value);

    match op0 {
        0b0000 => unimplemented!(), //
        0b0001 => unimplemented!(), //
        0b0010 => unimplemented!(), //
        0b0011 => unimplemented!(), //
        0b0100 => unimplemented!(), //
        0b0101 => unimplemented!(), //
        0b0110 => unimplemented!(), //
        0b0111 => unimplemented!(), //
        0b1000 => unimplemented!(), //
        0b1001 => unimplemented!(), //
        0b1010 => unimplemented!(), //
        0b1011 => unimplemented!(), //
        0b1100 => unimplemented!(), //
        0b1101 => unimplemented!(), //
        0b1110 => unimplemented!(), //
        0b1111 => unimplemented!(), //
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, Instruction};

    #[test]
    fn it_works() {
        let data = [0x42, 0x43, 0x44, 0x45];
        let decoded = decode(&data);
        assert!(decoded == None);
    }

    #[test]
    fn it_doesnt_work() {
        let data = [0x42, 0x43];
        let decoded = decode(&data);
        assert!(decoded == None);
    }

    #[test]
    fn j() {
        let data = [0x06, 0x01, 0x00];
        let decoded = decode(&data);
        assert!(decoded == Some(Instruction::J(4)));
    }
}
