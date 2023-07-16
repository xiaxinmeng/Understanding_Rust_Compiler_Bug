
error[E0594]: cannot assign to `t.v` which is behind a `&` reference
  --> src/main.rs:12:9
   |
11 |     for mut t in map.values() {
   |                  ------------
   |                  |   |
   |                  |   help: use mutable method: `values_mut()`
   |                  this iterator yields `&` references
12 |         t.v += 1;
   |         ^^^^^^^^ `t` is a `&` reference, so the data it refers to cannot be written
