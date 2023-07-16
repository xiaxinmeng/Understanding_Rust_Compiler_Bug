 rust
struct Bar<'b> {
    _field : &'b [u8]
}
struct Foo<'a, 'b: 'a> {
    _field: &'a Bar<'b>
}
fn main() {
}
