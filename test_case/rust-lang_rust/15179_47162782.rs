 .rs
#![feature(asm)]
fn main() {
    let canary: u64;
    unsafe {
        asm!("movq %fs:0x28, $0" : "=r"(canary))
    };
    println!("{:016x}", canary);
}
