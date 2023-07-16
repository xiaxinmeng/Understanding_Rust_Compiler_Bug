
let mut it = (0..).every_nth(2).take(3);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(3));
assert_eq!(it.next(), Some(6));
assert_eq!(it.next(), None);
