 rust
[1, 2, 3, 4].iter().filter(|&i| i %2 == 0).size_hint()  // => (0, Some(4))
[1, 2, 3, 4].iter().filter(|&i| i %2 == 0).optimistic().size_hint()  // => (4, Some(4))
