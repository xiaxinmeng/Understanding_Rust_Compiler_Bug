plain
test util::lev_distance::test_lev_distance ... ok

failures:

---- core::source::source_id::test_cratesio_hash stdout ----
thread 'core::source::source_id::test_cratesio_hash' panicked at 'assertion failed: `(left == right)`
  left: `"5f0049709550bc18"`,
 right: `"1ecc6299db9ec823"`', src/tools/cargo/src/cargo/core/source/source_id.rs:634:5


failures:
    core::source::source_id::test_cratesio_hash
    core::source::source_id::test_cratesio_hash

test result: FAILED. 46 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s

error: test failed, to rerun pass '--lib'
Build completed unsuccessfully in 0:23:33
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
