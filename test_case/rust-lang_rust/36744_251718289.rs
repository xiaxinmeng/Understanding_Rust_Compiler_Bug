 Rust
pub struct A<'a>(&'a ());
pub struct S<T>(T);

#[no_mangle]
pub fn bad<'s>(v: &mut S<fn(A<'s>)>, y: S<for<'b> fn(A<'b>)>) {
    *v = y;
}

fn main() {}
