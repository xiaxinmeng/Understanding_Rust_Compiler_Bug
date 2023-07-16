
#![feature(asm)]

pub fn xswap(word: &mut u64, value: u64) -> u64 {
    let result: u64;
    let wp = word as *mut u64;
    unsafe {
        asm!("lock; xchgq %0, %1"
             : "=r"(result), "=m"(*wp)
             : "0"(value), "m"(*wp)
             :: "volatile");
    }
    result
}

fn main() {
    let mut a = 0u64;
    let b = xswap(&mut a, 1);
    println!("a = {}, b = {}", a, b);
}
