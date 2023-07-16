 rust

use std::io::IoResult;
use std::io::{standard_error, InvalidInput};

fn build_vec_fail() -> IoResult<Vec<uint>> {
  Err(standard_error(InvalidInput))
}

fn build_vec() -> IoResult<Vec<uint>> {
  Ok(vec![])
}

struct MyStruct {
  field1: Vec<uint>,
  field2: Vec<uint>
}

fn real_main() -> IoResult<MyStruct> {
  // This is fine if we assign it to a variable then return the
  // variable. As written this segfaults.
  Ok(MyStruct {
    field1: try!(build_vec()),
    field2: try!(build_vec_fail()),
  })
}

fn main() { real_main(); }
