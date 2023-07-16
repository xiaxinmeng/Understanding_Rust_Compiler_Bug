rust
#![feature(convert_id)]
use std::convert::identity;

let iter = vec![Some(1), None, Some(3)].into_iter();
let filtered = iter.filter_map(identity).collect::<Vec<_>>();
assert_eq!(vec![1, 3], filtered);
