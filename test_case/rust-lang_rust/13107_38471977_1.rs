 rust
enum Foo {
  Bar(~str, ~str)
}

impl<D: Decoder, E> Decodable<D, E> for Foo {
  fn decode(&self, d: &mut D) -> Result<(), E> {
    match *self {
      Bar(ref a, ref b) => {
        // i forget to use `emit_enum_variant`
        d.emit_enum(|d| {
          try!(d.emit_enum_variant_arg(0, |d| d.emit_str(a)));
          d.emit_enum_variant_arg(1, |d| d.emit_str(b))
        })
      }
    }
  }
}
