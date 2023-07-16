 rust
impl<D: Decoder> Decodable<D> for Json {
  fn (d: &mut D) -> Json {
    // can i somehow match against types?
    match Decodable::decode(d) {
      f64 => Number,
      // etc?
    }
  }
}
