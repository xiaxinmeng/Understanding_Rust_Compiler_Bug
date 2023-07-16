
let mut it = (0..).every_nth(0).take(2);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(1));
assert_eq!(it.next(), None);
