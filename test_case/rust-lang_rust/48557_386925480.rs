rust
#![feature(nll)]

trait A {}

impl<'a> A for &'a i32 {}

struct Dst<T: ?Sized> { x: T }

fn unsized_local() where Dst<A>: Sized {
    let y = 1;
    let x: Dst<A> = *(Box::new(Dst { x: &y }) as Box<Dst<A>>);
    // ^error: internal compiler error: unexpected region for local data ReStatic
}

fn main() {}
