plain
travis_time:end:20520578:start=1549521777318023954,finish=1549521778209728281,duration=891704327
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:48:27]    Compiling parking_lot_core v0.3.0
[00:48:29]    Compiling parking_lot v0.6.4
[00:48:32]    Compiling tempfile v3.0.5
[00:48:33]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:48:33] thread 'rustc' panicked at 'src/librustc/hir/def.rs:258: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:48:33] 
[00:48:33] error: internal compiler error: unexpected panic
[00:48:33] 
[00:48:33] note: the compiler unexpectedly panicked. this is a bug.
---
[00:48:33] 
[00:48:33] 
[00:48:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:48:33] Build completed unsuccessfully in 0:44:32
[00:48:33] Makefile:18: recipe for target 'all' failed
[00:48:33] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11cca9ec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 07:31:42 UTC 2019
---
travis_time:end:16db2c9e:start=1549524702951178843,finish=1549524702955956990,duration=4778147
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060b1c54
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
