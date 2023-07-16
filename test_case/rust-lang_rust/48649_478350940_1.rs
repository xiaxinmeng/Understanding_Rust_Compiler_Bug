rust
let mut iter = &[0, 1, 2, 3, 4, 5];
for i in iter { if *i > 2 { break; } }
FromIterator::from_iter(iter)
