rust
trait Foo {
  const X: u32;
  fn get_x(&self) -> u32 { Self::X } 
}
