plain
travis_time:end:0679d760:start=1550022544328494600,finish=1550022546783521036,duration=2455026436
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:45:47]    Compiling parking_lot_core v0.3.0
[00:45:48]    Compiling parking_lot v0.6.4
[00:45:51]    Compiling tempfile v3.0.5
[00:45:52]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:45:52] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:45:52] 
[00:45:52] error: internal compiler error: unexpected panic
[00:45:52] 
[00:45:52] note: the compiler unexpectedly panicked. this is a bug.
---
[00:45:52] 
[00:45:52] 
[00:45:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:45:52] Build completed unsuccessfully in 0:42:15
[00:45:52] make: *** [all] Error 1
[00:45:52] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:188bfa91
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 02:35:09 UTC 2019
