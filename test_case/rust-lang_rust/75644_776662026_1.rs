rust
#[derive(Debug)]
enum CustomLibErrors {
  ...
  FillError(FillError),
  ...
}

impl From<FillError> for CustomLibErrors {
  fn from(from: FillError) -> Self {
    Self::FillError(from)
  }
}

fn main() -> Result<(), CustomLibErrors> {
  let _: [i32; 123]  = (0..).map(|_| { ... stuff ... }).collect::<Result<[i32; 123], _>>()?;
}
