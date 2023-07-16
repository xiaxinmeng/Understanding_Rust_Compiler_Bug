plain
............ii................................................F.........

failures:

---- src/types.rs - types::INVALID_NAN_COMPARISONS (line 121) stdout ----
warning: incorrect NaN comparison, NaN cannot be directly compared to itself
  |
  |
4 | if a == f32::NAN {}
  |
  = note: `#[warn(invalid_nan_comparisons)]` on by default
  = note: `#[warn(invalid_nan_comparisons)]` on by default
help: use `f32::is_nan()` or `f64::is_nan()` instead
  |
4 - if a == f32::NAN {}
4 + if a.is_nan() {}

warning: 1 warning emitted

Test compiled successfully, but it's marked `compile_fail`.
Test compiled successfully, but it's marked `compile_fail`.

failures:
    src/types.rs - types::INVALID_NAN_COMPARISONS (line 121)

test result: FAILED. 69 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 3.16s

error: doctest failed, to rerun pass `-p rustc_lint --doc`
