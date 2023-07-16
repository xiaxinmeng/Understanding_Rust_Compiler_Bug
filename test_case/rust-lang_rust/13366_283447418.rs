rust
#![feature(asm)]

#[inline(never)]
unsafe fn print_first_half(arr: [u8; 16]) {
    let out: u64;
    asm!("movups $1, %xmm0
          pextrq $$0, %xmm0, $0"
          : "=r"(out) : "m"(arr) : "xmm0");
    println!("{:?}", out);
}

fn main() {
    let arr: [u8; 16] = [0; 16];
    unsafe { print_first_half(arr); }
}
