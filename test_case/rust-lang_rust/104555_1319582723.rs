plain
test str::utf8_char_counts ... ok

failures:

---- rc::rc_from_vec_opt stdout ----
thread 'rc::rc_from_vec_opt' panicked at 'assertion failed: `(left == right)`
  left: `9568`,
 right: `16`: Vector allocation not reused', library\alloc\tests\rc.rs:217:9

failures:
    rc::rc_from_vec_opt


test result: FAILED. 652 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.18s

error: test failed, to rerun pass `-p alloc --test collectionstests`
Build completed unsuccessfully in 0:48:25
make: *** [Makefile:83: ci-mingw-subset-1] Error 1
