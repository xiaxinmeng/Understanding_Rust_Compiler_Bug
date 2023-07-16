rust
#![feature(range_is_empty)]
#![feature(exact_size_is_empty)]
fn main() {
    println!("{:?}", (0..0).is_empty());
}
