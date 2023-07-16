
warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
 --> src/main.rs:8:32
  |
8 | fn return_a_guard() -> Box<dyn Drop> {
  |                                ^^^^
  |
  = note: `#[warn(dyn_drop)]` on by default
