rust
#![feature(repr_align_enum)]

#[repr(align(1))]
enum X { X(u64) }

fn main() {
    match &X::X(5) {
        X::X(x) => println!("{:?}", x),
    }
}
