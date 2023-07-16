
error[E0368]: binary assignment operation `+=` cannot be applied to type `std::sync::MutexGuard<'_, {integer}>`
 --> src/main.rs:6:5
  |
6 |     a += 1;
  |     -^^^^^
  |     |
  |     cannot use `+=` on type `std::sync::MutexGuard<'_, {integer}>`
  |
  = note: an implementation of `std::ops::AddAssign` might be missing for `std::sync::MutexGuard<'_, {integer}>`
