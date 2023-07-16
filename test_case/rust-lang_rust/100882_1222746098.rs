rust
error[E0283]: type annotations needed
  --> src/main.rs:13:25
   |
13 |     if vec.len() != len.parse().unwrap() {
   |                  --     ^^^^^ cannot infer type of the type parameter `F` declared on the associated function `parse`
   |                  |
   |                  type must be known at this point
   |
   = note: multiple `impl`s satisfying `usize: PartialEq<_>` found in the following crates: `core`, `serde_json`:
           - impl PartialEq for usize;
           - impl PartialEq<Value> for usize;
help: consider specifying the generic argument
   |
13 |     if vec.len() != len.parse::<F>().unwrap() {
   |                              +++++

For more information about this error, try `rustc --explain E0283`.
