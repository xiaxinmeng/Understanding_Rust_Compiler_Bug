plain
travis_time:end:024c9d7e:start=1542649368783138568,finish=1542649369751720741,duration=968582173
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:18:18]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:18:18] error[E0507]: cannot move out of an `Rc`
[00:18:18]   --> librustc_lint/unused.rs:71:43
[00:18:18]    |
[00:18:18] 71 |                     for (predicate, _) in cx.tcx.predicates_of(def).predicates {
[00:18:18]    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of an `Rc`
[00:18:18] error: aborting due to previous error
[00:18:18] 
[00:18:18] For more information about this error, try `rustc --explain E0507`.
[00:18:18] error: Could not compile `rustc_lint`.
[00:18:18] error: Could not compile `rustc_lint`.
[00:18:18] warning: build failed, waiting for other jobs to finish...
[00:18:51] error: build failed
[00:18:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:51] expected success, got: exit code: 101
[00:18:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:51] Build completed unsuccessfully in 0:15:10
[00:18:51] Makefile:28: recipe for target 'all' failed
[00:18:51] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:36d6db2f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 19 18:01:51 UTC 2018
