 rust
error[E0309]: the parameter type `T` may not live long enough
  --> src/lib.rs:45:5
   |
45 |     fn leak<'a>(mut self) -> &'a [T] where [T]: 'a {
   |     ^
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the type `[T]` will meet its required lifetime bounds
  --> src/lib.rs:45:5
   |
45 |     fn leak<'a>(mut self) -> &'a [T] where [T]: 'a {
   |     ^
