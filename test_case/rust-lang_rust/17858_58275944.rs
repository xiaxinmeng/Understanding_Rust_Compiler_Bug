 rust
#![feature(unsafe_destructor)]

struct S<'a>(&'a T);

struct T;

#[unsafe_destructor]
impl<'a> Drop for S<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let _ = S(&T);
}
