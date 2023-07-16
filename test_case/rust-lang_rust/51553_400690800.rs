plain
[01:12:34] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:34]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:12:47] error[E0599]: no method named `is_unnamed` found for type `sys_common::unixsocket::SocketAddr` in the current scope
[01:12:47]     |
[01:12:47]     |
[01:12:47] 909 |         assert!(addr.is_unnamed());
[01:12:47]     | 
[01:12:47]     | 
[01:12:47]    ::: libstd/sys_common/unixsocket.rs:42:1
[01:12:47]     |
[01:12:47] 42  | pub struct SocketAddr(pub(crate) inner::SocketAddr);
[01:12:47]     | ---------------------------------------------------- method `is_unnamed` not found for this
[01:12:47] 
[01:12:47] error[E0599]: no method named `is_unnamed` found for type `sys_common::unixsocket::SocketAddr` in the current scope
[01:12:47]     |
[01:12:47]     |
[01:12:47] 930 |         assert!(addr.is_unnamed());
[01:12:47]     | 
[01:12:47]     | 
[01:12:47]    ::: libstd/sys_common/unixsocket.rs:42:1
[01:12:47]     |
[01:12:47] 42  | pub struct SocketAddr(pub(crate) inner::SocketAddr);
[01:12:47]     | ---------------------------------------------------- method `is_unnamed` not found for this
[01:12:47] error: aborting due to 2 previous errors
[01:12:47] 
[01:12:47] For more information about this error, try `rustc --explain E0599`.
[01:12:47] error: Could not compile `std`.
[01:12:47] error: Could not compile `std`.
[01:12:47] 
[01:12:47] To learn more, run the command again with --verbose.
[01:12:47] 
[01:12:47] 
[01:12:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:12:47] 
[01:12:47] 
[01:12:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:47] Build completed unsuccessfully in 0:31:04
[01:12:47] Build completed unsuccessfully in 0:31:04
[01:12:47] Makefile:58: recipe for target 'check' failed
[01:12:47] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00df796a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02fb0519:start=1530109518356803412,finish=1530109518365194565,duration=8391153
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:022906c0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01f41dc0
$ dmesg | grep -i kill
