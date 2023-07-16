rust
#![feature(llvm_asm)]

pub unsafe fn xchg(addr: usize, newval: u32) -> () {
    let result: u32;
    llvm_asm ! ("lock; xchgl $0, $1" : "+m" (* (addr as * mut u32)) , "=a" (result) : "1" (newval) : "cc");
}

fn main() {
    let mut x: usize = 0;
    unsafe { xchg(&mut x as *mut _ as usize, 1) };
}
