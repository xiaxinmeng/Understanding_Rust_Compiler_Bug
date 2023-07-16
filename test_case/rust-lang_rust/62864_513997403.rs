rust
let y = {
  loop {
    match poll(Pin::new_unchecked(&mut x)) { .. }
  }
  drop(x);
};
