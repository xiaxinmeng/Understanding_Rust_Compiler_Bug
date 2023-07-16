plain
test fs::tests::read_large_dir ... ok

failures:

---- f32::tests::test_gamma stdout ----
thread 'f32::tests::test_gamma' panicked at 'assertion failed: (-1.0f32).gamma().is_nan()', library\std\src\f32\tests.rs:565:5

---- f64::tests::test_gamma stdout ----
thread 'f64::tests::test_gamma' panicked at 'assertion failed: (-1.0f64).gamma().is_nan()', library\std\src\f64\tests.rs:550:5

failures:
    f32::tests::test_gamma
    f64::tests::test_gamma
    f64::tests::test_gamma

test result: FAILED. 904 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 52.68s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:41:58
Build completed unsuccessfully in 0:41:58
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
