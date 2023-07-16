rust
let mut iter = [1, 2, 3].iter();
let sum: i32 = iter.by_ref().take(2).fold(0, |acc, i| acc + i);
assert_eq!(sum, 3);
assert_eq!(iter.next(), Some(&3));
