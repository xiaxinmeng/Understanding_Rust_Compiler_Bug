
unsafe fn u128_by_u64_div_rem(duo: u128, div: u64) -> (u64, u64) {
    let duo_lo = duo as u64;
    let duo_hi = (duo >> 64) as u64;
    let quo: u64;
    let rem: u64;
    unsafe {
        // divides the combined registers rdx:rax (`duo` is split into two 64 bit parts to do this)
        // by `div`. The quotient is stored in rax and the remainder in rdx.
        // FIXME: Use the Intel syntax once we drop LLVM 9 support on rust-lang/rust.
        core::arch::asm!(
            "div {0}",
            in(reg) div,
            inlateout("rax") duo_lo => quo,
            inlateout("rdx") duo_hi => rem,
            options(att_syntax, pure, nomem, nostack)
        );
    }
    (quo, rem)
}
