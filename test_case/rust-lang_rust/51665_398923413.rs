
   Compiling playground v0.0.1 (file:///playground)
error: cannot find macro `foo!` in this scope
  --> src/main.rs:10:5
   |
10 |     foo!();
   |     ^^^ not found in this scope
   |
help: include the `foo!` macro from `mod a` in the scope
 --> src/main.rs:1:0
  |
1 | #[macro_use] mod a {
  | ^^^^^^^^^^^^
