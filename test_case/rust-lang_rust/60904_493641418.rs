plain
travis_time:end:0db7b98a:start=1558140643943904230,finish=1558140733431993876,duration=89488089646
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:24:45] .................................................................................................... 100/2959
[01:25:00] .................................................................................i.................. 200/2959
[01:25:10] .................................................................................................... 300/2959
[01:25:23] .................................................................................................... 400/2959
[01:25:34] .........................................F.......................................................... 500/2959
[01:26:05] .................................................................................................... 700/2959
[01:26:18] .................................................................................................... 800/2959
[01:26:29] .................................................................................................... 900/2959
[01:26:45] .................................................................................................... 1000/2959
[01:26:45] .................................................................................................... 1000/2959
[01:27:01] .................................................................................................... 1100/2959
[01:27:12] .................................................................................................... 1200/2959
[01:27:24] .................................................................................................... 1300/2959
[01:27:38] ..........................ii........................................................................ 1400/2959
[01:27:51] ...F................................................................................................ 1500/2959
[01:28:18] .................................................................................................... 1700/2959
[01:28:35] .................................................................................................... 1800/2959
[01:28:47] .................................................................................................... 1900/2959
[01:29:03] .........i.......................................................................i.................. 2000/2959
---
[01:32:06] ------------------------------------------
[01:32:06] stderr:
[01:32:06] ------------------------------------------
[01:32:06] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:32:06]   left: `[104, 101, 108, 108, 111, 50, 10]`,
[01:32:06]  right: `[104, 101, 108, 108, 111, 10, 104, 101, 108, 108, 111, 50, 10]`', /checkout/src/test/run-pass/command-pre-exec.rs:42:5
[01:32:06] 
[01:32:06] ------------------------------------------
[01:32:06] 
[01:32:06] 
---
[01:32:06] ------------------------------------------
[01:32:06] stderr:
[01:32:06] ------------------------------------------
[01:32:06] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:32:06]   left: `"child stderr\n"`,
[01:32:06]  right: `"parent stdout\nchild stderr\n"`', /checkout/src/test/run-pass/issues/issue-30490.rs:78:5
[01:32:06] 
[01:32:06] ------------------------------------------
[01:32:06] 
[01:32:06] 
---
[01:32:06] test result: FAILED. 2948 passed; 2 failed; 9 ignored; 0 measured; 0 filtered out
[01:32:06] 
[01:32:06] 
[01:32:06] 
[01:32:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:32:06] 
[01:32:06] 
[01:32:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:06] Build completed unsuccessfully in 0:13:02
[01:32:06] Build completed unsuccessfully in 0:13:02
[01:32:06] Makefile:48: recipe for target 'check' failed
[01:32:06] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0687a334
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 18 02:24:30 UTC 2019
