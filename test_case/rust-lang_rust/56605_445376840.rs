
trait ArrayGenerator {
  const LEN: usize;
}

fn generate<T>(obj: T) -> [u8; <T as ArrayGenerator>::LEN] where T: ArrayGenerator {
  [0; T::LEN]
}

struct Foo { }

impl ArrayGenerator for Foo {
  const LEN: usize = 8;
}

fn main() {
  println!("{:?}", generate(Foo{}));
}
