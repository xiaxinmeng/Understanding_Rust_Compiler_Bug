plain
test fs::tests::file_test_io_seek_and_write ... ok
test fs::tests::file_test_io_seek_read_write ... ok
test fs::tests::file_test_iounlinking_invalid_path_should_raise_condition ... ok
test fs::tests::file_test_io_seek_shakedown ... ok
test fs::tests::hiberfil_sys ... FAILED
test fs::tests::file_test_io_smoke_test ... ok
test fs::tests::file_try_clone ... ok
test fs::tests::invalid_path_raises ... ok
test fs::tests::file_test_stat_is_correct_on_is_file ... ok
---
test fs::tests::read_large_dir ... ok

failures:

---- fs::tests::hiberfil_sys stdout ----
thread 'fs::tests::hiberfil_sys' panicked at 'assertion failed: `(left == right)`
  left: `true`,
 right: `false`', library\std\src\fs\tests.rs:1546:5

failures:
failures:
    fs::tests::hiberfil_sys
test result: FAILED. 902 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 74.60s


error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:46:44
Build completed unsuccessfully in 0:46:44
make: *** [Makefile:70: ci-subset-1] Error 1
