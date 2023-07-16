rust
fn f<T: HasAssocType<Inner = Y>, Y>() -> impl Tr<T, Y> { /* .. */ }
