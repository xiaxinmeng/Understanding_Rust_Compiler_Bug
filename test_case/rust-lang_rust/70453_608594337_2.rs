rust
enum Bar {
    X = {
        const FOO: isize = 3;
        struct Baz<T>(T);
        Baz::<FOO>;
        0
    }
}
