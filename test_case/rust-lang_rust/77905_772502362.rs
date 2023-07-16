rust
fn somefn_gat<'a, T: Foo>(f: fn(T::In<'a>) -> T::In<'a>) {
}
