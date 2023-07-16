
trait Trait {
  static fn bar();

  fn foo() {
    self.bar();
  }
}
