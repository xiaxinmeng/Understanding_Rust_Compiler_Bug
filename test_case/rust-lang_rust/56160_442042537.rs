plain
travis_time:end:068749bc:start=1543318854857107104,finish=1543318936209875863,duration=81352768759
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:06] .................................................................................................... 400/5068
[00:49:10] .................................................................................................... 500/5068
[00:49:13] .............................i...................................................................... 600/5068
[00:49:17] .................................................................................................... 700/5068
[00:49:23] .................................................................F.................................. 800/5068
[00:49:30] ...................iiiii............................................................................ 1000/5068
[00:49:33] .................................................................................................... 1100/5068
[00:49:36] .................................................................................................... 1200/5068
[00:49:38] .................................................................................................... 1300/5068
---
[00:51:43] ---- [ui] ui/consts/const_let_eq_float.rs stdout ----
[00:51:43] 
[00:51:43] error: ui test compiled successfully!
[00:51:43] status: exit code: 0
[00:51:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_eq_float.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_eq_float/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_eq_float/auxiliary" "-A" "unused"
[00:51:43] ------------------------------------------
[00:51:43] 
[00:51:43] ------------------------------------------
[00:51:43] stderr:
---
[00:51:43] 
[00:51:43] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:51:43] 
[00:51:43] 
[00:51:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" nknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
52160 ./src/llvm/test/CodeGen/X86
51388 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47648 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
47464 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
---
travis_time:end:1f2806de:start=1543322050310041235,finish=1543322050318283635,duration=8242400
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00f22dda
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ 
