rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Foo<T>(T);

fn test() where [u8; {
    let _: Foo<[u8; 7]>;
    3
}]: Sized {}