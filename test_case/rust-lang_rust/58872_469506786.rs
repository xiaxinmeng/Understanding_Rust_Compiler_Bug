plain
travis_time:end:23442358:start=1551747703021913513,finish=1551747776395586629,duration=73373673116
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:08:03] .................................................................................................... 2600/5429
[01:08:07] .................................................................................................... 2700/5429
[01:08:11] .................................................................................................... 2800/5429
[01:08:15] .................................................................................................... 2900/5429
[01:08:19] ......................F............................................................................. 3000/5429
[01:08:26] .................................................................................................... 3200/5429
[01:08:29] ...............................................................i.................................... 3300/5429
[01:08:33] .................................................................................................... 3400/5429
[01:08:36] ...................................ii...i..ii....................................................... 3500/5429
---
[01:09:48] failures:
[01:09:48] 
[01:09:48] ---- [ui] ui/issues/issue-56031.rs stdout ----
[01:09:48] 
[01:09:48] error: /checkout/src/test/ui/issues/issue-56031.rs:3: unexpected error: '3:10: 3:11: expected `<`, found `T`'
[01:09:48] error: 1 unexpected errors found, 0 expected errors not found
[01:09:48] status: exit code: 1
[01:09:48] status: exit code: 1
[01:09:48] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56031.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56031/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56031/auxiliary" "-A" "unused"
[01:09:48]     Error {
[01:09:48]         line_num: 3,
[01:09:48]         kind: Some(
[01:09:48]             Error
[01:09:48]             Error
[01:09:48]         ),
[01:09:48]         msg: "3:10: 3:11: expected `<`, found `T`"
[01:09:48] ]
[01:09:48] 
[01:09:48] thread '[ui] ui/issues/issue-56031.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1378:13
[01:09:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:09:48] 
[01:09:48] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:09:48] 
[01:09:48] 
[01:09:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:48] 
[01:09:48] 
[01:09:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:48] Build completed unsuccessfully in 0:04:13
[01:09:48] Build completed unsuccessfully in 0:04:13
[01:09:48] make: *** [check] Error 1
[01:09:48] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c060be5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar  5 02:12:52 UTC 2019
---
travis_time:end:0cd29e28:start=1551751974078069165,finish=1551751974082431320,duration=4362155
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c211800
$ ln -s . checkout && for CORE in o
