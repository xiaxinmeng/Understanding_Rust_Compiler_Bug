rust
let mut a = [1, 2, 3, 4, 5, 6, 7];
let offset = 2;
let i = a[offset..].iter_mut().partition_in_place(|&n| n % 2 == 0);
assert!(a[offset..i+offset].iter().all(|&n| n % 2 == 0)); // evens
assert!(a[i+offset..].iter().all(|&n| n % 2 == 1)); // odds
