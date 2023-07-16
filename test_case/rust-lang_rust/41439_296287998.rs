rs
let mut it = (0..).every_nth(1).take(3);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(2));
assert_eq!(it.next(), Some(4));
assert_eq!(it.next(), None);
