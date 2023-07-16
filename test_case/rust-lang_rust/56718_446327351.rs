plain
travis_time:end:02dd6c7c:start=1544553219482254895,finish=1544553220600916814,duration=1118661919
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:16]    Compiling failure v0.1.3
[00:46:16] error[E0034]: multiple applicable items in scope
[00:46:16]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.3/src/backtrace/mod.rs:132:20
[00:46:16]     |
[00:46:16] 132 |                 bt.fmt(f)
[00:46:16]     |                    ^^^ multiple `fmt` found
[00:46:16]     |
[00:46:16]     = note: candidate #1 is defined in an impl of the trait `std::fmt::Debug` for the type `backtrace::backtrace::Backtrace`
[00:46:16]     = note: candidate #2 is defined in an impl of the trait `std::fmt::Display` for the type `backtrace::backtrace::Backtrace`
[00:46:16] error[E0034]: multiple applicable items in scope
[00:46:16]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/failure-0.1.3/src/backtrace/mod.rs:140:20
[00:46:16]     |
[00:46:16]     |
[00:46:16] 140 |                 bt.fmt(f)
[00:46:16]     |                    ^^^ multiple `fmt` found
[00:46:16]     error: build failed
[00:46:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--message-format" "json"
[00:46:16] expected success, got: exit code: 101
[00:46:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:16] Build completed unsuccessfully in 0:00:51
[00:46:16] make: *** [check] Error 1
[00:46:16] Makefile:58: recipe for target 'check' failed
122892 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
115664 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115340 ./src/llvm/test/CodeGen
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
---
travis_time:end:2208208e:start=1544556006901013071,finish=1544556006907334300,duration=6321229
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:191e172a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/
