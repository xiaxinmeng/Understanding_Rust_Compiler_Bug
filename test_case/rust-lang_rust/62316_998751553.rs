rust
use std::collections::BTreeSet;

fn main() {
    let s = (0..12).collect::<BTreeSet<_>>();
    // Keys 0..=5 are in first leaf, 6 is in root node, 7..=11 are in second leaf
    let mut it = s.into_iter();
    assert_eq!(it.next_back(), Some(11));
    assert_eq!(it.next_back(), Some(10));
    assert_eq!(it.next_back(), Some(9));
    assert_eq!(it.next_back(), Some(8));
    assert_eq!(it.last(), Some(7));
}
