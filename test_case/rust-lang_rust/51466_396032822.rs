rust
let mut borrow = c.borrow_mut();
let (begin, end) = borrow.split_at_mut(2);
assert_eq!(*begin, [1, 2]);
assert_eq!(*end, [3, 4]);
begin.copy_from_slice(&[4, 3]);
end.copy_from_slice(&[2, 1]);
