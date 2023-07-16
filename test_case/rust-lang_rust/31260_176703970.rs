 rust
struct Foo { s: &'static [u8] }
static FOO: Foo = Foo { s: b"foo" };
