
#![feature(asm)]

fn main() { unsafe {
    // Enable "Invalid Operation" exceptions (SSE is used)
    // MXCSR.IM = 0
    let mut mxcsr = 0u32;
    asm!("stmxcsr $0" : "=*m" (&mxcsr) ::: "volatile");
    println!("default  mxcsr {:#016b}", mxcsr);
    mxcsr &= !(1 << 7);
    println!("modified mxcsr {:#016b}", mxcsr);
    asm!("ldmxcsr $0" :: "m" (mxcsr) :: "volatile");

    let a: f64 = std::mem::transmute(0xFFF0000000000001u64);
    let b = a + 1.0;
    println!("b {}", b);
}}
