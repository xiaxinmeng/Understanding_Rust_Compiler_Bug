plain

   Doc-tests rustc_lint_defs

running 119 tests
ii..i..i........i........i.ii..i.......i........iii..i..FF..............i..........Fi..i  88/119

failures:

---- src/builtin.rs - builtin::PRIVATE_INTERFACES (line 4188) stdout ----
---- src/builtin.rs - builtin::PRIVATE_INTERFACES (line 4188) stdout ----
error: private type `Priv` in public interface
   |
8  |     struct Priv;
8  |     struct Priv;
   |     ----------- `Priv` declared as private
9  |     impl crate::SemiPriv {
10 |         pub fn f(_: Priv) {}
   |         ^^^^^^^^^^^^^^^^^ can't leak private type
note: the lint level is defined here
  --> src/builtin.rs:4191:9
   |
   |
4  | #![deny(private_interfaces)]

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- src/builtin.rs - builtin::PRIVATE_BOUNDS (line 4225) stdout ----
error: private type `PrivTy` in public interface
  |
6 | struct PrivTy;
6 | struct PrivTy;
  | ------------- `PrivTy` declared as private
  | ^^^^^^^^^^^^ can't leak private type
  |
note: the lint level is defined here
 --> src/builtin.rs:4228:9
 --> src/builtin.rs:4228:9
  |
4 | #![deny(private_bounds)]

error: aborting due to previous error

Couldn't compile the test.
---
    src/builtin.rs - builtin::UNNAMEABLE_TYPES (line 4255)

test result: FAILED. 98 passed; 3 failed; 18 ignored; 0 measured; 0 filtered out; finished in 1.09s

error: doctest failed, to rerun pass `-p rustc_lint_defs --doc`
