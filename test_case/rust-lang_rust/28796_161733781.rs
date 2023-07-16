 rust
#![crate_type = "lib"]
#![feature(fundamental)]

#[fundamental]
trait FnBox<A> {}
#[fundamental]
trait FnOnce<A> {}

struct Box<T: ?Sized>(T);

impl<A> FnOnce<A> for Box<FnBox<A>> {}

impl<F: FnOnce<A> + ?Sized, A> FnOnce<A> for Box<F> {}
