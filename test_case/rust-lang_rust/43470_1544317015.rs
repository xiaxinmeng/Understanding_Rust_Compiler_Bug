sh
cargo run
   Compiling test_43470 v0.1.0 (/home/michal/projects/tests/test_43470)
error[E0499]: cannot borrow `parser` as mutable more than once at a time
  --> src/main.rs:19:29
   |
19 |     while let Some(token) = parser.next() {
   |                             ^^^^^^^^^^^^^ `parser` was mutably borrowed here in the previous iteration of the loop
20 |         tokens.push(token)
   |         ------------------ first borrow used here, in later iteration of loop

For more information about this error, try `rustc --explain E0499`.
error: could not compile `test_43470` due to previous error
