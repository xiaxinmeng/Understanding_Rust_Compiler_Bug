 rust
trait Foo {
  fn answer(&mut self) -> u8;
}

struct Newtype<'a, T: 'a> {
  value: &'a mut T,
}

// impl<'a, T> Foo for Newtype<'a, T> { fn answer(&mut self) -> u8 { 0 } }

fn stuff<'a, T>(this: &'a mut T) -> u8 where Newtype<'a, T>: Foo {
    let newtype: Newtype<'a, T> = Newtype{value: this};
    newtype.answer()
}

fn main() {}
