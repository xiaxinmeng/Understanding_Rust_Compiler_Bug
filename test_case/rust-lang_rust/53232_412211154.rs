rust
// rustup run nightly rustc -C opt-level=3 -C target-cpu=native -o main_exp_rustc main_exp.rs && ./main_exp_rustc
// rustup run nightly rustc -C opt-level=3 -C target-cpu=native --emit asm -o main_exp_rustc.s main_exp.rs

#![feature(asm)]

use std::arch::x86_64::*;

union XmmTransmute {
    epi8: [i8; 16],
    xmm: __m128i,
}

/// **Rust extension:** Compare packed strings in `a` and `b` with lengths `la`
/// and `lb` using the control byte in `imm8`, and return a tuple of:
/// - the resulting mask
/// - the CF (1 if the resulting mask was non-zero; otherwise, 0)
/// - the ZF (1 if -16 < `lb` < 16; otherwise, 0)
/// - the SF (1 if -16 < `la` < 16; otherwise, 0)
/// - the OF (the least significant bit of the resulting mask)
#[inline(always)]
fn cmpestrm_and_flags(a: __m128i, la: i64, b: __m128i, lb: i64, imm8: u8) -> (__m128i, i32, i32, i32, i32) {
    let mask: __m128i;
    let status_flags: u16;
    // GCC supports "flag output constraints" (e.g. =@ccc for the CF):
    // https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html#index-asm-flag-output-operands
    // This would be great to use, but it appears that LLVM does not have a
    // similar feature:
    // http://llvm.org/docs/LangRef.html#supported-constraint-code-list
    unsafe {
        asm!("pcmpestrm $6, $4, $2
              ${:comment} lahf stores the CF, PF, AF, ZF, and SF into %ah, but not the OF.
              ${:comment} We'll place OF at bit 0 of %ax by conditionally moving.
              mov $$0, %ax
              mov $$1, %dx
              cmovo %dx, %ax
              lahf"
             : "={xmm0}"(mask), "={ax}"(status_flags)
             : "x"(a), "{rax}"(la), "x"(b), "{rdx}"(lb), "i"(imm8)
             : "flags");
    }
    let cf = ((status_flags >> 8) & 1_u16) as i32;
    let zf = ((status_flags >> 14) & 1_u16) as i32;
    let sf = ((status_flags >> 15) & 1_u16) as i32;
    let of = (status_flags & 1_u16) as i32;
    (mask, cf, zf, sf, of)
}

fn main() {
    let haystack = "the quick brown fox jumps over a lazy dog\u{0}";

    unsafe {
        let haystack = haystack.get_unchecked(..16);
        println!("haystack = '{}'", haystack);

        let needle_xmm = XmmTransmute { epi8: ['a' as i8, 'e' as i8, 'i' as i8, 'o' as i8, 'u' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }.xmm;
        let haystack_xmm = _mm_loadu_si128(haystack.as_ptr() as *const __m128i);

        println!("\nTrying PCMPESTRM...");
        let (mask_xmm, cf, zf, sf, of) = cmpestrm_and_flags(needle_xmm, 5, haystack_xmm, 16, 0);
        println!("cf = {}, zf = {}, sf = {}, of = {}", cf, zf, sf, of);
        if cf != 0 {
            let mask = _mm_extract_epi32(mask_xmm, 0) as u16;
            println!("PCMPESTRM found {} vowel(s). mask = 0b{:016b}", mask.count_ones(), mask);
        } else {
            println!("No vowel was found by PCMPESTRM.");
        }
    }
}
