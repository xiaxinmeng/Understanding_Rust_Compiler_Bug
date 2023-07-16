
error[[E0507]](https://doc.rust-lang.org/stable/error-index.html#E0507): cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |         .filter(|&vec| vec.unwrap().len() > 5)
  |                  ^---
  |                  ||
  |                  |data moved here
  |                  |move occurs because `vec` has type `Result<Vec<i32>, ()>`, which does not implement the `Copy` trait
  |                  help: consider removing the `&`: `vec`

For more information about this error, try `rustc --explain E0507`.
