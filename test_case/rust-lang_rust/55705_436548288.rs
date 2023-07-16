plain
travis_time:end:1eaa3ed8:start=1541580016565814781,finish=1541580078217353319,duration=61651538538
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:07]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:15] error: This node does not have a stability attribute
[00:04:15]     --> libcore/num/mod.rs:4799:5
[00:04:15]      |
[00:04:15] 4799 | /     pub fn kind(self) -> IntErrorKind {
[00:04:15] 4800 | |         self.kind
[00:04:15]      | |_____^
[00:04:15] 
[00:04:15] error: aborting due to previous error
[00:04:15] 
[00:04:15] 
[00:04:16] error: Could not compile `core`.
[00:04:16] 
[00:04:16] To learn more, run the command again with --verbose.
[00:04:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:16] expected success, got: exit code: 101
[00:04:16] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:04:16] travis_fold:end:stage0-std

[00:04:16] travis_time:end:stage0-std:start=1541580323734497013,finish=1541580345307811196,duration=21573314183


[00:04:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:16] Build completed unsuccessfully in 0:00:22
[00:04:16] Makefile:28: recipe for target 'all' failed
[00:04:16] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:22abf800
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
