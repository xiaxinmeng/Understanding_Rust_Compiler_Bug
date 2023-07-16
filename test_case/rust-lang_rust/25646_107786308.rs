 rust
use std::mem;

#[repr(packed)]
struct Foo {
    _x: u8,
    y: u32,
}
fn main() {
    let x = Foo { _x: 0, y: 0 };

    println!("{}", &x.y as *const _ as usize % mem::align_of::<u32>())
}
