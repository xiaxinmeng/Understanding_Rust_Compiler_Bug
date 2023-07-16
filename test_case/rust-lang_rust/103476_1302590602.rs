Rust
#![feature(let_chains)]
#![allow(irrefutable_let_patterns)]

struct B<T: ?Sized> { _t: std::marker::PhantomData<T>, }

impl<T: ?Sized> std::ops::Drop for B<T> {
    fn drop(&mut self) {}
}

fn it(_: &()) -> B<dyn Tr<'_>> { todo!() }

trait Tr<'a> {}

fn f() {
    if let n = () && let _ = it(&n) {};
}
