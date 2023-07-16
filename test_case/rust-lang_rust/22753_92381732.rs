 rust
pub trait Test {
  fn something(&self) -> i32;
}

#[derive(Copy, Clone)]
pub struct Foo {
  field: i32
}

#[derive(Copy, Clone)]
pub struct Foo2 {
  field: i32
}

impl Test for Foo {
  #[no_mangle]
  fn something(&self) -> i32 {
    self.field
  }
}

impl Test for Foo2 {
  #[no_mangle]
  fn something(&self) -> i32 {
    self.field
  }
}

fn main(){}
