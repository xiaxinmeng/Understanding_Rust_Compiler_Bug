 rust
#![feature(const_fn)]
const fn a(i: bool) -> bool { i || a(true) }

fn main() {
  println!("{}", a(false));
}
