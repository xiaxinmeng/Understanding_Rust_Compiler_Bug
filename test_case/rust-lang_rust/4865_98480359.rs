 rust
pub mod a {
  use b::*;
}

pub mod b {
  use a::*;
}

use a::*;

fn main() {
}
