plain
travis_time:end:028a9697:start=1554736294062221224,finish=1554736296264206350,duration=2201985126
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:26] .................................................................................................... 800/2961
[01:09:36] .................................................................................................... 900/2961
[01:09:51] .................................................................................................... 1000/2961
[01:10:05] .................................................................................................... 1100/2961
[01:10:14] ...............................................................F.................................... 1200/2961
[01:10:36] .................................................................................................... 1400/2961
[01:10:49] .................................................................................................... 1500/2961
[01:10:58] .................................................................................i.................. 1600/2961
[01:11:12] .................................................................................................... 1700/2961
---
[01:14:34] failures:
[01:14:34] 
[01:14:34] ---- [run-pass] run-pass/issues/issue-18353.rs stdout ----
[01:14:34] normalized stderr:
[01:14:34] warning: unused return value of `std::option::Option::<T>::is_some` that must be used
[01:14:34]    |
[01:14:34] LL |     str.is_some();
[01:14:34]    |     ^^^^^^^^^^^^^^
[01:14:34]    |
[01:14:34]    |
[01:14:34]    = note: #[warn(unused_must_use)] on by default
[01:14:34] 
[01:14:34] 
[01:14:34] 
[01:14:34] 
[01:14:34] The actual stderr differed from the expected stderr.
[01:14:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/issue-18353.stderr
[01:14:34] To update references, rerun the tests and pass the `--bless` flag
[01:14:34] To only update this specific test, also pass `--test-args issues/issue-18353.rs`
[01:14:34] error: 1 errors occurred comparing output.
[01:14:34] status: exit code: 0
[01:14:34] status: exit code: 0
[01:14:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18353.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18353/auxiliary"
[01:14:34] ------------------------------------------
[01:14:34] 
[01:14:34] ------------------------------------------
[01:14:34] stderr:
[01:14:34] stderr:
[01:14:34] ------------------------------------------
[01:14:34] {"message":"unused return value of `std::option::Option::<T>::is_some` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/issues/issue-18353.rs","byte_start":234,"byte_end":248,"line_start":14,"line_end":14,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    str.is_some();","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_must_use)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: unused return value of `std::option::Option::<T>::is_some` that must be used\n  --> /checkout/src/test/run-pass/issues/issue-18353.rs:14:5\n   |\nLL |     str.is_some();\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: #[warn(unused_must_use)] on by default\n\n"}
[01:14:34] ------------------------------------------
[01:14:34] 
[01:14:34] thread '[run-pass] run-pass/issues/issue-18353.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:14:34] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:14:34] 
[01:14:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:14:34] 
[01:14:34] 
[01:14:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:34] 
[01:14:34] 
[01:14:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:34] Build completed unsuccessfully in 0:11:06
[01:14:34] Build completed unsuccessfully in 0:11:06
[01:14:34] make: *** [check] Error 1
[01:14:34] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00f03225
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 16:26:21 UTC 2019
---
travis_time:end:1e3e21b4:start=1554740782698621896,finish=1554740782753960234,duration=55338338
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e11d99
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:164cc948
$ dmesg | grep -i kill
