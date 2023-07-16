plain
running 24 tests
.......i.......F........
failures:

---- src/unord.rs - unord::UnordItems (line 25) stdout ----
error[E0425]: cannot find value `unordered_items` in this scope
 --> src/unord.rs:27:1
  |
4 | unordered_items.all(|x| ordered.push(x));

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    src/unord.rs - unord::UnordItems (line 25)
test result: FAILED. 22 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.57s


error: doctest failed, to rerun pass `-p rustc_data_structures --doc`
