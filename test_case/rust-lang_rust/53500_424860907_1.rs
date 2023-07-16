rust
let iter = vec![Some(1), None, Some(3)].into_iter();
let filtered = iter.flatten().collect::<Vec<_>>();
assert_eq!(vec![1, 3], filtered);
