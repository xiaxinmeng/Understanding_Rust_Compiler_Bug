rust
impl Buffer {
  ...
  crate fn write_str(&mut self, s: &str) {
     <Self as Write>::write_str(self, s).unwrap();
  }

  crate fn write_fmt(&mut self, v: fmt::Arguments<'_>) {
     <Self as Write>::write_fmt(self, v).unwrap();
  }
  ...
}
