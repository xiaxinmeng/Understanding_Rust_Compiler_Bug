
#![feature(asm)]
#![allow(dead_code)]
#[cfg(target_arch = "x86")]
#[cfg(target_arch = "x86_64")]
pub fn main() {
    // assignment not dead
    let mut x: int = 17;
    unsafe {
        asm!("mov $1, $0"
             : "=r"(x)
             : "r"(5u),
               "0"({println!("eval input-2, x: {}", x); x}) : "cc");
    }
    assert_eq!(x, 5);
}
