plain
travis_time:end:0c232b60:start=1540936628965695122,finish=1540936686248772934,duration=57283077812
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:47:21] .................................................................................................... 1200/4983
[00:47:23] .................................................................................................... 1300/4983
[00:47:26] .................................................................................................... 1400/4983
[00:47:28] .................................................................................i.................. 1500/4983
[00:47:31] ...................................................i....F........................................... 1600/4983
[00:47:38] .................................................................................................... 1800/4983
[00:47:41] ..........................................................................................i......... 1900/4983
[00:47:45] .................................................................................................... 2000/4983
[00:47:48] .................................................................................................... 2100/4983
---
[00:49:22] failures:
[00:49:22] 
[00:49:22] ---- [ui] ui/huge-enum.rs stdout ----
[00:49:22] 
[00:49:22] error: Error: expected failure status (Some(1)) but received status None.
[00:49:22] status: signal: 11
[00:49:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/huge-enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/huge-enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/bulib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:22] 
[00:49:22] 
[00:49:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:22] Build completed unsuccessfully in 0:03:38
[00:49:22] Build completed unsuccessfully in 0:03:38
[00:49:22] make: *** [check] Error 1
[00:49:22] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:176cfa0a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17d2871e:start=1540939660257607188,finish=1540939661046736071,duration=789128883
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25375c65
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14ed680a
$ dmesg | grep -i kill
