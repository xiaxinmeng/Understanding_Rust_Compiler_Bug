
error[E0308]: mismatched types
 --> src/main.rs:6:9
  |
6 |     a = 1;
  |         ^ expected struct `std::sync::MutexGuard`, found integer
  |
  = note: expected type `std::sync::MutexGuard<'_, i32>`
             found type `{integer}`
