rust
struct Type<'a, 'b, T> {
    a: T,
    b: &'b &'a (),
}
