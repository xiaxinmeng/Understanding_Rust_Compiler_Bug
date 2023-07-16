
error[E0310]: the parameter type `impl Iterator<Item = u32> + Send` may not live long enough
 --> src/main.rs:2:5
  |
2 |     Box::new(x)
  |     ^^^^^^^^^^^
  |
note: ...so that the type `impl Iterator<Item = u32> + Send` will meet its required lifetime bounds
 --> src/main.rs:2:5
  |
2 |     Box::new(x)
  |     ^^^^^^^^^^^
help: consider adding an explicit lifetime bound  `'static` to `impl Iterator<Item = u32> + Send`...
  |
1 | fn wot(x: impl Iterator<Item = u32> + Send + 'static) -> Box<Iterator<Item = u32> + Send> {
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
