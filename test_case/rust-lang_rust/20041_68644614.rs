 rust
#![feature(associated_types)]

struct CustomStruct {
    this: int,
    that: int,
}

fn do_something(i: int) {
    println!("{}", i);
}

// Old code
#[cfg(target_os="none")]
fn foo_old<I>(mut iter: I) where I: Iterator<CustomStruct> {
    for obj in iter {
        do_something(obj.this + obj.that);
    }
}

// New code, but doesn't work due to #20041.
/*
fn foo_new<I>(mut iter: I) where I: Iterator, <I as Iterator>::Item = CustomStruct {
    for obj in iter {
        do_something(obj.this + obj.that);
    }
}
*/

// Workaround code, inspired by http://redd.it/2r2fbl
trait Is<Sized? A> { fn this(&self) -> &A; }
impl<Sized? A> Is<A> for A { fn this(&self) -> &A { self } }
fn workaround_20041<A, B: Is<A>>(a: &B) -> &A { a.this() }

fn foo_workaround<I>(mut iter: I) where I: Iterator, <I as Iterator>::Item: Is<CustomStruct> {
    for obj in iter {
        let obj = workaround_20041::<CustomStruct, _>(&obj);
        do_something(obj.this + obj.that);
    }
}

fn main() {
    foo_workaround(vec![CustomStruct { this: 11111, that: 22222 }].into_iter());
}
