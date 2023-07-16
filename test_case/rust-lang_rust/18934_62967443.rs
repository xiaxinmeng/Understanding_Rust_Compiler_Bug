 rust
use std::default::Default;

fn main() {
    Some(42u).unwrap_or_else(Default::default);
}
