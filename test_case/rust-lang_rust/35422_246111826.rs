
#![feature(asm)]

macro_rules! x {
    () => (asm!("x"));
}

fn main() {
    unsafe{x!()};
}
