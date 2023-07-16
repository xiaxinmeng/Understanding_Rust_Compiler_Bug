rust
let it = (-5..5).map(|x| 100 / x);
assert_eq!(it.size_hint(), (10, Some(10)));
