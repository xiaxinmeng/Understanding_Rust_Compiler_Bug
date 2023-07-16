plain
travis_time:end:04ab31c7:start=1558073747108693651,finish=1558073749178045994,duration=2069352343
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:16:46] .................................................................................................... 100/2960
[01:16:59] .................................................................................i.................. 200/2960
[01:17:07] .................................................................................................... 300/2960
[01:17:18] .................................................................................................... 400/2960
[01:17:27] ...........................................F........................................................ 500/2960
[01:17:53] .................................................................................................... 700/2960
[01:18:04] .................................................................................................... 800/2960
[01:18:13] .................................................................................................... 900/2960
[01:18:26] .................................................................................................... 1000/2960
[01:18:26] .................................................................................................... 1000/2960
[01:18:40] .................................................................................................... 1100/2960
[01:18:49] .................................................................................................... 1200/2960
[01:18:59] .................................................................................................... 1300/2960
[01:19:10] ...........................ii....................................................................... 1400/2960
[01:19:22] ....F............................................................................................... 1500/2960
[01:19:44] .................................................................................................... 1700/2960
[01:19:57] .................................................................................................... 1800/2960
[01:20:07] .................................................................................................... 1900/2960
[01:20:21] ..........i.......................................................................i................. 2000/2960
---
[01:22:59] ------------------------------------------
[01:22:59] stderr:
[01:22:59] ------------------------------------------
[01:22:59] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:22:59]   left: `[104, 101, 108, 108, 111, 50, 10]`,
[01:22:59]  right: `[104, 101, 108, 108, 111, 10, 104, 101, 108, 108, 111, 50, 10]`', /checkout/src/test/run-pass/command-pre-exec.rs:42:5
[01:22:59] 
[01:22:59] ------------------------------------------
[01:22:59] 
[01:22:59] 
---
[01:22:59] ------------------------------------------
[01:22:59] stderr:
[01:22:59] ------------------------------------------
[01:22:59] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:22:59]   left: `"child stderr\n"`,
[01:22:59]  right: `"parent stdout\nchild stderr\n"`', /checkout/src/test/run-pass/issues/issue-30490.rs:78:5
[01:22:59] 
[01:22:59] ------------------------------------------
[01:22:59] 
[01:22:59] 
---
[01:22:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:22:59] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:59] 
[01:22:59] 
[01:22:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:59] 
[01:22:59] 
[01:22:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:59] Build completed unsuccessfully in 0:11:00
[01:22:59] Build completed unsuccessfully in 0:11:00
[01:22:59] Makefile:48: recipe for target 'check' failed
[01:22:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06263529
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 07:38:58 UTC 2019
---
travis_time:end:29a6b02e:start=1558078740022301581,finish=1558078740026978087,duration=4676506
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:35127651
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05e48497
travis_time:start:05e48497
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:069260fd
$ dmesg | grep -i kill
