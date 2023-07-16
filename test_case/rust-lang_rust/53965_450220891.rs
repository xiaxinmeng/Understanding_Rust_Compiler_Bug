
#![feature(trait_alias)]

trait F8 = FnOnce() -> u8;

fn higher<T: F8>(t: T) -> u8 {
    t()
}

fn main() {
    higher(|| 3u16);
}
