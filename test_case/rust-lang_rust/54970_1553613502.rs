sh
cargo run
   Compiling test_playground v0.1.0 (/home/michal/projects/tests/test_playground)
warning: associated type `not_really_ambiguous_i_promise` should have an upper camel case name
 --> src/main.rs:2:10
  |
2 |     type not_really_ambiguous_i_promise;
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `NotReallyAmbiguousIPromise`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

error[E0223]: ambiguous associated type
  --> src/main.rs:10:12
   |
10 |     let x: isize::not_really_ambiguous_i_promise = 17;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the fully-qualified path: `<isize as Foo>::not_really_ambiguous_i_promise`

For more information about this error, try `rustc --explain E0223`.
warning: `test_playground` (bin "test_playground") generated 1 warning
error: could not compile `test_playground` (bin "test_playground") due to previous error; 1 warning emitted
