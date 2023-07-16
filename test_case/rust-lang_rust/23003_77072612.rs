 rust
use std::marker::PhantomData;

trait Async {
    type Cancel;
}

struct Receipt<A:Async> {
    marker: PhantomData<A>,
}

struct Complete<B> {
    core: Option<B>,
}

impl<B> Async for Complete<B> {
    type Cancel = Receipt<Complete<Option<B>>>;
}

fn foo(r: Receipt<Complete<()>>) { }

fn main() { }
