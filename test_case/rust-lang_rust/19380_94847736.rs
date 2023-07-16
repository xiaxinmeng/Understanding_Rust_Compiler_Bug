 rust
#![crate_type = "lib"]

trait Qiz {
  fn qiz();
}

struct Foo;
impl Qiz for Foo {
  fn qiz() {}
}

struct Bar {
  foos: &'static [&'static (Qiz + 'static)]
}

const FOO : Foo = Foo;
const BAR : Bar = Bar { foos: &[&FOO]};
