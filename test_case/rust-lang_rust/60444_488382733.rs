plain
travis_time:end:01587ca2:start=1556733278992615532,finish=1556733279775283181,duration=782667649
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:12:28] .................................................................................................... 4700/5476
[01:12:33] .................................................................................................... 4800/5476
[01:12:36] .................................................................................................... 4900/5476
[01:12:40] .................................................................................................... 5000/5476
[01:12:44] ....................................F............................................................... 5100/5476
[01:12:51] .................................................................................................... 5300/5476
[01:12:53] .................................................................................................... 5400/5476
[01:12:56] ..............i.............................................................
[01:12:56] failures:
[01:12:56] failures:
[01:12:56] 
[01:12:56] ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
[01:12:56] normalized stderr:
[01:12:56] error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
[01:12:56]   --> $DIR/cycle-cache-err-60010.rs:27:5
[01:12:56]    |
[01:12:56] LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data,
[01:12:56]    |
[01:12:56]    |
[01:12:56]    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
[01:12:56] 
[01:12:56] error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
[01:12:56]   --> $DIR/cycle-cache-err-60010.rs:30:6
[01:12:56]    |
[01:12:56] LL | impl Database for RootDatabase {
[01:12:56]    |
[01:12:56]    |
[01:12:56]    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
[01:12:56]    = note: required because it appears within the type `SalsaStorage`
[01:12:56] error: aborting due to 2 previous errors
[01:12:56] 
[01:12:56] For more information about this error, try `rustc --explain E0275`.
[01:12:56] 
[01:12:56] 
[01:12:56] 
[01:12:56] 
[01:12:56] The actual stderr differed from the expected stderr.
[01:12:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
[01:12:56] To update references, rerun the tests and pass the `--bless` flag
[01:12:56] To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`
[01:12:56] error: 1 errors occurred comparing output.
[01:12:56] status: exit code: 1
[01:12:56] status: exit code: 1
[01:12:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
[01:12:56] ------------------------------------------
[01:12:56] 
[01:12:56] ------------------------------------------
[01:12:56] stderr:
[01:12:56] stderr:
[01:12:56] ------------------------------------------
[01:12:56] error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
[01:12:56]   --> /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:27:5
[01:12:56]    |
[01:12:56] LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data,
[01:12:56]    |
[01:12:56]    |
[01:12:56]    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
[01:12:56] 
[01:12:56] error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
[01:12:56]   --> /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:30:6
[01:12:56]    |
[01:12:56] LL | impl Database for RootDatabase {
[01:12:56]    |
[01:12:56]    |
[01:12:56]    = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
[01:12:56]    = note: required because it appears within the type `SalsaStorage`
[01:12:56] error: aborting due to 2 previous errors
[01:12:56] 
[01:12:56] For more information about this error, try `rustc --explain E0275`.
[01:12:56] 
---
[01:12:56] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[01:12:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:56] 
[01:12:56] 
[01:12:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:56] 
[01:12:56] 
[01:12:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:56] Build completed unsuccessfully in 0:04:23
[01:12:56] Build completed unsuccessfully in 0:04:23
[01:12:56] make: *** [check] Error 1
[01:12:56] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18c16d00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 19:07:46 UTC 2019
---
travis_time:end:1b4f988a:start=1556737668306194413,finish=1556737668313158650,duration=6964237
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00c10f95
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01c722fc
travis_time:start:01c722fc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b2e759d
$ dmesg | grep -i kill
