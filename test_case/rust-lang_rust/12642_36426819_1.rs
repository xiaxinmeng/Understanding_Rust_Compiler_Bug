 rust
struct SmallerX { priv data: [u8,..1] }

impl Compact<SmallerX> for X {
  fn compact(self)->SmallerX {
    let mut result = SmallerX{data:[0u8]};
    match self {
      Foo(x) => {
        match x {
          Some(y) => {
            match y {
              true => result.data[0] = 0,
              false => result.data[0] = 1,
            }
          },
          None => result.data[0] = 2,
        }
      },
      Bar(x) => {
        match x {
          true => result.data[0] = 3,
          false => result.data[0] = 4,
        }
      },
    }
    result
  }
}

impl SmallerX {
  fn uncompact(self) -> X {
    match self.data[0] {
      0 => Foo(Some(true)),
      1 => Foo(Some(false)),
      2 => Foo(None),
      3 => Bar(true),
      4 => Bar(false),
    }
  }
}
