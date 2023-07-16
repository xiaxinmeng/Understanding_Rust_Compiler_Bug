rust
 % rustc check.rs 
error: entry symbol `main` defined multiple times
 --> check.rs:4:1
  |
4 | fn main(){}
  | ^^^^^^^^^^^
  |
  = help: did you use #[no_mangle] on `fn main`? Use #[start] instead

error: aborting due to previous error
