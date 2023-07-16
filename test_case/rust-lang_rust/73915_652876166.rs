rust
use std::mem;

fn test_all_refs<'a, T: 'a>(dummy: &mut T, iter: impl Iterator<Item = &'a mut T>) {
    // Gather all thrse references
    let mut refs: Vec<&mut T> = iter.collect();
    // Use them all. Twice, to be sure we got all interleavings.
    for r in refs.iter_mut() {
        mem::swap(dummy, r);
    }
    for r in refs {
        mem::swap(dummy, r);
    }
}

fn main() {
    let mut a = std::collections::BTreeMap::new();
    a.insert(1, 1);
    a.insert(2, 2);
    test_all_refs(&mut 13, a.values_mut());
}
