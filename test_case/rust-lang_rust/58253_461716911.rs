plain
travis_time:end:10cb7cd6:start=1549608657266969159,finish=1549608659827542038,duration=2560572879
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:49:01]    Compiling parking_lot_core v0.3.0
[00:49:03]    Compiling parking_lot v0.6.4
[00:49:05]    Compiling tempfile v3.0.5
[00:49:06]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:49:07] thread 'rustc' panicked at 'src/librustc/hir/def.rs:258: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:49:07] 
[00:49:07] error: internal compiler error: unexpected panic
[00:49:07] 
[00:49:07] note: the compiler unexpectedly panicked. this is a bug.
---
[00:49:07] 
[00:49:07] 
[00:49:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:49:07] Build completed unsuccessfully in 0:45:29
[00:49:07] make: *** [all] Error 1
[00:49:07] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2628da04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  8 07:40:18 UTC 2019
---
travis_time:end:0aac7b12:start=1549611618926361534,finish=1549611618931496911,duration=5135377
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05696628
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c4a9960
travis_time:start:0c4a9960
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22326eee
$ dmesg | grep -i kill
