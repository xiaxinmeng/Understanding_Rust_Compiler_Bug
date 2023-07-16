plain
travis_time:end:04561fb0:start=1556129657440460589,finish=1556129772861380448,duration=115420919859
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:10:46] .................................................................................................... 1500/5464
[01:10:49] .................................................................................................... 1600/5464
[01:10:52] ..i................................................................................................. 1700/5464
[01:10:55] .................................................................................................... 1800/5464
[01:10:59] ...............................................................................................F.... 1900/5464
[01:11:06] .......................................i............................................................ 2100/5464
[01:11:10] .................................................................................................... 2200/5464
[01:11:14] .................................................................................................... 2300/5464
[01:11:18] .................................................................................................... 2400/5464
---
[01:13:15] normalized stderr:
[01:13:15] warning: the feature `futures_api` has been stable since 1.36.0 and no longer requires an attribute to enable
[01:13:15]   --> $DIR/issue-53249.rs:4:60
[01:13:15]    |
[01:13:15] LL | #![feature(arbitrary_self_types, async_await, await_macro, futures_api)]
[01:13:15]    |
[01:13:15]    = note: #[warn(stable_features)] on by default
[01:13:15] 
[01:13:15] 
[01:13:15] 
[01:13:15] 
[01:13:15] 
[01:13:15] The actual stderr differed from the expected stderr.
[01:13:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53249/issue-53249.stderr
[01:13:15] To update references, rerun the tests and pass the `--bless` flag
[01:13:15] To only update this specific test, also pass `--test-args issue-53249.rs`
[01:13:15] error: 1 errors occurred comparing output.
[01:13:15] status: exit code: 0
[01:13:15] status: exit code: 0
[01:13:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-53249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53249/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53249/auxiliary" "-A" "unused"
[01:13:15] ------------------------------------------
[01:13:15] 
[01:13:15] ------------------------------------------
[01:13:15] stderr:
[01:13:15] stderr:
[01:13:15] ------------------------------------------
[01:13:15] warning: the feature `futures_api` has been stable since 1.36.0 and no longer requires an attribute to enable
[01:13:15]   --> /checkout/src/test/ui/issue-53249.rs:4:60
[01:13:15]    |
[01:13:15] LL | #![feature(arbitrary_self_types, async_await, await_macro, futures_api)]
[01:13:15]    |
[01:13:15]    = note: #[warn(stable_features)] on by default
[01:13:15] 
[01:13:15] 
---
[01:13:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:13:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:15] 
[01:13:15] 
[01:13:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:15] 
[01:13:15] 
[01:13:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:15] Build completed unsuccessfully in 0:04:20
[01:13:15] Build completed unsuccessfully in 0:04:20
[01:13:15] make: *** [check] Error 1
[01:13:15] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d78518
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 19:29:37 UTC 2019
---
travis_time:end:1610e940:start=1556134178707165436,finish=1556134178712080079,duration=4914643
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:064703c2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:098c1f8c
travis_time:start:098c1f8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:140bf324
$ dmesg | grep -i kill
