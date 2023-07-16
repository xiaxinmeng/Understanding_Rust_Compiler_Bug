rust
let _ = || {
  let mut count = 0usize;
  loop {
    count += 1; // <-- What variant should we use?
    if count % 2 == 0 {
      yield 2;  // Suspend0
    }
    yield count;  // Suspend1
  }
}
