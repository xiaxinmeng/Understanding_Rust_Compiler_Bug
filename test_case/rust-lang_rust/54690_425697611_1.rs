console
error[E0119]: conflicting implementations of trait `Trait` for type `&()`:
 --> src/lib.rs:3:1
  |
2 | impl Trait for &() {}
  | ------------------ first implementation here
3 | impl Trait for &'static () {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&()`
