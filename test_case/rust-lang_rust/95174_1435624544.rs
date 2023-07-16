rust
#![feature(adt_const_params)]

#[derive(Debug, PartialEq, Eq)]
struct Foo(u32);

const fn bar() {
    const ONE: Foo = Foo(1);
    match Foo(0) {
        ONE => unreachable!(),
        _ => {},
    }
}

const fn baz<const ONE: Foo>() {
    match Foo(0) {
        ONE => unreachable!(),
        _ => {},
    }
}
