
failures:

---- ../liballoc/src/lib.rs - (line 58) stdout ----
error[E0425]: cannot find value `x` in this scope
 --> ../liballoc/src/lib.rs:59:7
  |
4 | match x {}
  |       ^ not found in this scope

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    ../liballoc/src/lib.rs - (line 58)

test result: FAILED. 581 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 70.29s

error: test failed, to rerun pass '--doc'
