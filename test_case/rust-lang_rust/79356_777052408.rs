rust
#![feature(const_generics)]
#![allow(incomplete_features)]

struct Foo<T>(T);

fn test() where [u8; {
    let _: Foo<[u8; 3 + 4]>;
    3
}]: Sized,
[u8; {
    let _: Foo<[u8; 3 + 4]>;
    4
}]: Sized  {}
