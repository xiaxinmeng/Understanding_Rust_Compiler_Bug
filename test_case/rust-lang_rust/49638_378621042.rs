rust
use std::collections::{Bound, BTreeSet};
fn get_lt<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Unbounded, Bound::Excluded(value))).next_back()
}

fn get_le<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Unbounded, Bound::Included(value))).next_back()
}

fn get_gt<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Excluded(value), Bound::Unbounded)).next()
}

fn get_ge<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Included(value), Bound::Unbounded)).next()
}

fn main() {
    let set = vec![1, 4, 6].into_iter().collect::<BTreeSet<_>>();
    assert_eq!(get_lt(&set, &0), None);
    assert_eq!(get_lt(&set, &1), None);
    assert_eq!(get_lt(&set, &2), Some(&1));
    assert_eq!(get_lt(&set, &7), Some(&6));
    
    assert_eq!(get_le(&set, &0), None);
    assert_eq!(get_le(&set, &1), Some(&1));
    assert_eq!(get_le(&set, &2), Some(&1));
    assert_eq!(get_le(&set, &7), Some(&6));
    
    assert_eq!(get_gt(&set, &7), None);
    assert_eq!(get_gt(&set, &6), None);
    assert_eq!(get_gt(&set, &5), Some(&6));
    assert_eq!(get_gt(&set, &0), Some(&1));
    
    assert_eq!(get_ge(&set, &7), None);
    assert_eq!(get_ge(&set, &6), Some(&6));
    assert_eq!(get_ge(&set, &5), Some(&6));
    assert_eq!(get_ge(&set, &0), Some(&1));
}
