rust
use std::arch::asm;

fn main() {
    let mut x: u64 = 4;
    let s: u8 = 1;
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, cl",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
            in("cl") s,
        );
    }
    println!("{}", x);
}
