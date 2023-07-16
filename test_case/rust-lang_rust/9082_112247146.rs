 rust
use std::collections::hash_set::Iter;
use std::collections::HashSet;

fn iter_to_vec<'b, X>(i: Iter<'b, X>) -> Vec<X> {
    let i = i.map(|x| x.clone());
    i.collect()
}

fn main() {
    let mut s = HashSet::new();
    s.insert(1u8);
    println!("{:?}", iter_to_vec(s.iter()));
}
