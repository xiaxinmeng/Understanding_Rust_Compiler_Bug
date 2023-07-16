
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> test-region.rs:3:17
  |
3 | fn example() -> impl Generator {
  |                 ^^^^^^^^^^^^^^
  |
  = note: hidden type `[generator@test-region.rs:4:5: 4:20 for<'r> {i32, &'r i32, ()}]` captures BUG[ReEmpty]
