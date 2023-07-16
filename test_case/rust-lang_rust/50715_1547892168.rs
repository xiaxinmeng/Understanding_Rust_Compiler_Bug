sh
cargo run
   Compiling test_playground v0.1.0 (/home/michal/projects/tests/test_playground)
error[E0283]: type annotations needed: cannot satisfy `M: A`
 --> src/main.rs:6:12
  |
6 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  |            ^
  |
note: multiple `impl`s satisfying `M: A` found
 --> src/main.rs:6:1
  |
6 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  | ^^^^^^^^^^^^
7 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  | ^^^^^^^^^^^^

error[E0283]: type annotations needed: cannot satisfy `M: A`
 --> src/main.rs:7:12
  |
7 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  |            ^
  |
note: multiple `impl`s satisfying `M: A` found
 --> src/main.rs:6:1
  |
6 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  | ^^^^^^^^^^^^
7 | impl A for M where for<'a> M: B<'a> { /* ... */ }
  | ^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0283`.
error: could not compile `test_playground` (bin "test_playground") due to 2 previous errors
