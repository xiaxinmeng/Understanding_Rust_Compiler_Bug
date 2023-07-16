rust
fn func() -> &'static [&'static str] {
  ::std::future::from_generator(gen || {
    &["hi"]
  })
}
