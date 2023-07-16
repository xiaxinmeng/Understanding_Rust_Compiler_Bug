rust
use std::collections::HashSet;

fn main() {
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [4, 2, 3, 4].iter().cloned().collect();

    let union: HashSet<i32> = a.union(&b).cloned().collect();
    assert_eq!(union, [1, 2, 3, 4].iter().cloned().collect());
}
