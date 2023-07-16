plain
travis_time:end:0c913244:start=1549556944749216322,finish=1549556946670548214,duration=1921331892
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:52:42]    Compiling rand v0.6.1
[00:52:44]    Compiling parking_lot v0.6.4
[00:52:47]    Compiling tempfile v3.0.5
[00:52:48]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:52:49] thread 'rustc' panicked at 'src/librustc/hir/def.rs:258: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:52:49] 
[00:52:49] error: internal compiler error: unexpected panic
[00:52:49] 
[00:52:49] note: the compiler unexpectedly panicked. this is a bug.
---
[00:52:49] 
[00:52:49] 
[00:52:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:52:49] Build completed unsuccessfully in 0:46:04
[00:52:49] make: *** [all] Error 1
[00:52:49] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b7f73e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 17:22:05 UTC 2019
