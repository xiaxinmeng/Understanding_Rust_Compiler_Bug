

trait Tr {
  fn dostuff<A>(&self, v: A) -> A {
    v
  }
}

struct St <T> {
  val: T,
}

impl<T> Tr for St <T> {}

fn main () {
  let a = St { val: 1 };
  let b = &a as &Tr;
}
