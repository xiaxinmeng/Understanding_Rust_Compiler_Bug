plain
travis_time:end:0fcde900:start=1549447520811812088,finish=1549447523009847313,duration=2198035225
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:44]    Compiling rustc-demangle v0.1.10
[00:06:44] error: hidden lifetime parameters in types are deprecated
[00:06:44]    --> src/liballoc/collections/vec_deque.rs:968:45
[00:06:44]     |
[00:06:44] 968 |     pub fn drain<R>(&mut self, range: R) -> Drain<T>
[00:06:44]     |                                             ^^^^^^^^ help: indicate the anonymous lifetime: `Drain<'_, T>`
[00:06:46] error: aborting due to previous error
[00:06:46] 
[00:06:46] error: Could not compile `alloc`.
[00:06:46] 
[00:06:46] 
[00:06:46] To learn more, run the command again with --verbose.
[00:06:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:46] expected success, got: exit code: 101
[00:06:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:46] Build completed unsuccessfully in 0:00:38
[00:06:46] make: *** [all] Error 1
[00:06:46] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:267aae15
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 10:12:19 UTC 2019
