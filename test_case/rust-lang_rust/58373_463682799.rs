plain
travis_time:end:00f7216e:start=1550157841394524196,finish=1550157956627325586,duration=115232801390
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:34:54]    Compiling lock_api v0.1.3
[00:34:55]    Compiling polonius-engine v0.6.2
[00:34:55]    Compiling rustc_version v0.2.3
[00:34:56]    Compiling rls-span v0.4.0
[00:34:56] error[E0658]: use of unstable library feature 'stdsimd' (see issue #27731)
[00:34:56]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.1.2/src/specialized/pclmulqdq.rs:13:12
[00:34:56]    |
[00:34:56] 13 |         if is_x86_feature_detected!("pclmulqdq")
[00:34:56]    |
[00:34:56]    |
[00:34:56]    = help: add #![feature(stdsimd)] to the crate attributes to enable
[00:34:56] 
[00:34:56] error: aborting due to previous error
[00:34:56] 
[00:34:56] For more information about this error, try `rustc --explain E0658`.
[00:34:56] For more information about this error, try `rustc --explain E0658`.
[00:34:56] error: Could not compile `crc32fast`.
travis_time:end:01be1930:start=1550157965171492903,finish=1550160062806801651,duration=2097635308748
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0311357c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
175688 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
156148 ./src/llvm-project/clang
155976 ./obj/build/bootstrap/debug/incremental
141204 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
141200 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f9hh046clh-1gvzu9y-2ee07hmknei1u
138576 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
135800 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123292 ./src/llvm-project/llvm/test/CodeGen
108528 ./src/llvm-project/lldb
