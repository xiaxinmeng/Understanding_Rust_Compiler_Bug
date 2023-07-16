plain
travis_time:end:28cc7da5:start=1561080575161169292,finish=1561080664852383195,duration=89691213903
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:40] 
[00:56:40] running 2922 tests
[00:56:51] .................................................................................................... 100/2922
[00:57:03] .................................................F..........................i....................... 200/2922
[00:57:23] .................................................................................................... 400/2922
[00:57:32] .................................................................................................... 500/2922
[00:57:43] .................................................................................................... 600/2922
[00:57:58] .................................................................................................... 700/2922
---
[01:03:02] failures:
[01:03:02] 
[01:03:02] ---- [run-pass] run-pass/async-fn-size-moved-locals.rs stdout ----
[01:03:02] 
[01:03:02] error: test compilation failed although it shouldn't!
[01:03:02] status: exit code: 1
[01:03:02] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/async-fn-size-moved-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-fn-size-moved-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/async-fn-size-moved-locals/auxiliary"
[01:03:02] ------------------------------------------
[01:03:02] 
[01:03:02] ------------------------------------------
[01:03:02] stderr:
[01:03:02] stderr:
[01:03:02] ------------------------------------------
[01:03:02] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:52:1
[01:03:02]    |
[01:03:02] LL | async fn single() {
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:57:1
[01:03:02]    |
[01:03:02] LL | async fn single_with_noop() {
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:63:1
[01:03:02]    |
[01:03:02] LL | async fn joined() {
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0670]: `async fn` is not permitted in the 2015 edition
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:76:1
[01:03:02]    |
[01:03:02] LL | async fn joined_with_noop() {
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0609]: no field `await` on type `BigFut`
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:54:7
[01:03:02]    |
[01:03:02] LL |     x.await;
[01:03:02]    |
[01:03:02]    = note: available fields are: `0`
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0609]: no field `await` on type `BigFut`
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:60:7
[01:03:02]    |
[01:03:02] LL |     x.await;
[01:03:02]    |
[01:03:02]    = note: available fields are: `0`
[01:03:02] 
[01:03:02] 
[01:03:02] error[E0609]: no field `await` on type `Joiner`
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:73:12
[01:03:02]    |
[01:03:02] LL |     joiner.await
[01:03:02]    |
[01:03:02]    |
[01:03:02]    = note: available fields are: `a`, `b`, `c`
[01:03:02] 
[01:03:02] error[E0609]: no field `await` on type `Joiner`
[01:03:02]   --> /checkout/src/test/run-pass/async-fn-size-moved-locals.rs:87:12
[01:03:02]    |
[01:03:02] LL |     joiner.await
[01:03:02]    |
[01:03:02]    |
[01:03:02]    = note: available fields are: `a`, `b`, `c`
[01:03:02] error: aborting due to 8 previous errors
[01:03:02] 
[01:03:02] Some errors have detailed explanations: E0609, E0670.
[01:03:02] For more information about an error, try `rustc --explain E0609`.
---
[01:03:02] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:03:02] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:03:02] 
[01:03:02] 
[01:03:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:02] 
[01:03:02] 
[01:03:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:02] Build completed unsuccessfully in 0:59:03
---
156504 ./src/llvm-project/clang
150156 ./obj/build/bootstrap/debug/incremental
145036 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools
134688 ./obj/build/bootstrap/debug/incremental/bootstrap-1llt3ypt1ftzv
134684 ./obj/build/bootstrap/debug/incremental/bootstrap-1llt3ypt1ftzv/s-fdcum8p00l-azhk3x-2adl2f17sobif
118076 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
108532 ./src/llvm-project/lldb
98116 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
97592 ./src/llvm-project/clang/test
---
travis_time:end:04356d38:start=1561084457058468223,finish=1561084457064196197,duration=5727974
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f827862
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b72d434
travis_time:start:0b72d434
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d8c4e00
$ dmesg | grep -i kill
