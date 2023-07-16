 rust
struct Foo {
  bar: ~str,
  baz: uint
}
impl<E: Encoder> Encodable<E> for Foo {
  fn encode(&self, e: &mut E) {
    e.emit_str(self.bar);
    e.emit_uint(self.baz);
  }
}
