plain

   Doc-tests rustc_lint_defs

running 119 tests
ii..i..i........i..........i....iii....i........iii..i.........F........i....F......i..i  88/119
........F......i...............
failures:

---- src/builtin.rs - builtin::PRIVATE_INTERFACES (line 4188) stdout ----
---- src/builtin.rs - builtin::PRIVATE_INTERFACES (line 4188) stdout ----
error[E0412]: cannot find type `SemiPriv` in module `super`
   |
   |
10 |     impl super::SemiPriv {
   |                 ^^^^^^^^ not found in `super`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.
Couldn't compile the test.
---
    src/builtin.rs - builtin::UNNAMEABLE_TYPES (line 4251)

test result: FAILED. 98 passed; 3 failed; 18 ignored; 0 measured; 0 filtered out; finished in 1.14s

error: doctest failed, to rerun pass `-p rustc_lint_defs --doc`
