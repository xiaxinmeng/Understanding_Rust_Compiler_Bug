plain
test fs::tests::read_large_dir ... ok

failures:

---- net::tcp::tests::nodelay stdout ----
thread 'net::tcp::tests::nodelay' panicked at 'assertion failed: `(left == right)`
 right: `4`', library\std\src\sys_common\net.rs:74:9


failures:
failures:
    net::tcp::tests::nodelay

test result: FAILED. 902 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 44.49s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:51:47
make: *** [Makefile:72: ci-subset-1] Error 1
