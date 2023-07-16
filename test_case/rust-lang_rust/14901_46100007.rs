 rust
use std::io::{Reader,BufReader};

enum Wrapper<'a> {
  WrapReader(&'a Reader)
}

trait Wrap<'a> {
  fn wrap(self) -> Wrapper<'a>;
}

impl<'a> Wrap<'a> for &'a mut Reader {
  fn wrap(self) -> Wrapper<'a> {
    WrapReader(self)
  }
}

fn doit<'a>(b: &'a [u8]) {
  let mut r = BufReader::new(b);
  (&mut r as &mut Reader).wrap();
}

pub fn main() {
  let s = "zomg".to_string();
  doit(s.as_slice().as_bytes());
  println!("Zomg!");
}
