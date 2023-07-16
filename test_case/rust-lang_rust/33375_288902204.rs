rust
trait Foo {
  fn apply() -> i64;
}

struct Baz{}
static BAZ: Baz = Baz{};
impl Foo for Baz {
    fn apply(&self) -> i64 { 4 }
}

fn lookup() -> Box<&'static Foo> {
    Box::new(&BAZ)
}
