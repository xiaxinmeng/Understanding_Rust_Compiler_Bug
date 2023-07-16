 rust
let Tup(.._) = Tup(1, false);
let (first, second, ..middle, last) = (1, "hi", false, 2, 3.0);
assert_eq!(first, 1);
assert_eq!(second, "hi");
assert_eq!(middle, (false, 2));
assert_eq!(last, 3.0)
