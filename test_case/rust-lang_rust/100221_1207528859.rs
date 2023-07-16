rust
trait Foo {
  fn needs_sized() where Self: Sized;
}

struct Unsized([u8]);

impl Foo for Unsized {
  fn needs_sized() where Self: Sized {}
}
