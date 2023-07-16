
error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
4 |     a = &panic!();
  |         ^^^^^^^^^ expected `usize`, found `!`
  |
  = note: expected reference `&usize`
             found reference `&!`
