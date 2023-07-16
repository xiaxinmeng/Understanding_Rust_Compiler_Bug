plain
travis_time:end:067b5e64:start=1553829091273309915,finish=1553829167122583413,duration=75849273498
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:12:10] ..............................................................................................ii.... 3800/5504
[01:12:13] .................................................................................................... 3900/5504
[01:12:15] ............i....................................................................................... 4000/5504
[01:12:18] .......................................................................i............................ 4100/5504
[01:12:20] ...F................................................................................................ 4200/5504
[01:12:37] .................................................................................................... 4400/5504
[01:12:41] .................................................................................................... 4500/5504
[01:12:44] .................................................................................................... 4600/5504
[01:12:49] ...................................i................................................................ 4700/5504
---
[01:13:21] failures:
[01:13:21] 
[01:13:21] ---- [ui] ui/parser/pat-tuple-1.rs stdout ----
[01:13:21] 
[01:13:21] error: /checkout/src/test/ui/parser/pat-tuple-1.rs:3: unexpected error: '3:10: 3:11: expected pattern, found `,`'
[01:13:21] error: /checkout/src/test/ui/parser/pat-tuple-1.rs:2: expected error not found: expected pattern, found `,`
[01:13:21] 
[01:13:21] error: 1 unexpected errors found, 1 expected errors not found
[01:13:21] status: exit code: 1
[01:13:21] status: exit code: 1
[01:13:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-tuple-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-tuple-1/auxiliary" "-A" "unused"
[01:13:21]     Error {
[01:13:21]         line_num: 3,
[01:13:21]         kind: Some(
[01:13:21]             Error
[01:13:21]             Error
[01:13:21]         ),
[01:13:21]         msg: "3:10: 3:11: expected pattern, found `,`"
[01:13:21] ]
[01:13:21] 
[01:13:21] not found errors (from test file): [
[01:13:21]     Error {
[01:13:21]     Error {
[01:13:21]         line_num: 2,
[01:13:21]         kind: Some(
[01:13:21]             Error
[01:13:21]         ),
[01:13:21]         msg: "expected pattern, found `,`"
[01:13:21] ]
[01:13:21] 
[01:13:21] thread '[ui] ui/parser/pat-tuple-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1381:13
[01:13:21] 
---
[01:13:21] 
[01:13:21] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:13:21] 
[01:13:21] 
[01:13:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:13:21] 
[01:13:21] 
[01:13:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:21] Build completed unsuccessfully in 0:04:43
[01:13:21] Build completed unsuccessfully in 0:04:43
[01:13:21] make: *** [check] Error 1
[01:13:21] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a5cd64c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 29 04:26:18 UTC 2019
