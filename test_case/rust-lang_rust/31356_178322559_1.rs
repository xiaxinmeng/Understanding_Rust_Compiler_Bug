 rust
fn test() {
  // I got this when I ran it the first time, shouldn't change!
  let expected = vec![1, 3, 5, 6, 11, 13];
  let map = HashMap::new();
  for k in get_test_input() {
    map.insert(k, compute(k));
  }
  let result = map.values().cloned().collect();
  assert_eq!(result, expected);
}
