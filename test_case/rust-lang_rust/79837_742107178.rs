rust
#![feature(asm)]

fn main() {
    let mut addr1: u64;
    let mut addr2: u64;
    unsafe {
        asm!(
            "lea {addr1}, 1f[rip]",
            "1:",
            "lea {addr2}, 2f[rip]",
            "2:",
            addr1 = out(reg) addr1,
            addr2 = out(reg) addr2,
        );
    }
    println!("{:?}\n{:#x}\n{:#x}", main as *const (), addr1, addr2);
}
