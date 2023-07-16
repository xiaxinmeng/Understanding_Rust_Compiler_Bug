rust
#[link(name = "hello-lib.js", kind = "framework")]
extern {
  fn hello();
}

fn main() {
  unsafe { hello() }
}
