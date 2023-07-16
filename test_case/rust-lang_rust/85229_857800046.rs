rust
#![no_std]
#![feature(asm)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

const RD_SHIFT: usize = 7;
const RS1_SHIFT: usize = 15;
const RS2_SHIFT: usize = 20;
const XREG_A0: usize = 10;
const XREG_A1: usize = 11;

pub unsafe fn asm_word_a0_a1_a0<const N: usize>(rs1: usize, rs2: usize) -> usize {   
    let rd: usize;
    asm!(".word {}", const N, in("a0") rs1, in("a1") rs2, lateout("a0") rd);
    rd
}

pub const fn addsl_instruction<const I: u8>() -> usize {
    0b0001011 |
    (XREG_A0 << RD_SHIFT) |
    (0b001 << 12) |
    (XREG_A0 << RS1_SHIFT) |
    (XREG_A1 << RS2_SHIFT) |
    ((I as usize) << 25)
} // const I should be <= 0b11

// rd = rd1 + (rs2 << imm2)
pub fn addsl<const I1: u8>(rs1: usize, rs2: usize) -> usize 
where [(); addsl_instruction::<I1>()]: Sized 
{
    unsafe { asm_word_a0_a1_a0::<{ addsl_instruction::<I1>() }>(rs1, rs2) }
}

pub fn main() {
    let rs1 = 100200300400;
    let rs2 = 200300400500;
    let a = addsl::<0>(rs1, rs2);
    let b = addsl::<1>(rs1, rs2);
    let c = addsl::<2>(rs1, rs2);
}
