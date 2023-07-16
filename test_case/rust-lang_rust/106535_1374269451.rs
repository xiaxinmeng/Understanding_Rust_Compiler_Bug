rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::from([1, 2, 3, 4, 5, 6]);
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set, HashSet::from([2, 4, 6]));
}
