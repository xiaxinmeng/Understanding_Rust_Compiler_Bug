plain
travis_time:end:0697c323:start=1541455300905558340,finish=1541455301982113055,duration=1076554715
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:29] 
[00:04:29] error: missing documentation for an enum
[00:04:29]     --> libcore/num/mod.rs:4780:1
[00:04:29]      |
[00:04:29] 4780 | pubm|     Underflow,
[00:04:29] 
[00:04:29] error: aborting due to 7 previous errors
[00:04:29] 
[00:04:29] error: Could not compile `core`.
[00:04:29] error: Could not compile `core`.
[00:04:29] 
[00:04:29] To learn more, run the command again with --verbose.
[00:04:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:29] expected success, got: exit code: 101
[00:04:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:04:29] travis_fold:end:stage0-std

[00:04:29] travis_time:end:stage0-std:start=1541455560642288556,finish=1541455581554374197,duration=20912085641


[00:04:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:29] Build completed unsuccessfully in 0:00:22
[00:04:29] Makefile:28: recipe for target 'all' failed
[00:04:29] make: *** [all] Error 1
56436 ./src/llvm/test/MC
51388 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
50128 ./src/llvm/test/CodeGen/X86
43808 ./src/libcompiler_builtins
