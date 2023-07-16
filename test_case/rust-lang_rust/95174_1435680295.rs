rust

const fn bar() {
    const ONE: Foo = Foo(0);
    match Foo(0) {
        ONE => unreachable!(),
        _ => {},
    }
}
