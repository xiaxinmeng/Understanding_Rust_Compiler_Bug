plain

failures:

---- create_dir_all_bare stdout ----
thread 'create_dir_all_bare' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: Uncategorized, message: "The process cannot access the file because it is being used by another process." }', library\std\tests\common\mod.rs:46:20


failures:
    create_dir_all_bare
    create_dir_all_bare

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p std --test create_dir_all_bare`
Build completed unsuccessfully in 0:38:04
make: *** [Makefile:68: ci-subset-1] Error 1
