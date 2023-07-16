rust
let mut flat = vec![vec!['b'; 1], vec!['c'; 1_000_000]].into_iter().flatten();
dbg!(flat.size_hint());
flat.next();
dbg!(flat.size_hint());
flat.next();
dbg!(flat.size_hint());
