plain
travis_time:end:060849be:start=1542009320871679467,finish=1542009322013149862,duration=1141470395
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:24]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:26] error: unused import: `Size`
[00:13:26]   --> librustc_mir/interpret/machine.rs:20:32
[00:13:26]    |
[00:13:26] 20 | use rustc::ty::{self, layout::{Size, TyLayout}, query::TyCtxtAt};
[00:13:26]    |
[00:13:26]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:26] 
[00:13:40] error: aborting due to previous error
[00:13:40] error: aborting due to previous error
[00:13:40] 
[00:13:40] error: Could not compile `rustc_mir`.
[00:13:40] warning: build failed, waiting for other jobs to finish...
[00:14:25] error: build failed
[00:14:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:14:25] expected success, got: exit code: 101
[00:14:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:14:25] travis_fold:end:stage0-rustc

[00:14:25] travis_time:end:stage0-rustc:start=1542009648910514339,finish=1542010197227927856,duration=548317413517


[00:14:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:14:25] Build completed unsuccessfully in 0:10:18
[00:14:25] Makefile:28: recipe for target 'all' failed
[00:14:25] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1db74948
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 12 08:09:57 UTC 2018
