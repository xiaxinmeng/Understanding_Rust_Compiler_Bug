plain
[01:21:14] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:14]    Compiling rustc_driver v0.0.0 (file:///checkout/src/librustc_driver)
[01:21:15] error[E0423]: expected function, found struct `ty::DebruijnIndex`
[01:21:15]    --> librustc_driver/test.rs:187:31
[01:21:15]     |
[01:21:15] 187 | const D2: ty::DebruijnIndex = ty::DebruijnIndex(1);
[01:21:15]     |                               ^^^^^^^^^^^^^^^^^ did you mean `ty::DebruijnIndex { /* fields */ }`?
[01:21:16] error: aborting due to previous error
[01:21:16] 
[01:21:16] For more information about this error, try `rustc --explain E0423`.
[01:21:16] error: Could not compile `rustc_driver`.
[01:21:16] error: Could not compile `rustc_driver`.
[01:21:16] 
[01:21:16] To learn more, run the command again with --verbose.
[01:21:16] 
[01:21:16] 
[01:21:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:21:16] 
[01:21:16] 
[01:21:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:16] Build completed unsuccessfully in 0:38:05
[01:21:16] Build completed unsuccessfully in 0:38:05
[01:21:16] Makefile:58: recipe for target 'check' failed
[01:21:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20392e06
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04d86f68:start=1529000091602002402,finish=1529000091609688778,duration=7686376
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00150318
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dc55a0a
$ dmesg | grep -i kill
