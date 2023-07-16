plain
travis_time:end:00a319d0:start=1544027880995004132,finish=1544027939576858534,duration=58581854402
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[01:06:00] ......................ii............................................................................ 3600/5109
[01:06:01] ........................................i........................................................... 3700/5109
[01:06:03] ................................................................................................i... 3800/5109
[01:06:04] .................................................................................................... 3900/5109
[01:06:10] .......................F............................................................................ 4000/5109
[01:06:19] .................................................................................................... 4200/5109
[01:06:22] ..........................................................................................i......... 4300/5109
[01:06:28] .................................................................................................... 4400/5109
[01:06:32] .................................................................................................... 4500/5109
---
[01:06:52] 
[01:06:52] ---- [ui] ui/print-fuel/print-fuel.rs stdout ----
[01:06:52] diff of stderr:
[01:06:52] 
[01:06:52] - Fuel used by foo: 3
[01:06:52] + Fuel used by foo: 0
[01:06:52] 
[01:06:52] 
[01:06:52] The actual stderr differed from the expected stderr.
[01:06:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/print-fuel.stderr
[01:06:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/print-fuel.stderr
[01:06:52] To update references, rerun the tests and pass the `--bless` flag
[01:06:52] To only update this specific test, also pass `--test-args print-fuel/print-fuel.rs`
[01:06:52] error: 1 errors occurred comparing output.
[01:06:52] status: exit code: 0
[01:06:52] status: exit code: 0
[01:06:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/print-fuel/print-fuel.rs" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human" "-Z" "print-fuel=foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print-fuel/print-fuel/auxiliary" "-A" "unused"
[01:06:52] ------------------------------------------
[01:06:52] 
[01:06:52] ------------------------------------------
[01:06:52] stderr:
[01:06:52] stderr:
[01:06:52] ------------------------------------------
[01:06:52] Fuel used by foo: 0
[01:06:52] ------------------------------------------
[01:06:52] 
[01:06:52] thread '[ui] ui/print-fuel/print-fuel.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[01:06:52] 
---
[01:06:52] 
[01:06:52] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:06:52] 
[01:06:52] 
[01:06:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:52] 
[01:06:52] 
[01:06:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:52] Build completed unsuccessfully in 0:04:14
[01:06:52] Build completed unsuccessfully in 0:04:14
[01:06:52] make: *** [check] Error 1
[01:06:52] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01a0c973
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec  5 17:46:01 UTC 2018
