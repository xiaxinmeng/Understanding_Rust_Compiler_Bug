rust
#![feature(optin_builtin_traits)]

struct Foo;

impl Foo {
  fn v(&self) -> () {}
}

impl !Send for Foo {}

fn assert_send<T: Send>(_: T) {}

async fn take<T>(_: T) {}

fn main() {
  // This compiles fine
  assert_send(async {
    let v = Foo.v();

    take(v).await;
  });

  // This used to work but now doesn't
  assert_send(async {
    take(Foo.v()).await;
  });
}
