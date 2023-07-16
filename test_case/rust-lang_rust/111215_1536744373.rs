plain
iiiiiiii.F...

failures:

---- src/late.rs - late::NoConstantGenericsReason::IsEnumDiscriminant (line 139) stdout ----
error: generic parameters may not be used in enum discriminant values
  |
  |
4 |     Variant = { N }, // this anon const is not allowed to use generics
  |                 ^ cannot perform const operation using `N`
  = note: const parameters may not be used in enum discriminant values

error: aborting due to previous error


Couldn't compile the test.

failures:
    src/late.rs - late::NoConstantGenericsReason::IsEnumDiscriminant (line 139)
test result: FAILED. 4 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out; finished in 178.12ms


error: doctest failed, to rerun pass `-p rustc_resolve --doc`
