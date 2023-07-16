 rust
use std::path::MAIN_SEPARATOR;

fn main() {
  match Some('/') {
    Some(MAIN_SEPARATOR) => { },
    _                    => { },
  }
}
