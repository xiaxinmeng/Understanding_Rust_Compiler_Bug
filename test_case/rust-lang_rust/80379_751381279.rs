rust
// crate1
use std::collections::*;

pub enum Types {
  Map(HashMap<usize, usize>),
  List(Vec<usize>),
  Pointer(Box<usize>),
}

// crate2
pub use crate1::Types;
