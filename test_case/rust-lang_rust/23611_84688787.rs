
use std::mem;
use std::ptr;

struct Noisy(usize);
impl Drop for Noisy {
    fn drop(&mut self) { 
        println!("Dropping {:x}.", self.0)
    }
}

enum Foo {
    Bar(Noisy),
    Baz(usize)
}
use Foo::*;

impl Drop for Foo {
    fn drop(&mut self) {
        match *self {
            Bar(_) => {
                unsafe { mem::forget(mem::replace(self, Foo::Baz(0xdeadbeef))) }
            }
            Baz(_) => ()
        }
    }
}

fn main() {
    let _ = Foo::Bar(Noisy(0));
}
