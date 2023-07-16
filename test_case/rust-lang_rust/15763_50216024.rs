 rust

use std::io::{IoResult, InvalidInput, standard_error};

fn deserialize() -> IoResult<(Box<uint>, ())> {
  Ok((try!(Ok(box 0)),
      try!(Err(standard_error(InvalidInput)))))
}

fn main() {
  println!("{}", deserialize());
}
