 rust
fn foo<T>() {
    fn bar(t: T) {} // invalid
    static FOO: T = ...; // invalid
    // etc.
}
