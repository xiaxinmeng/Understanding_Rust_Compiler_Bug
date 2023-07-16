plain
travis_time:end:00856986:start=1544887679055168668,finish=1544887680103388685,duration=1048220017
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:10:42] 
[00:10:42] error[E0433]: failed to resolve: use of undeclared type or module `AdtKind`
[00:10:42]     --> src/librustc_typeck/check/mod.rs:5179:33
[00:10:42]      |
[00:10:42] 5179 |                                 AdtKind::Union => {
[00:10:42]      |                                 ^^^^^^^ use of undeclared type or module `AdtKind`
[00:10:42] error[E0425]: cannot find value `tcx` in this scope
[00:10:42]     --> src/librustc_typeck/check/mod.rs:5168:39
[00:10:42]      |
[00:10:42]      |
[00:10:42] 5168 |                         (variant.did, tcx.type_of(variant.did))
[00:10:42] 
[00:10:42] error[E0425]: cannot find value `tcx` in this scope
[00:10:42]     --> src/librustc_typeck/check/mod.rs:5171:39
[00:10:42]      |
[00:10:42]      |
[00:10:42] 5171 |                         let mut err = tcx.sess.struct_span_err(span,
[00:10:42] 
[00:10:42] error[E0425]: cannot find value `tcx` in this scope
[00:10:42]     --> src/librustc_typeck/check/mod.rs:5189:39
[00:10:42]      |
[00:10:42]      |
[00:10:42] 5189 |                         (impl_def_id, tcx.types.err)
[00:10:42] 
[00:10:48] error: aborting due to 7 previous errors
[00:10:48] 
[00:10:48] Some errors occurred: E0425, E0433.
---
[00:13:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:15] expected success, got: exit code: 101
[00:13:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:15] Build completed unsuccessfully in 0:10:28
[00:13:15] Makefile:28: recipe for target 'all' failed
[00:13:15] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0030aa07
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 15:41:23 UTC 2018
