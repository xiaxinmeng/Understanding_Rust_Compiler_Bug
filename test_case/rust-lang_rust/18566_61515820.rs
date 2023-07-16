 Rust
#![feature(tuple_indexing)]

struct MyPtr<'a>(&'a mut uint);
impl<'a> Deref<uint> for MyPtr<'a> {
    fn deref<'b>(&'b self) -> &'b uint { self.0 }
}

trait Tr {
  fn poke(&self, s: &mut uint);
}
impl Tr for uint {
  fn poke(&self, s: &mut uint)  {
      println!("{}", self);
      *s = 2;
      println!("{}", self);
  }
}

fn main() {
    let s = &mut 1u;

    MyPtr(s).poke(s);
}
