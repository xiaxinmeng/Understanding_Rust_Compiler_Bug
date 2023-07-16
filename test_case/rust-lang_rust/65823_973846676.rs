rust
#[my_proc_attribute_macro(foo)]
#[my_proc_attribute_macro(bar)]
struct S {
  #[helper(option)]
  a: u8,
}
