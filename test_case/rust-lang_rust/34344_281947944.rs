rust
#![feature(associated_consts)]
trait Size {
    const size: usize;
}

struct One;
impl Size for One {
    const size: usize = 1;
}

struct MyArray<T: Size> {
    data: [i32; T::size],
}

fn main() { }
