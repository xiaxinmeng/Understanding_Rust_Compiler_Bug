 rust
// This is easier for me to understand
impl<T : PartialEq> Foo {
  fn method1(&self, a: T);
  fn method2(&self, a: T);
  fn method3(&self, a: T);
  fn method4(&self, a: T);
  fn method5(&self, a: T);
} 
// But it needs to be this way?
impl Foo {
  fn method1<T : PartialEq>(&self, a: T);
  fn method2<T : PartialEq>(&self, a: T);
  fn method3<T : PartialEq>(&self, a: T);
  fn method4<T : PartialEq>(&self, a: T);
  fn method5<T : PartialEq>(&self, a: T);
} 
