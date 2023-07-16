plain
travis_time:end:0934da20:start=1547147821963856316,finish=1547147824303879101,duration=2340022785
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:24] .......................i............................................................................ 2100/5298
[01:02:28] .................................................................................................... 2200/5298
[01:02:32] .................................................................................................... 2300/5298
[01:02:36] .................................................................................................... 2400/5298
[01:02:40] ...............................F.................................................................... 2500/5298
[01:02:48] .................................................................................................... 2700/5298
[01:02:52] .................................................................................................... 2800/5298
[01:02:56] .................................................................................................... 2900/5298
[01:02:59] .................................................................................................... 3000/5298
---
[01:03:24] ....................................ii.............................................................. 3700/5298
[01:03:26] ......................................................i............................................. 3800/5298
[01:03:28] .................................................................................................... 3900/5298
[01:03:30] ..........i......................................................................................... 4000/5298
[01:03:34] .......................................F............................................................ 4100/5298
[01:03:48] .................................................................................................... 4300/5298
[01:03:51] .................................................................................................... 4400/5298
[01:03:55] .......................................................i............................................ 4500/5298
[01:04:01] .................................................................................................... 4600/5298
---
[01:04:24] ---- [ui] ui/issues/issue-30240-b.rs stdout ----
[01:04:24] 
[01:04:24] error: ui test compiled successfully!
[01:04:24] status: exit code: 0
[01:04:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-30240-b.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30240-b/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30240-b/auxiliary" "-A" "unused"
[01:04:24] ------------------------------------------
[01:04:24] 
[01:04:24] ------------------------------------------
[01:04:24] stderr:
---
[01:04:24] ---- [ui] ui/pattern/slice-pattern-const-3.rs stdout ----
[01:04:24] 
[01:04:24] error: ui test compiled successfully!
[01:04:24] status: exit code: 0
[01:04:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/slice-pattern-const-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/slice-pattern-const-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/slice-pattern-const-3/auxiliary" "-A" "unused"
[01:04:24] ------------------------------------------
[01:04:24] 
[01:04:24] ------------------------------------------
[01:04:24] stderr:
---
[01:04:24] 
[01:04:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:24] 
[01:04:24] 
[01:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:24] 
[01:04:24] 
[01:04:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:24] Build completed unsuccessfully in 0:04:12
[01:04:24] Build completed unsuccessfully in 0:04:12
[01:04:24] make: *** [check] Error 1
[01:04:24] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03a32fd4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 10 20:21:39 UTC 2019
---
travis_time:end:198eb9d3:start=1547151700816659387,finish=1547151700821657444,duration=4998057
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b86a790
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|');
