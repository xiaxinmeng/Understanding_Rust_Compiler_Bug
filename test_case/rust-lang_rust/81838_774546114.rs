rust
#![feature(asm)]

const BAR: u64 = 10;

pub fn bar(mut v: u64) -> u64 {
    unsafe { 
        asm!(r"
        .macro foo, arg, size
          add \arg, \size
        .endm");
        asm!("foo {v}, {}", const(BAR), v = inout(reg) v);
    }
    v
}
