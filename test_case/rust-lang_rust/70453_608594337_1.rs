rust
enum Bar<FOO> {
    X = {
        const FOO: isize = 3;
        struct Baz<T>(T);
        Baz::<FOO>;
        0
    }
}
