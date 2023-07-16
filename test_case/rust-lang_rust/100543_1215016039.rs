rust
#[derive(PartialEq)]
struct S;

const FOO: fn(&S, &S) -> bool = S::eq;
