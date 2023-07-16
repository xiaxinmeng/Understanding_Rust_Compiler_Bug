rs
#[repr(transparent)]
pub struct Foo<T> {
  content: Vec<T>,
}
