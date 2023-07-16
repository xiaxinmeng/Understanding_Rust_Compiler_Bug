rust
fn foo<T: Trait>() -> &'static i32 {
  if T::ASSOC != 0 { &(14/T::ASSOC) } else { panic!() }
}
