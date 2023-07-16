 Rust
use std::ops::Deref;

fn write<'a, F>(f: F) where F: FnOnce() -> &'a u32 {
    println!("{}", f());
}

fn main() {
    let k = || (&mut 42).deref(); // no rvalue promotion here please
    write(k)
}
