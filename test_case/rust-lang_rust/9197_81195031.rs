 rust
trait Foo {
  fn noop(&self) {}
}

trait Bar: Foo {}

#[allow(dead_code)]
fn bar<T: Foo + Bar>() {}

fn main() {}
