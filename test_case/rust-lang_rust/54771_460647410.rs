
error[E0308]: mismatched types
 --> src/main.rs:6:12
  |
6 |     bar(|| { 5u8; })
  |            ^^^^^^^^ expected u32, found ()
  |
  = note: expected type `u32`
             found type `()`
