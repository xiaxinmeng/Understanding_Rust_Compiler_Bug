rust
#[escapes(S, T)] // `S` and `T` are unhygienic, i.e. are at the call site of an expansion
macro m() {
    struct S; // defines `S` at the call site
    T // resolves at the call site
}
