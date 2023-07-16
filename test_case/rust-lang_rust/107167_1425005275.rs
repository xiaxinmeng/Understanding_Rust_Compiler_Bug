plain
test fs::tests::read_large_dir ... ok

failures:

---- fs::tests::create_dir_all_with_junctions stdout ----
thread 'fs::tests::create_dir_all_with_junctions' panicked at 'symlink_junction(&target, &junction) failed with: The data present in the reparse point buffer is invalid. (os error 4392)', library\std\src\fs\tests.rs:1381:5
---- fs::tests::recursive_rmdir stdout ----
---- fs::tests::recursive_rmdir stdout ----
thread 'fs::tests::recursive_rmdir' panicked at 'symlink_junction(&d2, &dt.join("d2")) failed with: The data present in the reparse point buffer is invalid. (os error 4392)', library\std\src\fs\tests.rs:583:5
---- fs::tests::recursive_rmdir_of_symlink stdout ----
---- fs::tests::recursive_rmdir_of_symlink stdout ----
thread 'fs::tests::recursive_rmdir_of_symlink' panicked at 'symlink_junction(&dir, &link) failed with: The data present in the reparse point buffer is invalid. (os error 4392)', library\std\src\fs\tests.rs:600:5

failures:
    fs::tests::create_dir_all_with_junctions
    fs::tests::recursive_rmdir
    fs::tests::recursive_rmdir
    fs::tests::recursive_rmdir_of_symlink

test result: FAILED. 923 passed; 3 failed; 4 ignored; 0 measured; 0 filtered out; finished in 73.75s

error: test failed, to rerun pass `-p std --lib`
Build completed successfully in 0:39:35
make: *** [Makefile:68: ci-subset-1] Error 1
