 rust
use std::marker::PhantomData;

trait Async {
    type Cancel;
}

struct Receipt<A:Async> {
    marker: PhantomData<A>,
}

struct Complete {
    core: Option<()>,
}

impl Async for Complete {
    type Cancel = Receipt<Complete>;
}

fn foo(r: Receipt<Complete>) { }

fn main() { }
