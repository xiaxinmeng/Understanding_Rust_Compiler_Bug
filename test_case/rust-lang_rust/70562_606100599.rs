rust
use std::convert::TryInto;

fn main() {
    let s: &[u32] = &[0, 1];
    let [x, y] = s.try_into().unwrap();
}
