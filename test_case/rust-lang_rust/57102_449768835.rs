plain
travis_time:end:0394abd0:start=1545679466785563247,finish=1545679539125547910,duration=72339984663
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:55:14] .................................................................................................... 2300/5198
[00:55:18] .................................................................................................... 2400/5198
[00:55:21] .................................................................................................... 2500/5198
[00:55:25] .................................................................................................... 2600/5198
[00:55:29] ..........................F......................................................................... 2700/5198
[00:55:35] .................................................................................................... 2900/5198
[00:55:38] .................................................................................................... 3000/5198
[00:55:42] ..................................................................................................i. 3100/5198
[00:55:45] .................................................................................................... 3200/5198
---
[00:56:49] 
[00:56:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:505:22
[00:56:49] 
[00:56:49] 
[00:56:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:49] 
[00:56:49] 
[00:56:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:49] Build completed unsuccessfully in 0:03:40
---
150164 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
150160 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
147768 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
144216 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144212 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7wbf29w0r-1dmdvfy-c95hburtu2sh
