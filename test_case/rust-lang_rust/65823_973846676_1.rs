rust
#[my_proc_attribute_macro]
#[my_proc_attribute_macro_helper(foo)]
#[my_proc_attribute_macro_helper(bar)]
struct S {
  #[helper(option)]
  a: u8,
}
