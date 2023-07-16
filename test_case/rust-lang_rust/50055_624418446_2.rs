rust
struct X<'a> {
    y: &'a (),
}
struct S {
    x: X<'_>,
}
