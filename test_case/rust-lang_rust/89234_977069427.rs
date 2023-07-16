rust
// uncommenting the explicit discriminants...
#[repr(u8)]
enum Enum {
    Foo /* = 0 */,
    Bar() /* = 1 */,
    Baz{} /* = 2 */,
}
// ...will break downstream casts like this:
let x = Enum::Bar() as u8;
