rust
#![feature(derive_const)]

#[derive_const(PartialEq)]
#[derive(Debug, Eq)]
struct Foo([u32; 4]);

const fn bar() {
    const ONE: Foo = Foo([1, 0, 0, 0]);
    match Foo([0, 0, 0, 0]) {
        ONE => unreachable!(),
        _ => {},
    }

    if ONE == Foo([0, 0, 0, 0]) {
        unreachable!()
    }
}
