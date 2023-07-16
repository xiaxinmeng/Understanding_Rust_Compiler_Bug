
error: unsatisfied lifetime constraints
 --> src/main.rs:5:8
  |
5 |     || &mut v;
  |     -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
  |     ||
  |     |return type of closure is &'2 mut std::vec::Vec<()>
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `FnMut`, so references to captured variables can't escape the closure
