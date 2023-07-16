
error[E0308]: mismatched types
 --> src/main.rs:6:13
  |
6 |     for (x, &y) in &u {
  |             ^^     -- this expression has type `Option<&(&u64, S)>`
  |             |
  |             expected struct `S`, found reference
  |             help: you can probably remove the explicit borrow: `y`
  |
  = note: expected struct `S`
          found reference `&_`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `tmp` due to previous error
