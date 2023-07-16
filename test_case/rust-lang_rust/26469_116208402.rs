 rust
#![feature(asm)]

fn main() {
        println!("{:x}", unsafe { apicaddr() })
}

unsafe fn apicaddr() -> u64 {
        let mut a: u64;
        let mut d: u64;
        asm!("rdmsr" : "={eax}" (a), "={edx}" (d) : "{rcx}" (0x1Bu32) : "rdx");
        d << 32 | a
}
