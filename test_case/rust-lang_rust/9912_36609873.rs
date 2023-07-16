 rust
struct Foo {...}
struct Bar {
    super: Foo,
    ...
}
// This can even be done by a simple macro:
impl Deref<Foo> for Bar {
    fn deref<'a>(&'a self) -> &'a Foo {&self.super}
}
impl DerefMut<Foo> for Bar {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Foo {&mut self.super}
}
