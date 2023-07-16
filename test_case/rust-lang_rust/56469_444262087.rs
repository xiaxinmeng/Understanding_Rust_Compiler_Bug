rust
thread_local! {
    static ref FOO: Foo = const { Foo };
}
