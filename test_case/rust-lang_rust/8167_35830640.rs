 rust
mod barfoo {
  fn foo() {}
}

fn main() {
  concat_idents!(bar, foo);

  ::foo();
}
