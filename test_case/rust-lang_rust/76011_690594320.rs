
error[E0658]: async closures are unstable
 --> src/main.rs:2:21
  |
2 |     let my_future = async || {
  |                     ^^^^^
  |
  = note: see issue #62290 <https://github.com/rust-lang/rust/issues/62290> for more information
  = help: to use an async block, remove the `||`: `async

error: aborting due to previous error
