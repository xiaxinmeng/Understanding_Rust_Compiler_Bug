plain
travis_time:end:07722250:start=1549933917633151571,finish=1549933918516362335,duration=883210764
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    73% |███████████████████████▍        | 51kB 41.0MB/s eta 0:00:01
    87% |████████████████████████████    | 61kB 41.5MB/s eta 0:00:01
    100% |████████████████████████████████| 71kB 26.0MB/s 
Collecting botocore==1.12.92 (from awscli)
  Downloading https://files.pythonhosted.org/packages/a6/ec/e68d5d9b5eaa53d3552de0638231a8678c327737f4fc9fa62733483260fc/botocore-1.12.92-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 29.2MB/s eta 0:00:01
    0% |▏                               | 30kB 35.5MB/s eta 0:00:01
    0% |▎                               | 40kB 35.6MB/s eta 0:00:01
    0% |▎                               | 51kB 35.4MB/s eta 0:00:01
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:56] 
[01:08:56] running 97 tests
[01:09:10] .............................................................F...................................
[01:09:10] 
[01:09:10] ---- [incremental] incremental/issue-54242.rs stdout ----
[01:09:10] 
[01:09:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:09:10] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:09:10] error in revision `rpass2`: compilation failed!
[01:09:10] status: exit code: 101
[01:09:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-54242.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54242/issue-54242.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54242/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-54242/auxiliary"
[01:09:10] ------------------------------------------
[01:09:10] 
[01:09:10] ------------------------------------------
[01:09:10] stderr:
[01:09:10] stderr:
[01:09:10] ------------------------------------------
[01:09:10] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:149:9
[01:09:10] 
[01:09:10] error: internal compiler error: unexpected panic
[01:09:10] 
[01:09:10] note: the compiler unexpectedly panicked. this is a bug.
[01:09:10] note: the compiler unexpectedly panicked. this is a bug.
[01:09:10] 
[01:09:10] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:09:10] 
[01:09:10] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[01:09:10] 
[01:09:10] note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath
[01:09:10] 
[01:09:10] ------------------------------------------
[01:09:10] 
[01:09:10] thread '[incremental] incremental/issue-54242.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:09:10] test result: FAILED. 96 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:09:10] 
[01:09:10] 
[01:09:10] 
[01:09:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:10] 
[01:09:10] 
[01:09:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:10] Build completed unsuccessfully in 0:11:36
[01:09:10] Build completed unsuccessfully in 0:11:36
[01:09:10] make: *** [check] Error 1
[01:09:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:178548d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 02:21:20 UTC 2019
---
travis_time:end:08026c45:start=1549938081824055245,finish=1549938081829083899,duration=5028654
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10d8a41e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
