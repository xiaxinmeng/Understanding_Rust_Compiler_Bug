plain
travis_time:end:14b7e500:start=1542289814411928686,finish=1542289815802788038,duration=1390859352
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:45]    Compiling rand v0.4.3
[00:51:47] error[E0080]: it is undefined behavior to use this value
[00:51:47]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.4.3/src/prng/chacha.rs:39:1
[00:51:47]    |
[00:51:47] 39 | / static EMPTY: ChaChaRng = ChaChaRng {
[00:51:47] 40 | |     buffer:  [w(0); STATE_WORDS],
[00:51:47] 41 | |     state:   [w(0); STATE_WORDS],
[00:51:47] 42 | |     index:   STATE_WORDS
[00:51:47] 43 | | };
[00:51:47]    | |__^ tried to access memory with alignment 4, but alignment 8 is required
[00:51:47]    |
[00:51:47]    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:51:48] error: aborting due to previous error
[00:51:48] 
[00:51:48] For more information about this error, try `rustc --explain E0080`.
[00:51:48] error: Could not compile `rand`.
[00:51:48] error: Could not compile `rand`.
[00:51:48] warning: build failed, waiting for other jobs to finish...
[00:51:54] error: build failed
[00:51:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:51:54] expected success, got: exit code: 101
[00:51:54] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:51:54] travis_fold:end:stage1-rustc

[00:51:54] travis_time:end:stage1-rustc:start=1542292918623647135,finish=1542292939222870348,duration=20599223213


[00:51:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:51:54] Build completed unsuccessfully in 0:18:27
[00:51:54] make: *** [all] Error 1
[00:51:54] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00018db6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 15 14:42:19 UTC 2018
