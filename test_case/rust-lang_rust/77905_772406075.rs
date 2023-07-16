
#![feature(generic_associated_types)]

trait Foo {
    type In<'a>;
}

struct _Simple<'a>(std::marker::PhantomData<&'a u32>);

fn _somefn_simple(_f: for<'a> fn(_Simple<'a>) -> _Simple<'a>) {
    // compiles.
}

fn _somefn_gat<'a, T: Foo>(_f: for<'b> fn(T::In<'b>) -> T::In<'a>) {
    // errors.
}

fn main() {}
