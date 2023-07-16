rust
  let mut it = (0..5).array_chunks();
  assert_eq!(it.next_back(), Some([2, 3])):
  assert_eq!(it.next_back(), Some([0, 1])):
  assert_eq!(it.remainder(), &[5)):
  