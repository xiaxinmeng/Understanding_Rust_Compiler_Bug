plain
travis_time:end:0465d02c:start=1550117983081397819,finish=1550117985735539958,duration=2654142139
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:51:08]    Compiling parking_lot_core v0.3.0
[00:51:09]    Compiling parking_lot v0.6.4
[00:51:12]    Compiling tempfile v3.0.5
[00:51:13]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:51:14] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:51:14] 
[00:51:14] error: internal compiler error: unexpected panic
[00:51:14] 
[00:51:14] note: the compiler unexpectedly panicked. this is a bug.
---
[00:51:14] 
[00:51:14] 
[00:51:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:51:14] Build completed unsuccessfully in 0:45:52
[00:51:14] Makefile:18: recipe for target 'all' failed
[00:51:14] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bb5f250
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 05:11:10 UTC 2019
