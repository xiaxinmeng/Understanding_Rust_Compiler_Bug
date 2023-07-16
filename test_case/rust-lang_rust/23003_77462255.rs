 Rust
use std::marker;

trait Async {
    type Cancel;
}

struct Receipt<A: ?Sized + Async> {
    marker: marker::PhantomData<A>,
}

struct Complete<B: ?Sized> {
    core: marker::PhantomData<B>,
}

impl<B: ?Sized> Async for Complete<B> {
    type Cancel = Receipt<Complete<Option<B>>>;
}

fn foo(r: Receipt<Complete<()>>) { }

fn main() { }
