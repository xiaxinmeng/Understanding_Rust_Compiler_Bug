plain
travis_time:end:01e62106:start=1547706771405606304,finish=1547706772314363741,duration=908757437
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:41] .....................................ii............................................................. 3700/5297
[01:03:46] .................................................................................................... 3900/5297
[01:03:48] ...........i........................................................................................ 4000/5297
[01:03:51] .................................................................................................... 4100/5297
[01:04:02] ........................FF.......................................................................... 4200/5297
[01:04:09] .................................................................................................... 4400/5297
[01:04:13] ............................................................i....................................... 4500/5297
[01:04:19] .................................................................................................... 4600/5297
[01:04:22] .................................................................................................... 4700/5297
[01:04:22] .................................................................................................... 4700/5297
[01:04:25] .................................................................................................... 4800/5297
[01:04:30] .................................................................................................... 4900/5297
[01:04:34] .................................................................................................... 5000/5297
[01:04:37] .................................................................................................... 5100/5297
[01:04:39] .................................................................................................... 5200/5297
xport-macro/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/export-macro/auxiliary" "-A" "unused"
[01:04:42] ------------------------------------------
[01:04:42] 
[01:04:42] ------------------------------------------
[01:04:42] stderr:
---
[01:04:42] ---- [ui] ui/proc-macro/exports.rs stdout ----
[01:04:42] 
[01:04:42] error: ui test compiled successfully!
[01:04:42] status: exit code: 0
[01:04:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/exports.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/exports/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/exports/auxiliary" "-A" "unused"
[01:04:42] ------------------------------------------
[01:04:42] 
[01:04:42] ------------------------------------------
[01:04:42] stderr:
---
[01:04:42] 
[01:04:42] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:42] 
[01:04:42] 
[01:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:42] 
[01:04:42] 
[01:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:42] Build completed unsuccessfully in 0:04:12
[01:04:42] Build completed unsuccessfully in 0:04:12
[01:04:42] make: *** [check] Error 1
[01:04:42] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14d94515
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 07:37:44 UTC 2019
---
travis_time:end:01d0971e:start=1547710665686152910,finish=1547710665693091994,duration=6939084
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19073eec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:166b5595
$ dmesg | grep -i kill
