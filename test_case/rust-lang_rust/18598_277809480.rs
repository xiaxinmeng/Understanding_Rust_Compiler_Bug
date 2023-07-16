rust
use std::mem;

struct Bar;
struct Baz;

impl Drop for Bar {
    fn drop(&mut self) {
        println!("dropping bar");
    }
}

impl Drop for Baz {
    fn drop(&mut self) {
        println!("dropping baz");
    }
}

fn main() {
    unsafe {
        let bar: Box<Bar> = Box::new(Bar);
        let _baz: Box<Baz> = mem::transmute(bar);
    }
}
