rust
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let map = HashMap::from_iter(vec![
        (1, 2),
        (3, 4),
        (5, 6)
    ]);
}
