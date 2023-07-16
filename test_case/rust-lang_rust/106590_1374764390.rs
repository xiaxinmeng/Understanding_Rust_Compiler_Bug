rust
use std::collections::HashMap;

fn main() {
    let mut x = HashMap::<u32, _>::new();
    x.insert(1i32, Default::default());
}
