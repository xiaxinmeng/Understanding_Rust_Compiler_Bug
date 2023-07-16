rust
#![feature(never_type)]

fn main() {
    let mut x: (u32, !);
    x.0 = 100;
}
