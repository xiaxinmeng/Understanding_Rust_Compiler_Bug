rust
fn foo(x: A) -> ... {
    async move {
        let ref x = x;
        ...
    }
}
