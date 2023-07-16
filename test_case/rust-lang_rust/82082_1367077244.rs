
error[E0594]: cannot assign to `v.v`, which is behind a `&` reference
  --> src/main.rs:12:9
   |
11 |     for (k, v) in map.iter() {
   |                   ----------
   |                   |   |
   |                   |   help: use mutable method: `iter_mut()`
   |                   this iterator yields `&` references
12 |         v.v += 1;
   |         ^^^^^^^^ `v` is a `&` reference, so the data it refers to cannot be written
