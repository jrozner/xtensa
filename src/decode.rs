use byteorder::{ByteOrder, LittleEndian};

use crate::instruction::{Instruction, Rrr};

#[inline]
pub(crate) fn extract_op0(value: u8) -> u8 {
    value & 0b1111
}

#[inline]
pub(crate) fn extract_op1(value: u32) -> u32 {
    (value & 0xf0000) >> 16
}

#[inline]
pub(crate) fn extract_op2(value: u32) -> u32 {
    (value & 0xf00000) >> 20
}

#[inline]
pub(crate) fn extract_n(value: u32) -> u32 {
    (value & 0x30) >> 4
}

#[inline]
pub(crate) fn extract_m(value: u32) -> u32 {
    (value & 0xc0) >> 6
}

#[inline]
pub(crate) fn extract_r(value: u32) -> u32 {
    (value & 0xf000) >> 12
}

#[inline]
pub(crate) fn extract_s(value: u32) -> u32 {
    (value & 0xf00) >> 8
}

#[inline]
pub(crate) fn extract_t(value: u32) -> u32 {
    (value & 0xf0) >> 4
}

#[inline]
pub(crate) fn extract_rs(value: u32) -> u32 {
    (value & 0xff00) >> 8
}

pub fn decode(input: &[u8]) -> Option<Instruction> {
    if input.len() < 1 {
        return None;
    }

    let op0 = extract_op0(input[0]);

    // need to decide if we're looking at a normal or narrow (16bit) instruction. All narrow
    // instructions have the high bit of op0 set.
    let value = if (op0 & 0x8) > 0 {
        if input.len() < 2 {
            return None;
        }

        LittleEndian::read_u16(&input) as u32
    } else {
        if input.len() < 3 {
            return None;
        }

        LittleEndian::read_u24(&input)
    };

    match op0 {
        0b0000 => parse_qrst(value),
        0b0001 => unimplemented!(),   // l32r
        0b0010 => parse_lsai(value),  // lsai
        0b0011 => parse_lsci(value),  // lsci
        0b0100 => parse_mac16(value), // mac16
        0b0101 => parse_calln(value), // calln
        0b0110 => parse_si(value),    // si
        0b0111 => parse_b(value),     // b
        0b1000 => unimplemented!(),   // l32i.n
        0b1001 => unimplemented!(),   // s32i.n
        0b1010 => unimplemented!(),   // add.n
        0b1011 => unimplemented!(),   // addi.n
        0b1100 => parse_st2(value),   // st2
        0b1101 => parse_st3(value),   // st3
        _ => None,
    }
}

fn parse_qrst(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => parse_rst0(value),         // rst0
        0b0001 => parse_rst1(value),         // rst1
        0b0010 => parse_rst2(value),         // rst2
        0b0011 => parse_rst3(value),         // rst3
        0b0100 | 0b0101 => unimplemented!(), // extui
        0b0110 => unimplemented!(),          // cust0
        0b0111 => unimplemented!(),          // cust1
        0b1000 => parse_lscx(value),         // lscx
        0b1001 => parse_lsc4(value),         // lsc4
        0b1010 => parse_fp0(value),          // fp0
        0b1011 => parse_fp1(value),          // fp1
        _ => None,
    }
}

fn parse_rst0(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => parse_st0(value),                           // st0
        0b0001 => Some(Instruction::And(Rrr::from(value))),   // and
        0b0010 => Some(Instruction::Or(Rrr::from(value))),    // or
        0b0011 => Some(Instruction::Xor(Rrr::from(value))),   // xor
        0b0100 => parse_st1(value),                           // st1
        0b0101 => parse_tlb(value),                           // tlb
        0b0110 => parse_rt0(value),                           // rt0
        0b1000 => Some(Instruction::Add(Rrr::from(value))),   // add
        0b1001 => Some(Instruction::Addx2(Rrr::from(value))), // addx2
        0b1010 => Some(Instruction::Addx4(Rrr::from(value))), // addx4
        0b1011 => Some(Instruction::Addx8(Rrr::from(value))), // addx8
        0b1100 => Some(Instruction::Sub(Rrr::from(value))),   // sub
        0b1101 => Some(Instruction::Subx2(Rrr::from(value))), // subx2
        0b1110 => Some(Instruction::Subx4(Rrr::from(value))), // subx4
        0b1111 => Some(Instruction::Subx8(Rrr::from(value))), // subx8
        _ => None,
    }
}

fn parse_st0(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => parse_snm0(value), // snm0
        0b0001 => unimplemented!(),  // movsp
        0b0010 => parse_sync(value), // sync
        0b0011 => parse_rfei(value), // rfei
        0b0100 => unimplemented!(),  // break
        0b0101 => unimplemented!(),  // syscall
        0b0110 => unimplemented!(),  // rsil
        0b0111 => unimplemented!(),  // waiti
        0b1000 => unimplemented!(),  // any4
        0b1001 => unimplemented!(),  // all4
        0b1010 => unimplemented!(),  // any8
        0b1011 => unimplemented!(),  // all8
        _ => None,
    }
}

fn parse_snm0(value: u32) -> Option<Instruction> {
    let m = extract_m(value);

    match m {
        0b00 => unimplemented!(),   // ill
        0b10 => parse_jr(value),    // jr
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
        0b0000 => Some(Instruction::Isync(Rrr::from(value))), // isync
        0b0001 => Some(Instruction::Rsync(Rrr::from(value))), // rsync
        0b0010 => Some(Instruction::Esync(Rrr::from(value))), // esync
        0b0011 => Some(Instruction::Dsync(Rrr::from(value))), // dsync
        0b1000 => unimplemented!(),                           // excw
        0b1100 => Some(Instruction::Memw(Rrr::from(value))),  // memw
        0b1101 => Some(Instruction::Extw(Rrr::from(value))),  // extw
        //TODO 1111 is NOP?
        _ => None,
    }
}

fn parse_rfei(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => parse_rfet(value), // rfet
        0b0001 => unimplemented!(),  // rfi
        0b0010 => unimplemented!(),  // rfme
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
        0b0000 => Some(Instruction::Ssr(Rrr::from(value))), // ssr
        0b0001 => Some(Instruction::Ssl(Rrr::from(value))), // ssl
        0b0010 => Some(Instruction::Ssa8l(Rrr::from(value))), // ssa8l
        0b0011 => unimplemented!(), // ssa8b
        0b0100 => Some(Instruction::Ssai(Rrr::from(value))), // ssai
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
        0b0000 => Some(Instruction::Neg(Rrr::from(value))), // neg
        0b0001 => Some(Instruction::Abs(Rrr::from(value))), // abs
        _ => None,
    }
}

fn parse_rst1(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 | 0b0001 => Some(Instruction::Slli(Rrr::from(value))), // slli
        0b0010 | 0b0011 => Some(Instruction::Srai(Rrr::from(value))), // srai
        0b0100 => Some(Instruction::Srli(Rrr::from(value))),          // srli
        0b0110 => Some(Instruction::Xsr(Rrr::from(value))),          // xsr
        0b0111 => parse_accer(value),        // accer
        0b1000 => Some(Instruction::Src(Rrr::from(value))), // src
        0b1001 => Some(Instruction::Srl(Rrr::from(value))),          // srl
        0b1010 => Some(Instruction::Sll(Rrr::from(value))), // sll
        0b1011 => Some(Instruction::Sra(Rrr::from(value))), // sra
        0b1100 => unimplemented!(),          // mul16u
        0b1101 => unimplemented!(),          // mul16s
        0b1111 => parse_imp(value),          // imp
        _ => None,
    }
}

fn parse_accer(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // rer
        0b1000 => unimplemented!(), // wer
        _ => None,
    }
}

fn parse_imp(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(),  // lict
        0b0001 => unimplemented!(),  // sict
        0b0010 => unimplemented!(),  // licw
        0b0011 => unimplemented!(),  // sicw
        0b1000 => unimplemented!(),  // ldct
        0b1001 => unimplemented!(),  // sdct
        0b1110 => parse_rfdx(value), // rfdx
        _ => None,
    }
}

fn parse_rfdx(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => unimplemented!(), // rfdo
        0b0001 => unimplemented!(), // rfdd
        _ => None,
    }
}

fn parse_rst2(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // andb
        0b0001 => unimplemented!(), // andbc
        0b0010 => unimplemented!(), // orb
        0b0011 => unimplemented!(), // orbc
        0b0100 => unimplemented!(), // xorb
        0b1000 => unimplemented!(), // mull
        0b1010 => unimplemented!(), // muluh
        0b1011 => unimplemented!(), // mulsh
        0b1100 => unimplemented!(), // quou
        0b1101 => unimplemented!(), // quos
        0b1110 => unimplemented!(), // remu
        0b1111 => unimplemented!(), // rems
        _ => None,
    }
}

fn parse_rst3(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // rsr
        0b0001 => unimplemented!(), // wsr
        0b0010 => unimplemented!(), // sext
        0b0011 => unimplemented!(), // clamps
        0b0100 => unimplemented!(), // min
        0b0101 => unimplemented!(), // max
        0b0110 => unimplemented!(), // minu
        0b0111 => unimplemented!(), // maxu
        0b1000 => Some(Instruction::Moveqz(Rrr::from(value))), // moveqz
        0b1001 => unimplemented!(), // movnez
        0b1010 => Some(Instruction::Movltz(Rrr::from(value))), // movltz
        0b1011 => Some(Instruction::Movgez(Rrr::from(value))), // movgez
        0b1100 => unimplemented!(), // movf
        0b1101 => unimplemented!(), // movt
        0b1110 => unimplemented!(), // rur
        0b1111 => unimplemented!(), // wur
        _ => None,
    }
}

fn parse_lscx(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // lsx
        0b0001 => unimplemented!(), // lsxu
        0b0100 => unimplemented!(), // ssx
        0b0101 => unimplemented!(), // ssxu
        _ => None,
    }
}

fn parse_lsc4(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(), // l32e
        0b0100 => unimplemented!(), // s32e
        _ => None,
    }
}

fn parse_fp0(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0000 => unimplemented!(),   // add.s
        0b0001 => unimplemented!(),   // sub.s
        0b0010 => unimplemented!(),   // mul.s
        0b0100 => unimplemented!(),   // madd.s
        0b0101 => unimplemented!(),   // msub.s
        0b1000 => unimplemented!(),   // round.s
        0b1001 => unimplemented!(),   // trunc.s
        0b1010 => unimplemented!(),   // floor.s
        0b1011 => unimplemented!(),   // ceil.s
        0b1100 => unimplemented!(),   // float.s
        0b1101 => unimplemented!(),   // ufloat.s
        0b1110 => unimplemented!(),   // utrunc.s
        0b1111 => parse_fp1op(value), // fp1op
        _ => None,
    }
}

fn parse_fp1op(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => unimplemented!(), // mov.s
        0b0001 => unimplemented!(), // abs.s
        0b0100 => unimplemented!(), // rfr
        0b0101 => unimplemented!(), // wfr
        0b0110 => unimplemented!(), // neg.s
        _ => None,
    }
}

fn parse_fp1(value: u32) -> Option<Instruction> {
    let op2 = extract_op2(value);

    match op2 {
        0b0001 => unimplemented!(), // un.s
        0b0010 => unimplemented!(), // oeq.s
        0b0011 => unimplemented!(), // ueq.s
        0b0100 => unimplemented!(), // olt.s
        0b0101 => unimplemented!(), // ult.s
        0b0110 => unimplemented!(), // ole.s
        0b0111 => unimplemented!(), // ule.s
        0b1000 => unimplemented!(), // moveqz.s
        0b1001 => unimplemented!(), // movnez.s
        0b1010 => unimplemented!(), // movltz.s
        0b1011 => unimplemented!(), // movgez.s
        0b1100 => unimplemented!(), // movf.s
        0b1101 => unimplemented!(), // movt.s
        _ => None,
    }
}

fn parse_lsai(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(),   // l8ui
        0b0001 => unimplemented!(),   // l16ui
        0b0010 => unimplemented!(),   // l32i
        0b0100 => unimplemented!(),   // s8i
        0b0101 => unimplemented!(),   // s16i
        0b0110 => unimplemented!(),   // s32i
        0b0111 => parse_cache(value), // cache
        0b1001 => unimplemented!(),   // l16si
        0b1010 => unimplemented!(),   // movi
        0b1011 => unimplemented!(),   // l32ai
        0b1100 => unimplemented!(),   // addi
        0b1101 => unimplemented!(),   // addmi
        0b1110 => unimplemented!(),   // s32c1i
        0b1111 => unimplemented!(),   // s32ri
        _ => None,
    }
}

fn parse_cache(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => unimplemented!(), // dpfr
        0b0001 => unimplemented!(), // fpfw
        0b0010 => unimplemented!(), // fpfro
        0b0011 => unimplemented!(), // fpfwo
        0b0100 => unimplemented!(), // dhwb
        0b0101 => unimplemented!(), // dhwbi
        0b0110 => unimplemented!(), // dhi
        0b0111 => unimplemented!(), // dii
        0b1000 => parse_dce(value), // dce
        0b1100 => unimplemented!(), // ipf
        0b1101 => parse_ice(value), // ice
        0b1110 => unimplemented!(), // ihi
        0b1111 => unimplemented!(), // iii
        _ => None,
    }
}

fn parse_dce(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => unimplemented!(), // dpfl
        0b0010 => unimplemented!(), // dhu
        0b0011 => unimplemented!(), // diu
        0b0100 => unimplemented!(), // diwb
        0b0101 => unimplemented!(), // diwbi
        _ => None,
    }
}

fn parse_ice(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => unimplemented!(), // ipfl
        0b0010 => unimplemented!(), // ihu
        0b0011 => unimplemented!(), // iiu
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
        0b0000 => parse_macid(value), // macid
        0b0001 => parse_maccd(value), // maccd
        0b0010 => parse_macdd(value), // macdd
        0b0011 => parse_macad(value), // macad
        0b0100 => parse_macia(value), // macia
        0b0101 => parse_macca(value), // macca
        0b0110 => parse_macda(value), // macda
        0b0111 => parse_macaa(value), // macaa
        0b1000 => parse_maci(value),  // maci
        0b1001 => parse_macc(value),  // macc
        _ => None,
    }
}

fn parse_macid(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b1000 => unimplemented!(), // mula.dd.ll.ldinc
        0b1001 => unimplemented!(), // mula.dd.hl.ldinc
        0b1010 => unimplemented!(), // mula.dd.lh.ldinc
        0b1011 => unimplemented!(), // mula.dd.hh.ldinc
        _ => None,
    }
}

fn parse_macia(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b1000 => unimplemented!(), // mula.da.ll.ldinc
        0b1001 => unimplemented!(), // mula.da.hl.ldinc
        0b1010 => unimplemented!(), // mula.da.lh.ldinc
        0b1011 => unimplemented!(), // mula.da.hh.ldinc
        _ => None,
    }
}

fn parse_macdd(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0100 => unimplemented!(), // mul.dd.ll
        0b0101 => unimplemented!(), // mul.dd.hl
        0b0110 => unimplemented!(), // mul.dd.lh
        0b0111 => unimplemented!(), // mul.dd.hh
        0b1000 => unimplemented!(), // mula.dd.ll
        0b1001 => unimplemented!(), // mula.dd.hl
        0b1010 => unimplemented!(), // mula.dd.lh
        0b1011 => unimplemented!(), // mula.dd.hh
        0b1100 => unimplemented!(), // muls.dd.ll
        0b1101 => unimplemented!(), // muls.dd.hl
        0b1110 => unimplemented!(), // muls.dd.lh
        0b1111 => unimplemented!(), // muls.dd.hh
        _ => None,
    }
}

fn parse_macad(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0100 => unimplemented!(), // mul.ad.ll
        0b0101 => unimplemented!(), // mul.ad.hl
        0b0110 => unimplemented!(), // mul.ad.lh
        0b0111 => unimplemented!(), // mul.ad.hh
        0b1000 => unimplemented!(), // mula.ad.ll
        0b1001 => unimplemented!(), // mula.ad.hl
        0b1010 => unimplemented!(), // mula.ad.lh
        0b1011 => unimplemented!(), // mula.ad.hh
        0b1100 => unimplemented!(), // muls.ad.ll
        0b1101 => unimplemented!(), // muls.ad.hl
        0b1110 => unimplemented!(), // muls.ad.lh
        0b1111 => unimplemented!(), // muls.ad.hh
        _ => None,
    }
}

fn parse_maccd(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b1000 => unimplemented!(), // mula.dd.ll.lddec
        0b1001 => unimplemented!(), // mula.dd.hl.lddec
        0b1010 => unimplemented!(), // mula.dd.lh.lddec
        0b1011 => unimplemented!(), // mula.dd.hh.lddec
        _ => None,
    }
}

fn parse_macca(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b1000 => unimplemented!(), // mula.da.ll.lddec
        0b1001 => unimplemented!(), // mula.da.hl.lddec
        0b1010 => unimplemented!(), // mula.da.lh.lddec
        0b1011 => unimplemented!(), // mula.da.hh.lddec
        _ => None,
    }
}

fn parse_macda(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0100 => unimplemented!(), // mul.da.ll
        0b0101 => unimplemented!(), // mul.da.hl
        0b0110 => unimplemented!(), // mul.da.lh
        0b0111 => unimplemented!(), // mul.da.hh
        0b1000 => unimplemented!(), // mula.da.ll
        0b1001 => unimplemented!(), // mula.da.hl
        0b1010 => unimplemented!(), // mula.da.lh
        0b1011 => unimplemented!(), // mula.da.hh
        0b1100 => unimplemented!(), // muls.da.ll
        0b1101 => unimplemented!(), // muls.da.hl
        0b1110 => unimplemented!(), // muls.da.lh
        0b1111 => unimplemented!(), // muls.da.hh
        _ => None,
    }
}

fn parse_macaa(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => unimplemented!(), // umul.aa.ll
        0b0001 => unimplemented!(), // umul.aa.hl
        0b0010 => unimplemented!(), // umul.aa.lh
        0b0011 => unimplemented!(), // umul.aa.hh
        0b0100 => unimplemented!(), // mul.aa.ll
        0b0101 => unimplemented!(), // mul.aa.hl
        0b0110 => unimplemented!(), // mul.aa.lh
        0b0111 => unimplemented!(), // mul.aa.hh
        0b1000 => unimplemented!(), // mula.aa.ll
        0b1001 => unimplemented!(), // mula.aa.hl
        0b1010 => unimplemented!(), // mula.aa.lh
        0b1011 => unimplemented!(), // mula.aa.hh
        0b1100 => unimplemented!(), // muls.aa.ll
        0b1101 => unimplemented!(), // muls.aa.hl
        0b1110 => unimplemented!(), // muls.aa.lh
        0b1111 => unimplemented!(), // muls.aa.hh
        _ => None,
    }
}

fn parse_maci(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => unimplemented!(), // ldinc
        _ => None,
    }
}

fn parse_macc(value: u32) -> Option<Instruction> {
    let op1 = extract_op1(value);

    match op1 {
        0b0000 => unimplemented!(), // lddec
        _ => None,
    }
}

fn parse_calln(value: u32) -> Option<Instruction> {
    let n = extract_n(value);

    match n {
        0b00 => unimplemented!(), // call0
        0b01 => unimplemented!(), // call4
        0b10 => unimplemented!(), // call8
        0b11 => unimplemented!(), // call12
        _ => None,
    }
}

fn parse_si(value: u32) -> Option<Instruction> {
    let n = extract_n(value);

    match n {
        0b00 => unimplemented!(), // j
        0b01 => parse_bz(value),  // bz
        0b10 => parse_bi0(value), // bi0
        0b11 => parse_bi1(value), // bi1
        _ => None,
    }
}

fn parse_bz(value: u32) -> Option<Instruction> {
    let m = extract_m(value);

    match m {
        0b00 => unimplemented!(), // beqz
        0b01 => unimplemented!(), // bnez
        0b10 => unimplemented!(), // bltz
        0b11 => unimplemented!(), // bgez
        _ => None,
    }
}

fn parse_bi0(value: u32) -> Option<Instruction> {
    let m = extract_m(value);

    match m {
        0b00 => unimplemented!(), // beqi
        0b01 => unimplemented!(), // bnei
        0b10 => unimplemented!(), // blti
        0b11 => unimplemented!(), // bgei
        _ => None,
    }
}

fn parse_bi1(value: u32) -> Option<Instruction> {
    let m = extract_m(value);

    match m {
        0b00 => unimplemented!(), // entry
        0b01 => parse_b1(value),  // b1
        0b10 => unimplemented!(), // bltui
        0b11 => unimplemented!(), // bgeui
        _ => None,
    }
}

fn parse_b1(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(), // bf
        0b0001 => unimplemented!(), // bt
        0b1000 => unimplemented!(), // loop
        0b1001 => unimplemented!(), // loopnez
        0b1010 => unimplemented!(), // loopgtz
        _ => None,
    }
}

fn parse_b(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(),          // bnone
        0b0001 => unimplemented!(),          // beq
        0b0010 => unimplemented!(),          // blt
        0b0011 => unimplemented!(),          // bltu
        0b0100 => unimplemented!(),          // ball
        0b0101 => unimplemented!(),          // bbc
        0b0110 | 0b111 => unimplemented!(),  // bbci
        0b1000 => unimplemented!(),          // bany
        0b1001 => unimplemented!(),          // bne
        0b1010 => unimplemented!(),          // bge
        0b1011 => unimplemented!(),          // bgeu
        0b1100 => unimplemented!(),          // bnall
        0b1101 => unimplemented!(),          // bbs
        0b1110 | 0b1111 => unimplemented!(), // bbsi
        _ => None,
    }
}

fn parse_st2(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 | 0b0001 | 0b0010 | 0b0011 | 0b0100 | 0b0101 | 0b0110 | 0b0111 => unimplemented!(), // movi.n
        0b1000 | 0b1001 | 0b1010 | 0b1011 => unimplemented!(), // beqz.n
        0b1100 | 0b1101 | 0b1110 | 0b1111 => unimplemented!(), // bnez.n
        _ => None,
    }
}

fn parse_st3(value: u32) -> Option<Instruction> {
    let r = extract_r(value);

    match r {
        0b0000 => unimplemented!(), // mov.n
        0b1111 => parse_s3(value),  // s3
        _ => None,
    }
}

fn parse_s3(value: u32) -> Option<Instruction> {
    let t = extract_t(value);

    match t {
        0b0000 => unimplemented!(), // ret.n
        0b0001 => unimplemented!(), // retw.w
        0b0010 => unimplemented!(), // break.n
        0b0011 => unimplemented!(), // nop.n
        0b0110 => unimplemented!(), // ill.n
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use crate::decode::decode;
    use crate::instruction::Instruction;
    use crate::instruction::Rrr;

    #[test]
    fn abs() {
        let data = [0x00, 0x01, 0x60];
        let decoded = decode(&data);
        let expected = Some(Instruction::Abs(Rrr::new(0, 1, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn add() {
        let data = [0x00, 0x00, 0x80];
        let decoded = decode(&data);
        let expected = Some(Instruction::Add(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn addx2() {
        let data = [0x00, 0x00, 0x90];
        let decoded = decode(&data);
        let expected = Some(Instruction::Addx2(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn addx4() {
        let data = [0x00, 0x00, 0xa0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Addx4(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn addx8() {
        let data = [0x00, 0x00, 0xb0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Addx8(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn and() {
        let data = [0x00, 0x00, 0x10];
        let decoded = decode(&data);
        let expected = Some(Instruction::And(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn dsync() {
        let data = [0x30, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Dsync(Rrr::new(2, 0, 3)));
        assert!(decoded == expected);
    }

    #[test]
    fn esync() {
        let data = [0x20, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Esync(Rrr::new(2, 0, 2)));
        assert!(decoded == expected);
    }

    #[test]
    fn extui() {
        // TODO
    }

    #[test]
    fn extw() {
        let data = [0xd0, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Extw(Rrr::new(2, 0, 13)));
        assert!(decoded == expected);
    }

    #[test]
    fn isync() {
        let data = [0x00, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Isync(Rrr::new(2, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn memw() {
        let data = [0xc0, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Memw(Rrr::new(2, 0, 12)));
        assert!(decoded == expected);
    }

    #[test]
    fn moveqz() {
        let data = [0x00, 0x00, 0x83];
        let decoded = decode(&data);
        let expected = Some(Instruction::Moveqz(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn movgez() {
        let data = [0x00, 0x00, 0xb3];
        let decoded = decode(&data);
        let expected = Some(Instruction::Movgez(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn movltz() {
        let data = [0x00, 0x00, 0xa3];
        let decoded = decode(&data);
        let expected = Some(Instruction::Movltz(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn neg() {
        let data = [0x00, 0x00, 0x60];
        let decoded = decode(&data);
        let expected = Some(Instruction::Neg(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    // TODO: nop
    //#[test]
    //fn nop() {
    //    let data = [0x00, 0x00, 0x60];
    //    let decoded = decode(&data);
    //    let expected = Some(Instruction::Neg(Rrr::new(0, 0, 0)));
    //    assert!(decoded == expected);
    //}

    #[test]
    fn or() {
        let data = [0x00, 0x00, 0x20];
        let decoded = decode(&data);
        let expected = Some(Instruction::Or(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn rsync() {
        let data = [0x10, 0x20, 0x00];
        let decoded = decode(&data);
        let expected = Some(Instruction::Rsync(Rrr::new(2, 0, 1)));
        assert!(decoded == expected);
    }

    #[test]
    fn sll() {
        let data = [0x00, 0x00, 0xa1];
        let decoded = decode(&data);
        let expected = Some(Instruction::Sll(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn slli() {
        let data = [0x00, 0x00, 0x01];
        let decoded = decode(&data);
        let expected = Some(Instruction::Slli(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn sra() {
        let data = [0x00, 0x00, 0xb1];
        let decoded = decode(&data);
        let expected = Some(Instruction::Sra(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn srai() {
        let data = [0x00, 0x00, 0x21];
        let decoded = decode(&data);
        let expected = Some(Instruction::Srai(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn src() {
        let data = [0x00, 0x00, 0x81];
        let decoded = decode(&data);
        let expected = Some(Instruction::Src(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn srl() {
        let data = [0x00, 0x00, 0x91];
        let decoded = decode(&data);
        let expected = Some(Instruction::Srl(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn srli() {
        let data = [0x00, 0x00, 0x41];
        let decoded = decode(&data);
        let expected = Some(Instruction::Srli(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn ssa8l() {
        let data = [0x00, 0x20, 0x40];
        let decoded = decode(&data);
        let expected = Some(Instruction::Ssa8l(Rrr::new(2, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn ssai() {
        let data = [0x00, 0x40, 0x40];
        let decoded = decode(&data);
        let expected = Some(Instruction::Ssai(Rrr::new(4, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn ssl() {
        let data = [0x00, 0x10, 0x40];
        let decoded = decode(&data);
        let expected = Some(Instruction::Ssl(Rrr::new(1, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn ssr() {
        let data = [0x00, 0x00, 0x40];
        let decoded = decode(&data);
        let expected = Some(Instruction::Ssr(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn sub() {
        let data = [0x00, 0x00, 0xc0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Sub(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn subx2() {
        let data = [0x00, 0x00, 0xd0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Subx2(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn subx4() {
        let data = [0x00, 0x00, 0xe0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Subx4(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn subx8() {
        let data = [0x00, 0x00, 0xf0];
        let decoded = decode(&data);
        let expected = Some(Instruction::Subx8(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn xor() {
        let data = [0x00, 0x00, 0x30];
        let decoded = decode(&data);
        let expected = Some(Instruction::Xor(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }

    #[test]
    fn xsr() {
        let data = [0x00, 0x00, 0x61];
        let decoded = decode(&data);
        let expected = Some(Instruction::Xsr(Rrr::new(0, 0, 0)));
        assert!(decoded == expected);
    }
}
