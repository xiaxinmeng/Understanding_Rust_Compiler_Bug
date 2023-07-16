plain
travis_time:end:1ec68044:start=1543598434283655787,finish=1543598436631300479,duration=2347644692
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:00] .................................................................................................... 2100/5102
[00:48:04] .................................................................................................... 2200/5102
[00:48:08] .................................................................................................... 2300/5102
[00:48:12] .................................................................................................... 2400/5102
[00:48:16] ...................................................F................................................ 2500/5102
[00:48:23] .................................................................................................... 2700/5102
[00:48:27] .................................................................................................... 2800/5102
[00:48:30] .................................................................................................... 2900/5102
[00:48:33] .................................................................................................... 3000/5102
---
[00:49:30] .................................................................i.................................. 4700/5102
[00:49:33] .................................................................................................... 4800/5102
[00:49:36] .................................................................................................... 4900/5102
[00:49:39] .................................................................................................... 5000/5102
kout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36638.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36638/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36638/auxiliary" "-A" "unused"
[00:49:42]     Error {
[00:49:42]         line_num: 13,
[00:49:42]         kind: Some(
[00:49:42]             Error
[00:49:42]             Error
[00:49:42]         ),
[00:49:42]         msg: "13:12: 13:16: parameter `Self` is never used [E0392]"
[00:49:42] ]
[00:49:42] 
[00:49:42] thread '[ui] ui/issues/issue-36638.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13
[00:49:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:42] 
[00:49:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:49:42] 
[00:49:42] 
[00:49:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:42] 
[00:49:42] 
[00:49:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:42] Build completed unsuccessfully in 0:03:56
[00:49:42] Build completed unsuccessfully in 0:03:56
[00:49:42] make: *** [check] Error 1
[00:49:42] Makefile:58: recipe for target 'check' failed
123696 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
122844 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
115352 ./src/llvm/test/CodeGen
