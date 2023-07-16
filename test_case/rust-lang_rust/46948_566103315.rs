
running 3 tests
test tests::test_mul_add ... ok
test tests::f64_test_float_bits_conv ... FAILED
test tests::f32_test_float_bits_conv ... FAILED

failures:

---- tests::f64_test_float_bits_conv stdout ----
thread 'tests::f64_test_float_bits_conv' panicked at 'assertion failed: `(left == right)`
  left: `9221870836978985642`,
 right: `9219619037165300394`', src\lib.rs:70:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

---- tests::f32_test_float_bits_conv stdout ----
thread 'tests::f32_test_float_bits_conv' panicked at 'assertion failed: `(left == right)`
  left: `2144687445`,
 right: `2140493141`', src\lib.rs:49:9


failures:
    tests::f32_test_float_bits_conv
    tests::f64_test_float_bits_conv

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
