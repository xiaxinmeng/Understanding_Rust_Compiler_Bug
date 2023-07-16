plain
travis_time:end:14906344:start=1549778784349191054,finish=1549778882372121993,duration=98022930939
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:45:44]    Compiling parking_lot_core v0.3.0
[00:45:46]    Compiling parking_lot v0.6.4
[00:45:48]    Compiling tempfile v3.0.5
[00:45:49]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:45:50] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:45:50] 
[00:45:50] error: internal compiler error: unexpected panic
[00:45:50] 
[00:45:50] note: the compiler unexpectedly panicked. this is a bug.
---
[00:45:50] 
[00:45:50] 
[00:45:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:45:50] Build completed unsuccessfully in 0:42:29
[00:45:50] make: *** [all] Error 1
[00:45:50] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:30332ec7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 06:54:01 UTC 2019
---
travis_time:end:086402dc:start=1549781642686087394,finish=1549781642690366879,duration=4279485
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06e03622
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0039fa8a
