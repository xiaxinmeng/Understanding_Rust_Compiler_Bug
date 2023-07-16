 rust
  fn tail(&self) -> LinkedList {
    match *self {
      Cons(_, ref l) => {
        *l.clone()
      },
      _ => fail!("called `tail` on Empty"),
    }
  }
