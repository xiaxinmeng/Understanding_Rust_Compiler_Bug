rust
let _: impl Send = || {
    {
        let guard = Foo;
        drop(guard);
    }
    yield;
};
