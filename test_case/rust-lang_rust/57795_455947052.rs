plain
travis_time:end:00feae5a:start=1548041956236506024,finish=1548041957210845898,duration=974339874
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:55] .................................................................................................... 3500/5312
[01:04:58] .................................................................................................... 3600/5312
[01:05:01] ............................................ii...................................................... 3700/5312
[01:05:03] ..............................................................i..................................... 3800/5312
[01:05:05] ..................F................................................................................. 3900/5312
[01:05:11] .................................................................................................... 4100/5312
[01:05:21] .................................................................................................... 4200/5312
[01:05:25] .................................................................................................... 4300/5312
[01:05:28] .................................................................................................... 4400/5312
---
[01:06:01] failures:
[01:06:01] 
[01:06:01] ---- [ui] ui/parser/issue-17718-const-mut.rs stdout ----
[01:06:01] 
[01:06:01] error: /checkout/src/test/ui/parser/issue-17718-const-mut.rs:1: unexpected help message: '1:1: 1:6: you might want to declare a static instead'
[01:06:01] 
[01:06:01] error: /checkout/src/test/ui/parser/issue-17718-const-mut.rs:2: expected help message not found: you might want to declare a static instead
[01:06:01] error: 1 unexpected errors found, 1 expected errors not found
[01:06:01] status: exit code: 1
[01:06:01] status: exit code: 1
[01:06:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-17718-const-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-17718-const-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-17718-const-mut/auxiliary" "-A" "unused"
[01:06:01]     Error {
[01:06:01]         line_num: 1,
[01:06:01]         kind: Some(
[01:06:01]             Help
[01:06:01]             Help
[01:06:01]         ),
[01:06:01]         msg: "1:1: 1:6: you might want to declare a static instead"
[01:06:01] ]
[01:06:01] 
[01:06:01] not found errors (from test file): [
[01:06:01]     Error {
[01:06:01]     Error {
[01:06:01]         line_num: 2,
[01:06:01]         kind: Some(
[01:06:01]             Help
[01:06:01]         ),
[01:06:01]         msg: "you might want to declare a static instead"
[01:06:01] ]
[01:06:01] 
[01:06:01] thread '[ui] ui/parser/issue-17718-const-mut.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1342:13
[01:06:01] 
---
[01:06:01] 
[01:06:01] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:06:01] 
[01:06:01] 
[01:06:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:01] 
[01:06:01] 
[01:06:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:01] Build completed unsuccessfully in 0:04:09
[01:06:01] Build completed unsuccessfully in 0:04:09
[01:06:01] make: *** [check] Error 1
[01:06:01] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34e7eee8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 21 04:45:28 UTC 2019
---
travis_time:end:017e1250:start=1548045930034509422,finish=1548045930041359491,duration=6850069
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26c43fb7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:009af668
$ dmesg | grep -i kill
