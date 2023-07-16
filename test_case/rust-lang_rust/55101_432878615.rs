plain
travis_time:end:185e9bc0:start=1540426330860040938,finish=1540426384901309793,duration=54041268855
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:47:52] .................................................................................................... 2200/4944
[00:47:56] .................................................................................................... 2300/4944
[00:48:00] .................................................................................................... 2400/4944
[00:48:04] .................................................................................................... 2500/4944
[00:48:07] .................................................iiiiiiiii.......................................... 2600/4944
[00:48:11] ...................................................................................................i 2700/4944
[00:48:17] .................................................................................................... 2900/4944
[00:48:21] .........................................................................................i.......... 3000/4944
[00:48:23] .................................................................................................... 3100/4944
[00:48:26] ................................................i.i..ii............................................. 3200/4944
---
[00:53:08] ..................................................................................................ii 2200/2870
[00:53:23] .....................................................................i....i......................... 2300/2870
[00:53:37] ............i....................................................................................... 2400/2870
[00:53:51] .................................................................................................... 2500/2870
[00:54:14] ...............................................................F....F............................... 2600/2870
[00:54:31] .................................................................................................... 2800/2870
[00:54:40] ......................................................................
[00:54:40] failures:
[00:54:40] 
[00:54:40] 
[00:54:40] ---- [run-pass] run-pass/traits/trait-alias-object-type.rs stdout ----
[00:54:40] 
[00:54:40] error: test compilation failed although it shouldn't!
[00:54:40] status: exit code: 1
[00:54:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/traits/trait-alias-object-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-object-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/traits/trait-alias-object-type/auxiliary"
[un-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:40] 
[00:54:40] 
[00:54:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:40] Build completed unsuccessfully in 0:11:24
[00:54:40] Build completed unsuccessfully in 0:11:24
[00:54:40] Makefile:58: recipe for target 'check' failed
[00:54:40] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10f34c66
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
