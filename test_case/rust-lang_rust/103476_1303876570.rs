Rust
#![feature(dropck_eyepatch)]
#![feature(let_chains)]
#![allow(irrefutable_let_patterns)]

struct B<T: ?Sized> { _t: *const T, }

unsafe impl<#[may_dangle] T: ?Sized> std::ops::Drop for B<T> {
    fn drop(&mut self) {}
}

fn it(_: &()) -> B<&()> { todo!() }

trait Tr<'a> {}

fn f() {
    if let n = () && let _ = it(&n) {};
}
