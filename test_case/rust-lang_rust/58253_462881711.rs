plain
travis_time:end:0003db62:start=1549993960174776129,finish=1549993962295775663,duration=2120999534
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:47:11]    Compiling parking_lot_core v0.3.0
[00:47:12]    Compiling parking_lot v0.6.4
[00:47:15]    Compiling tempfile v3.0.5
[00:47:16]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:47:16] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:47:16] 
[00:47:16] error: internal compiler error: unexpected panic
[00:47:16] 
[00:47:16] note: the compiler unexpectedly panicked. this is a bug.
---
[00:47:16] 
[00:47:16] 
[00:47:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:47:16] Build completed unsuccessfully in 0:41:23
[00:47:16] Makefile:18: recipe for target 'all' failed
[00:47:16] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0af40cd6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 18:40:12 UTC 2019
---
travis_time:end:069a2840:start=1549996812972288607,finish=1549996812976876863,duration=4588256
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00cbeacc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30a6847d
travis_time:start:30a6847d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2761953a
$ dmesg | grep -i kill
