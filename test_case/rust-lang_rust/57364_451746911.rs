plain
travis_time:end:0ec3f578:start=1546782125819050653,finish=1546782196239804965,duration=70420754312
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:27] .................................................................................................... 3500/5299
[01:00:31] .................................................................................................... 3600/5299
[01:00:34] ......................................ii............................................................ 3700/5299
[01:00:36] .........................................................i.......................................... 3800/5299
[01:00:37] .........................................F.......................................................... 3900/5299
[01:00:42] .................................................................................................... 4100/5299
[01:00:52] .................................................................................................... 4200/5299
[01:00:55] .................................................................................................... 4300/5299
[01:00:59] .................................................................................................... 4400/5299
[01:00:59] .................................................................................................... 4400/5299
[01:01:02] ........................................................i........................................... 4500/5299
[01:01:08] .................................................................................................... 4600/5299
[01:01:12] .................................................................................................... 4700/5299
[01:01:15] .................................................................................................... 4800/5299
[01:01:19] .................................................................................................... 4900/5299
[01:01:22] .................................................................................................... 5000/5299
[01:01:31] - LL | trait Tr4: !Sized + !Send + Copy + !Sync + !Unpin {} //~ ERROR negative trait bounds are not supported
[01:01:31] -    |                                  --^^^^^
[01:01:31] + LL | trait Tr4: !Sized + Copy + !Sync + !Unpin {} //~ ERROR negative trait bounds are not supported
[01:01:31] +    |                                  --^^^^^^
[01:01:31] 47    |                                  help: remove this trait bound
[01:01:31] 48 
[01:01:31] 
[01:01:31] 49 error: negative trait bounds are not supported
[01:01:31] 49 error: negative trait bounds are not supported
[01:01:31] -   --> $DIR/issue-33418.rs:6:44
[01:01:31] -    |
[01:01:31] - LL | trait Tr4: !Sized + !Send + Copy + !Sync + !Unpin {} //~ ERROR negative trait bounds are not supported
[01:01:31] -    |                                          --^^^^^^
[01:01:31] -    |                                          help: remove this trait bound
[01:01:31] - 
[01:01:31] - error: negative trait bounds are not supported
[01:01:31] 58   --> $DIR/issue-33418.rs:7:12
[01:01:31] 58   --> $DIR/issue-33418.rs:7:12
[01:01:31] 59    |
[01:01:31] 60 LL | trait Tr5: !Sized + !Send {} //~ ERROR negative trait bounds are not supported
[01:01:31] 70    |                   |
[01:01:31] 71    |                   help: remove this trait bound
[01:01:31] 72 
[01:01:31] - error: aborting due to 9 previous errors
---
[01:01:31] 
[01:01:31] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:01:31] 
[01:01:31] 
[01:01:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/travis_time:end:2a9fb0c4:start=1546782205172551672,finish=1546785897059313420,duration=3691886761748
travis_time:start:03060326
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan  6 14:44:57 UTC 2019
Sun, 06 Jan 2019 14:44:57 GMT
