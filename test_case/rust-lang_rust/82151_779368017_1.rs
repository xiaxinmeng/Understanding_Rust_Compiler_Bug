rust
---bar
pub struct Bar;
pub fn bar() -> Bar {
  Bar
}

---foo
pub use bar::bar;
pub struct Foo;
pub fn foo() -> Foo {
  Foo
}

--shared
pub struct Test;
impl Test {
  pub fn new() -> Self {
    let _ = foo::foo();
    let _ = foo::bar();
    Self
  }
}

--app
fn main() {
  let _ = shared::Test::new();
}
