 rust
extern {
  static StackBase: *const usize;
  fn main();
}

#[link_section=".isr_vector_temp"]
static ISRVectors: &'static [*const usize] = &[
  StackBase as *const usize,
  main as *const usize,
];
