rust
#![feature(fn_traits, unboxed_closures)]

struct A;
struct B;

// Equivalent to `fn foo<F>(_: F) where F: FnOnce(A) + FnOnce(A, B),`
fn foo<F: FnOnce(A)>(_: F)
where
    F: FnOnce(A, B),
{}

struct C;

impl FnOnce<(A,)> for C {
    type Output = ();
    extern "rust-call" fn call_once(self, _: (A,)) {}
}

impl FnOnce<(A, B)> for C {
    type Output = ();
    extern "rust-call" fn call_once(self, _: (A, B)) {}
}

fn main() {
    foo(C);
}
