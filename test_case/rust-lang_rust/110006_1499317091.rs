plain

failures:

---- config::tests::detect_src_and_out stdout ----
thread 'config::tests::detect_src_and_out' panicked at 'assertion failed: `(left == right)`
  left: `"C:\\tmp"`,
 right: `"/tmp"`', config\tests.rs:57:13

failures:
    config::tests::detect_src_and_out


test result: FAILED. 24 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 66.15s

error: test failed, to rerun pass `--lib`
Build completed unsuccessfully in 1:08:11
make: *** [Makefile:68: ci-subset-1] Error 1
