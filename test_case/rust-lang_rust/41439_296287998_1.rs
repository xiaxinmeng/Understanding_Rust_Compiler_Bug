rs
let mut it = (0..).every_nth(1).take(3);
assert_eq!(it.next(), Some(1));
assert_eq!(it.next(), Some(3));
assert_eq!(it.next(), Some(5));
assert_eq!(it.next(), None);
