plain
travis_time:end:1ab9ce60:start=1556896367533439355,finish=1556896369597919973,duration=2064480618
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=compile-fail mode=compile-fail (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:18] 
[01:21:18] running 30 tests
[01:21:19] ERROR 2019-05-03T16:34:19Z: compiletest::runtest: fatal error, panic: "no error pattern specified in \"/checkout/src/test/compile-fail/meta-expected-error-wrong-rev.rs\""
[01:21:19] ................F.............
[01:21:19] 
[01:21:19] ---- [compile-fail] compile-fail/issue-43733-2.rs stdout ----
[01:21:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:21:19] 
[01:21:19] 
[01:21:19] error: /checkout/src/test/compile-fail/issue-43733-2.rs:24: unexpected error: '24:1: 24:36: `std::cell::Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely [E0277]'
[01:21:19] 
[01:21:19] error: /checkout/src/test/compile-fail/issue-43733-2.rs:24: expected error not found: `std::cell::Cell<bool>` cannot be shared between threads safely [E0277]
[01:21:19] error: 1 unexpected errors found, 1 expected errors not found
[01:21:19] status: exit code: 1
[01:21:19] status: exit code: 1
[01:21:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-43733-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43733-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-43733-2/auxiliary" "-A" "unused"
[01:21:19]     Error {
[01:21:19]         line_num: 24,
[01:21:19]         kind: Some(
[01:21:19]             Error,
[01:21:19]             Error,
[01:21:19]         ),
[01:21:19]         msg: "24:1: 24:36: `std::cell::Cell<std::thread::local::fast::DtorState>` cannot be shared between threads safely [E0277]",
[01:21:19] ]
[01:21:19] 
[01:21:19] not found errors (from test file): [
[01:21:19]     Error {
[01:21:19]     Error {
[01:21:19]         line_num: 24,
[01:21:19]         kind: Some(
[01:21:19]             Error,
[01:21:19]         ),
[01:21:19]         msg: "`std::cell::Cell<bool>` cannot be shared between threads safely [E0277]",
[01:21:19] ]
[01:21:19] 
[01:21:19] thread '[compile-fail] compile-fail/issue-43733-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1402:13
[01:21:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:21:19] test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:21:19] 
[01:21:19] 
[01:21:19] 
[01:21:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:19] 
[01:21:19] 
[01:21:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:19] Build completed unsuccessfully in 0:10:40
[01:21:19] Build completed unsuccessfully in 0:10:40
[01:21:19] Makefile:48: recipe for target 'check' failed
[01:21:19] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:079a1c33
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 16:34:20 UTC 2019
