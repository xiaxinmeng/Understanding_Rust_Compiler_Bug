plain
travis_time:end:19af7120:start=1549326632912247082,finish=1549326731097453263,duration=98185206181
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:20:04]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:20:05] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:20:05]     --> src/librustc_privacy/lib.rs:1076:42
[00:20:05]      |
[00:20:05] 1076 |                         Node::StructCtor(hir::VariantData::Tuple(_fields, _node_id)) => {
[00:20:05] 
[00:20:05] error: aborting due to previous error
[00:20:05] 
[00:20:05] For more information about this error, try `rustc --explain E0023`.
[00:20:05] For more information about this error, try `rustc --explain E0023`.
[00:20:05] error: Could not compile `rustc_privacy`.
[00:20:05] warning: build failed, waiting for other jobs to finish...
[00:20:38] error: build failed
[00:20:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:38] expected success, got: exit code: 101
[00:20:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:38] Build completed unsuccessfully in 0:17:05
[00:20:38] Makefile:18: recipe for target 'all' failed
[00:20:38] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:086e86e1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 00:52:59 UTC 2019
