plain
travis_time:end:062f5d9f:start=1558117023535016117,finish=1558117127525297136,duration=103990281019
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:13:24] .................................................................................................... 100/2959
[01:13:36] .................................................................................i.................. 200/2959
[01:13:45] .................................................................................................... 300/2959
[01:13:56] .................................................................................................... 400/2959
[01:14:06] .........................................F.......................................................... 500/2959
[01:14:32] .................................................................................................... 700/2959
[01:14:44] .................................................................................................... 800/2959
[01:14:53] .................................................................................................... 900/2959
[01:15:07] .................................................................................................... 1000/2959
[01:15:07] .................................................................................................... 1000/2959
[01:15:21] .................................................................................................... 1100/2959
[01:15:30] .................................................................................................... 1200/2959
[01:15:40] .................................................................................................... 1300/2959
[01:15:52] ..........................ii........................................................................ 1400/2959
[01:16:03] ...F................................................................................................ 1500/2959
[01:16:26] .................................................................................................... 1700/2959
[01:16:40] .................................................................................................... 1800/2959
[01:16:50] .................................................................................................... 1900/2959
[01:17:04] .........i.......................................................................i.................. 2000/2959
---
[01:19:46] ------------------------------------------
[01:19:46] stderr:
[01:19:46] ------------------------------------------
[01:19:46] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:19:46]   left: `[104, 101, 108, 108, 111, 50, 10]`,
[01:19:46]  right: `[104, 101, 108, 108, 111, 10, 104, 101, 108, 108, 111, 50, 10]`', /checkout/src/test/run-pass/command-pre-exec.rs:42:5
[01:19:46] 
[01:19:46] ------------------------------------------
[01:19:46] 
[01:19:46] 
---
[01:19:46] ------------------------------------------
[01:19:46] stderr:
[01:19:46] ------------------------------------------
[01:19:46] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:19:46]   left: `"child stderr\n"`,
[01:19:46]  right: `"parent stdout\nchild stderr\n"`', /checkout/src/test/run-pass/issues/issue-30490.rs:78:5
[01:19:46] 
[01:19:46] ------------------------------------------
[01:19:46] 
[01:19:46] 
---
[01:19:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:512:22
[01:19:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:46] 
[01:19:46] 
[01:19:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:46] 
[01:19:46] 
[01:19:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:46] Build completed unsuccessfully in 0:11:18
[01:19:46] Build completed unsuccessfully in 0:11:18
[01:19:46] make: *** [check] Error 1
[01:19:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:159479f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 17 19:38:43 UTC 2019
---
travis_time:end:35e59e50:start=1558121924747896241,finish=1558121924752881577,duration=4985336
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13fd6512
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08f85a24
travis_time:start:08f85a24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11710b00
$ dmesg | grep -i kill
