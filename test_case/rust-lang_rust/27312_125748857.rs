 rust
// Here the type of the integer literal 22 is inferred
// to `uint` because the default on `T` overrides the
// standard integer literal fallback.
fn foo<T=uint>(t: T) { ... }
foo(22)
