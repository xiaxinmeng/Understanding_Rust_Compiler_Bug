plain
.......................................................

failures:

---- f32::tests::test_total_cmp stdout ----
thread 'f32::tests::test_total_cmp' panicked at 'assertion failed: `(left == right)`
  left: `Less`,
 right: `Greater`', library/std/src/f32/tests.rs:781:5
---- f64::tests::test_total_cmp stdout ----
---- f64::tests::test_total_cmp stdout ----
thread 'f64::tests::test_total_cmp' panicked at 'assertion failed: `(left == right)`
  left: `Less`,
 right: `Greater`', library/std/src/f64/tests.rs:763:5

failures:
    f32::tests::test_total_cmp
    f64::tests::test_total_cmp
    f64::tests::test_total_cmp

test result: FAILED. 932 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out; finished in 13.51s

error: test failed, to rerun pass `-p std --lib`
