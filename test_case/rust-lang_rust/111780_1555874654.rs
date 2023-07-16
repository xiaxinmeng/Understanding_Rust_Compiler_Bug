plain
---- src/builtin.rs - builtin::UNKNOWN_DIAGNOSTIC_ATTRIBUTE (line 4184) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `diagnostic`
 --> src/builtin.rs:4185:3
  |
3 | #[diagnostic::does_not_exist]
  |   ^^^^^^^^^^ use of undeclared crate or module `diagnostic`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/builtin.rs - builtin::UNKNOWN_DIAGNOSTIC_ATTRIBUTE (line 4184)

test result: FAILED. 98 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 1.15s

error: doctest failed, to rerun pass `-p rustc_lint_defs --doc`
