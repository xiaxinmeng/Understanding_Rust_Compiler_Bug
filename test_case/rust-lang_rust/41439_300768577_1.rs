rust
let mut it = (0..).every_nth(0).take(4);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(1));
assert_eq!(it.next(), Some(2));
assert_eq!(it.next(), Some(3));
assert_eq!(it.next(), None);

let mut it = (0..).every_nth(1).take(4);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(2));
assert_eq!(it.next(), Some(4));
assert_eq!(it.next(), Some(6));
assert_eq!(it.next(), None);

let mut it = (0..).every_nth(3).take(4);
assert_eq!(it.next(), Some(0));
assert_eq!(it.next(), Some(4));
assert_eq!(it.next(), Some(8));
assert_eq!(it.next(), Some(12));
assert_eq!(it.next(), None);
