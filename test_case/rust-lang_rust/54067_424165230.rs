
// ice.rs
#![feature(asm)]
pub struct Foo {
    pub x: usize,
    pub y: usize
}
#[no_mangle]
pub unsafe fn ice() -> () {
    let bar: Foo = Foo { x: 0, y: 0 };
    asm!("lgdt %0" : : "m" (bar) : : );
}
fn main() { }
// EOF
