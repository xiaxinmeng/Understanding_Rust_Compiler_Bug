plain
travis_time:end:181305f0:start=1558725773589662470,finish=1558725774342713073,duration=753050603
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |████████████████████████████████| 542kB 54.6MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 21.1MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.156 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f5/aa/79fc47ccc3c7d0f36aafb9d85091d7d8a8f10d8ad24ccf3a89cf126b9f4e/botocore-1.12.156-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 22.7MB/s eta 0:00:01
    0% |▏                               | 30kB 28.1MB/s eta 0:00:01
    0% |▎                               | 40kB 31.2MB/s eta 0:00:01
    0% |▎                               | 51kB 34.1MB/s eta 0:00:01
---
[01:10:49] .................................................................................................... 2700/5578
[01:10:53] .................................................................................................... 2800/5578
[01:10:57] .................................................................................................... 2900/5578
[01:11:01] .................................................................................................... 3000/5578
[01:11:04] ......................F............................................................................. 3100/5578
[01:11:12] .................................................................................................... 3300/5578
[01:11:15] ...........i........................................................................................ 3400/5578
[01:11:19] .....................................................................................ii...i..ii..... 3500/5578
[01:11:22] .................................................................................................... 3600/5578
---
[01:12:39] failures:
[01:12:39] 
[01:12:39] ---- [ui] ui/issues/issue-6991.rs stdout ----
[01:12:39] 
[01:12:39] error: test compilation failed although it shouldn't!
[01:12:39] status: exit code: 101
[01:12:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6991.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6991/auxiliary" "-A" "unused"
[01:12:39] ------------------------------------------
[01:12:39] 
[01:12:39] ------------------------------------------
[01:12:39] stderr:
[01:12:39] stderr:
[01:12:39] ------------------------------------------
[01:12:39] error: internal compiler error: src/librustc_mir/interpret/place.rs:604: eval_place_to_mplace called on (*(x: &usize))
[01:12:39] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[01:12:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:39] error: aborting due to previous error
[01:12:39] 
[01:12:39] 
[01:12:39] 
[01:12:39] note: the compiler unexpectedly panicked. this is a bug.
[01:12:39] 
[01:12:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:12:39] 
[01:12:39] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[01:12:39] 
[01:12:39] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
[01:12:39] 
[01:12:39] ------------------------------------------
[01:12:39] 
[01:12:39] 
---
[01:12:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:12:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:12:39] 
[01:12:39] 
[01:12:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:12:39] 
[01:12:39] 
[01:12:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:39] Build completed unsuccessfully in 0:04:41
[01:12:39] Build completed unsuccessfully in 0:04:41
[01:12:39] make: *** [check] Error 1
[01:12:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d37ad37
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 20:35:44 UTC 2019
---
travis_time:end:0cd448d6:start=1558730146249856522,finish=1558730146254500936,duration=4644414
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c51d749
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c884f22
travis_time:start:0c884f22
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000db380
$ dmesg | grep -i kill
