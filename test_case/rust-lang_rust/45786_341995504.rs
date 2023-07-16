rust
let vals = [('a', 4), ('b', 5)];
let foo = BTreeMap::from_iter(vals.iter().cloned());
