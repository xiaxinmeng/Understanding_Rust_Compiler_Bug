 rust
use std::mem;
use std::marker::PhantomData;

trait Foo {
    type Error;
}

struct Bar<U: Foo> {
    stream: PhantomData<U::Error>,
}

fn foo<U: Foo>() -> Bar<U> {
    unsafe { mem::transmute([0u32; 4]) }
}
