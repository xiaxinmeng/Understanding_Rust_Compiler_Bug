 rust
use std::mem::transmute;
use std::ty::Unsafe;

struct A { a: int }
struct B { a: Unsafe<int> }

fn modify(x: &A) {
    let y: &B = unsafe { transmute(x) };
    unsafe {
        *y.a.get() = 2;
    }
}

fn main() {
    static x: A = A { a: 1 };
    let p = &x;
    modify(p);
}
