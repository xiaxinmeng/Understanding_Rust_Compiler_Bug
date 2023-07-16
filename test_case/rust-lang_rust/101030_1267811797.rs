plain

failures:

---- panic::location::location_const_file stdout ----
thread 'panic::location::location_const_file' panicked at 'assertion failed: `(left == right)`
  left: `"library\\core\\tests\\panic\\location.rs"`,
 right: `"library/core/tests/panic/location.rs"`', library\core\tests\panic\location.rs:16:5

failures:
    panic::location::location_const_file


test result: FAILED. 1496 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.59s

error: test failed, to rerun pass `-p core --test coretests`
Build completed unsuccessfully in 0:32:23
make: *** [Makefile:73: ci-subset-1] Error 1
