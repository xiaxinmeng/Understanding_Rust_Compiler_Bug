plain
travis_time:end:03cd7b0e:start=1549633771195686351,finish=1549633774336343522,duration=3140657171
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:10:21] .................................................................................................... 3800/5376
[01:10:23] .............i...................................................................................... 3900/5376
[01:10:25] ......................................................................i............................. 4000/5376
[01:10:27] .................................................................................................... 4100/5376
[01:10:35] .............................................F...................................................... 4200/5376
[01:10:45] .................................................................................................... 4400/5376
[01:10:49] .................................................................................................... 4500/5376
[01:10:53] ..........................i......................................................................... 4600/5376
[01:10:59] .................................................................................................... 4700/5376
---
[01:11:21] failures:
[01:11:21] 
[01:11:21] ---- [ui] ui/privacy/private-in-public-warn.rs stdout ----
[01:11:21] 
[01:11:21] error: /checkout/src/test/ui/privacy/private-in-public-warn.rs:75: unexpected warning: '75:29: 75:38: where-clauses are not enforced in type aliases [type_alias_bounds]'
[01:11:21] error: /checkout/src/test/ui/privacy/private-in-public-warn.rs:75: expected warning not found: where clauses are not enforced in type aliases
[01:11:21] 
[01:11:21] error: 1 unexpected errors found, 1 expected errors not found
[01:11:21] status: exit code: 1
[01:11:21] status: exit code: 1
[01:11:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-in-public-warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/auxiliary" "-A" "unused"
[01:11:21]     Error {
[01:11:21]         line_num: 75,
[01:11:21]         kind: Some(
[01:11:21]             Warning
[01:11:21]             Warning
[01:11:21]         ),
[01:11:21]         msg: "75:29: 75:38: where-clauses are not enforced in type aliases [type_alias_bounds]"
[01:11:21] ]
[01:11:21] 
[01:11:21] not found errors (from test file): [
[01:11:21]     Error {
[01:11:21]     Error {
[01:11:21]         line_num: 75,
[01:11:21]         kind: Some(
[01:11:21]             Warning
[01:11:21]         ),
[01:11:21]         msg: "where clauses are not enforced in type aliases"
[01:11:21] ]
[01:11:21] 
[01:11:21] thread '[ui] ui/privacy/private-in-public-warn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1379:13
[01:11:21] 
---
[01:11:21] 
[01:11:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:11:21] 
[01:11:21] 
[01:11:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:11:21] 
[01:11:21] 
[01:11:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:21] Build completed unsuccessfully in 0:04:13
[01:11:21] Build completed unsuccessfully in 0:04:13
[01:11:21] make: *** [check] Error 1
[01:11:21] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00dc6ef5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 15:01:06 UTC 2019
---
travis_time:end:21a42558:start=1549638067940169014,finish=1549638067945287767,duration=5118753
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1753a293
$ ln -s . checkout && for CORE in otravis_time:start:208766d6
travis_time:end:208766d6:start=1549638067970579447,finish=1549638067981113194,duration=10533747
travis_fold:end:after_failure.6

Done. Your build exited with 1.
