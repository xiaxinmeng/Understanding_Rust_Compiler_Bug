plain
travis_time:end:0cb87e51:start=1561309219599806964,finish=1561309222495434599,duration=2895627635
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:27] 
[01:06:27] running 9 tests
[01:06:27] iiiiiiiii
[01:06:27] 
[01:06:27]  finished in 0.154
[01:06:27] travis_fold:end:test_assembly


[01:06:27] travis_time:end:test_assembly:start=1561313220616724513,finish=1561313220771425133,duration=154700620

[01:06:27] travis_fold:start:test_incremental
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:27] 
[01:06:27] running 104 tests
[01:06:42] ............................................FF...................................................... 100/104
[01:06:43] failures:
[01:06:43] 
[01:06:43] ---- [incremental] incremental/hashes/while_let_loops.rs stdout ----
[01:06:43] 
[01:06:43] 
[01:06:43] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:43] status: exit code: 1
[01:06:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
[01:06:43] ------------------------------------------
[01:06:43] 
[01:06:43] ------------------------------------------
[01:06:43] stderr:
[01:06:43] stderr:
[01:06:43] ------------------------------------------
[01:06:43] error: `typeck_tables_of(change_break_label)` should be dirty but is not
[01:06:43]    |
[01:06:43]    |
[01:06:43] LL | / pub fn change_break_label() {
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     'outer: while let Some(0u32) = None {
[01:06:43] LL | |         'inner: while let Some(0u32) = None {
[01:06:43] LL | |     }
[01:06:43] LL | | }
[01:06:43]    | |_^
[01:06:43] 
[01:06:43] 
[01:06:43] error: `typeck_tables_of(change_continue_label)` should be dirty but is not
[01:06:43]    |
[01:06:43] LL | / pub fn change_continue_label() {
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     'outer: while let Some(0u32) = None {
[01:06:43] LL | |         'inner: while let Some(0u32) = None {
[01:06:43] LL | |     }
[01:06:43] LL | | }
[01:06:43]    | |_^
[01:06:43] 
---
[01:06:43] 
[01:06:43] 
[01:06:43] ---- [incremental] incremental/hashes/while_loops.rs stdout ----
[01:06:43] 
[01:06:43] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:06:43] status: exit code: 1
[01:06:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/auxiliary"
[01:06:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:06:43] ------------------------------------------
[01:06:43] 
[01:06:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:43] ------------------------------------------
[01:06:43] stderr:
[01:06:43] ------------------------------------------
[01:06:43] error: `typeck_tables_of(change_break_label)` should be dirty but is not
[01:06:43]    |
[01:06:43]    |
[01:06:43] LL | / pub fn change_break_label() {
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     'outer: while true {
[01:06:43] LL | |         'inner: while true {
[01:06:43] LL | |     }
[01:06:43] LL | | }
[01:06:43]    | |_^
[01:06:43] 
[01:06:43] 
[01:06:43] error: `typeck_tables_of(change_continue_label)` should be dirty but is not
[01:06:43]    |
[01:06:43] LL | / pub fn change_continue_label() {
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     let mut _x = 0;
[01:06:43] LL | |     'outer: while true {
[01:06:43] LL | |         'inner: while true {
[01:06:43] LL | |     }
[01:06:43] LL | | }
[01:06:43]    | |_^
[01:06:43] 
---
[01:06:43] test result: FAILED. 102 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:43] 
[01:06:43] 
[01:06:43] 
[01:06:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:43] 
[01:06:43] 
[01:06:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:43] Build completed unsuccessfully in 1:01:50
