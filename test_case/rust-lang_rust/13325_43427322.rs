 rust
extern {
  static StackBase: u32;
  fn main();
}

#[link_section=".isr_vector_temp"]
static ISRVectors: &'static [u32] = &'static [
  &StackBase as u32,
  main as u32,
];
