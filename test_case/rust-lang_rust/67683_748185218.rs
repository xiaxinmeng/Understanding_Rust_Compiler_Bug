
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut v = vec![1, 2, 3, 4, 5, 6, 7];
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[E0594]: cannot assign to `chunk[_]` which is behind a `&` reference
 --> src/main.rs:5:9
  |
4 |     for chunk in &mut chunks {
  |                  ----------- this iterator yields `&` references
5 |         chunk[0] += 1;
  |         ^^^^^^^^^^^^^ `chunk` is a `&` reference, so the data it refers to cannot be written
