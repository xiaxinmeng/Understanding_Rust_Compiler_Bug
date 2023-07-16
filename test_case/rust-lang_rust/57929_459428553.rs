plain
travis_time:end:000797a0:start=1548950965202139877,finish=1548951137597882785,duration=172395742908
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:05:43] 
[01:05:43] error: Could not document `test`.
[01:05:43] 
[01:05:43] Caused by:
[01:05:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name test src/libtest/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[01:05:43] 
[01:05:43] 
[01:05:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[01:05:43] 
[01:05:43] 
[01:05:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:05:43] Build completed unsuccessfully in 0:06:08
[01:05:43] Build completed unsuccessfully in 0:06:08
[01:05:43] make: *** [all] Error 1
[01:05:43] Makefile:18: recipe for target 'all' failed
2390004 ./obj
2389964 ./obj/build
1722116 ./obj/build/x86_64-unknown-linux-gnu
1345024 ./src
