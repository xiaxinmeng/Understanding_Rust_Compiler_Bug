
fn foo(f:&Foo) {
  f.huh();
}

trait Foo {
  fn bar(&self, self_:&Foo) {
    let self_:&Foo = self_;
    // let self_:&Foo = self; // Doesn't work
    foo(self_);
  }

  fn huh(&self) {
    println!("huh");
  }
}

struct IsFoo;
impl Foo for IsFoo {}

fn main() {
  let is_foo = IsFoo;
  let foo:&Foo = &is_foo;
  foo.bar(foo);
}
