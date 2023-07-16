plain
travis_time:end:257ea7f7:start=1544910026187639262,finish=1544910027244849955,duration=1057210693
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:43:45] .................................................................................................... 500/5177
[00:43:48] ..............................i..................................................................... 600/5177
[00:43:52] .................................................................................................... 700/5177
[00:43:57] .................................................................................................... 800/5177
[00:44:01] .i...............i.................................................................................. 900/5177
[00:44:04] ....................F....iiiii...................................................................... 1000/5177
[00:44:09] .................................................................................................... 1200/5177
[00:44:12] .................................................................................................... 1300/5177
[00:44:14] .................................................................................................... 1400/5177
[00:44:16] .................................................................................................... 1500/5177
---
[00:46:09] ................................i................................................................... 4800/5177
[00:46:12] .................................................................................................... 4900/5177
[00:46:15] .................................................................................................... 5000/5177
[00:46:17] .................................................................................................... 5100/5177
rc/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-56028-there-is-an-enum-variant/auxiliary" "-A" "unused"
[00:46:19]     Error {
[00:46:19]         line_num: 9,
[00:46:19]         kind: Some(
[00:46:19]             Error
[00:46:19]             Error
[00:46:19]         ),
[00:46:19]         msg: "9:15: 9:18: cannot find type `Set` in this scope [E0412]"
[00:46:19]     Error {
[00:46:19]         line_num: 9,
[00:46:19]         kind: Some(
[00:46:19]             Error
[00:46:19]             Error
[00:46:19]         ),
[00:46:19]         msg: "9:21: 9:24: cannot find value `Set` in this scope [E0425]"
[00:46:19] ]
[00:46:19] 
[00:46:19] thread '[ui] ui/did_you_mean/issue-56028-there-is-an-enum-variant.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1349:13
[00:46:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:46:19] 
[00:46:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:46:19] 
[00:46:19] 
[00:46:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:19] 
[00:46:19] 
[00:46:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:19] Build completed unsuccessfully in 0:03:46
[00:46:19] Build completed unsuccessfully in 0:03:46
[00:46:19] make: *** [check] Error 1
[00:46:19] Makefile:58: recipe for target 'check' failed
124332 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
118648 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115352 ./src/llvm/test/CodeGen
111164 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
