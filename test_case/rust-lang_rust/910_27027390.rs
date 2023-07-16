 rust
use std::rt::io::timer;

struct A {
  b: B,
}

struct B {
  foo: int,
}

impl Drop for A {
  fn drop(&mut self) {
    timer::sleep(50);
  }
}

impl Drop for B {
  fn drop(&mut self) {
    println!("dropping b\n");
  }
}

fn main() {
  do spawn {
    let _a = A { b: B { foo: 3 } };
  }
  fail!()
}
