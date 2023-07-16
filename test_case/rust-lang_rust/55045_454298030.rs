plain
travis_time:end:0adfb1d6:start=1547533872111688181,finish=1547533873240624492,duration=1128936311
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:20:08] .................................................................................................... 1200/5303
[01:20:10] .................................................................................................... 1300/5303
[01:20:13] .................................................................................................... 1400/5303
[01:20:16] .................................................................................................... 1500/5303
[01:20:20] .......F..................................................................................i......... 1600/5303
[01:20:27] .................................................................................................... 1800/5303
[01:20:32] .................................................................................................... 1900/5303
[01:20:36] .................................................................................................... 2000/5303
[01:20:39] ................i................................................................................... 2100/5303
---
[01:22:58] failures:
[01:22:58] 
[01:22:58] ---- [ui] ui/feature-gates/feature-gate-is_sorted.rs stdout ----
[01:22:58] 
[01:22:58] error: /checkout/src/test/ui/feature-gates/feature-gate-is_sorted.rs:3: unexpected error: '3:33: 3:42: use of unstable library feature 'is_sorted': new API (see issue #53485) [E0658]'
[01:22:58] 
[01:22:58] error: /checkout/src/test/ui/feature-gates/feature-gate-is_sorted.rs:5: unexpected error: '5:39: 5:55: use of unstable library feature 'is_sorted': new API (see issue #53485) [E0658]'
[01:22:58] 
[01:22:58] error: /checkout/src/test/ui/feature-gates/feature-gate-is_sorted.rs:9: unexpected error: '9:26: 9:35: use of unstable library feature 'is_sorted': new API (see issue #53485) [E0658]'
[01:22:58] 
[01:22:58] error: /checkout/src/test/ui/feature-gates/feature-gate-is_sorted.rs:11: unexpected error: '11:32: 11:48: use of unstable library feature 'is_sorted': new API (see issue #53485) [E0658]'
[01:22:58] error: 4 unexpected errors found, 0 expected errors not found
[01:22:58] status: exit code: 1
[01:22:58] status: exit code: 1
[01:22:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-is_sorted.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-is_sorted/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-is_sorted/auxiliary" "-A" "unused"
[01:22:58]     Error {
[01:22:58]         line_num: 3,
[01:22:58]         kind: Some(
[01:22:58]             Error
[01:22:58]             Error
[01:22:58]         ),
[01:22:58]         msg: "3:33: 3:42: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[01:22:58]     Error {
[01:22:58]         line_num: 5,
[01:22:58]         kind: Some(
[01:22:58]             Error
[01:22:58]             Error
[01:22:58]         ),
[01:22:58]         msg: "5:39: 5:55: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[01:22:58]     Error {
[01:22:58]         line_num: 9,
[01:22:58]         kind: Some(
[01:22:58]             Error
[01:22:58]             Error
[01:22:58]         ),
[01:22:58]         msg: "9:26: 9:35: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[01:22:58]     Error {
[01:22:58]         line_num: 11,
[01:22:58]         kind: Some(
[01:22:58]             Error
[01:22:58]             Error
[01:22:58]         ),
[01:22:58]         msg: "11:32: 11:48: use of unstable library feature \'is_sorted\': new API (see issue #53485) [E0658]"
[01:22:58] ]
[01:22:58] 
[01:22:58] thread '[ui] ui/feature-gates/feature-gate-is_sorted.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1342:13
[01:22:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:22:58] 
[01:22:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:22:58] 
[01:22:58] 
[01:22:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:22:58] 
[01:22:58] 
[01:22:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:58] Build completed unsuccessfully in 0:04:51
[01:22:58] Build completed unsuccessfully in 0:04:51
[01:22:58] Makefile:48: recipe for target 'check' failed
[01:22:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2348e4c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 07:54:22 UTC 2019
---
travis_time:end:20a3fd02:start=1547538863854479259,finish=1547538863862378013,duration=7898754
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02d29666
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e4fa251
$ dmesg | grep -i kill
