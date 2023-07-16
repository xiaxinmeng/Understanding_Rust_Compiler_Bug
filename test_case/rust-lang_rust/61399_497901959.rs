plain
travis_time:end:24a73e2e:start=1559348876747567560,finish=1559348975741547693,duration=98993980133
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:59:38] .............................................................................i...................... 200/2924
[00:59:47] .................................................................................................... 300/2924
[00:59:58] .................................................................................................... 400/2924
[01:00:07] .................................................................................................... 500/2924
[01:00:18] ...............................................F.................................................... 600/2924
[01:00:44] .................................................................................................... 800/2924
[01:00:53] .................................................................................................... 900/2924
[01:01:07] .................................................................................................... 1000/2924
[01:01:19] .................................................................................................... 1100/2924
---
[01:05:35] ------------------------------------------
[01:05:35] stderr:
[01:05:35] ------------------------------------------
[01:05:35] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:05:35]   left: `"&\'static str"`,
[01:05:35]  right: `"const_fn_type_name::Struct<i8, f64, bool>"`', /checkout/src/test/run-pass/ctfe/const-fn-type-name.rs:36:5
[01:05:35] 
[01:05:35] ------------------------------------------
[01:05:35] 
[01:05:35] 
---
[01:05:35] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:05:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:05:35] 
[01:05:35] 
[01:05:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:35] 
[01:05:35] 
[01:05:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:35] Build completed unsuccessfully in 1:01:18
---
travis_time:end:14ab1c0f:start=1559352921937355544,finish=1559352921942133706,duration=4778162
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07d521d7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03866620
travis_time:start:03866620
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02370a36
$ dmesg | grep -i kill
