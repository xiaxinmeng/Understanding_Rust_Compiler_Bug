rust
let mut a = [1, 2, 3, 4, 5, 6, 7];
let offset = 2;
let (bottom, top) = a[offset..].iter_mut().partition_in_place(|&n| n % 2 == 0);
assert!(bottom.all(|&n| n % 2 == 0)); // evens
assert!(top.all(|&n| n % 2 == 1)); // odds
