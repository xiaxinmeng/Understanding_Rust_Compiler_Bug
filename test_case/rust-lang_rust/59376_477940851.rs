rust
enum Foo {
    #[non_exhaustive]
    Unit,
    #[non_exhaustive]
    Tuple(u8, u8),
    #[non_exhaustive]
    Struct { x: u8, y: u8 },
}
