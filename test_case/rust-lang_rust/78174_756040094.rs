rust
#![feature(inline_const)]

fn main() {
  match "foo" {
    const {"foo"} => println!("It works!"),
    _ => (),
  }
}
