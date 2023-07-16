
yield fn foo(m: uint) -> pub struct FooIterator {
    loop {
        yield 1;
        yield 2;
        yield m;
    }
}
