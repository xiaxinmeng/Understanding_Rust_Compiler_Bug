plain
travis_time:end:078dc523:start=1551371192650870730,finish=1551371194817228159,duration=2166357429
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:07:10]    Compiling parking_lot_core v0.4.0
[00:07:13]     Checking tempfile v3.0.5
[00:07:13]     Checking parking_lot v0.7.1
[00:07:13]     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:07:14] thread 'rustc' panicked at 'src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:07:14] 
[00:07:14] error: internal compiler error: unexpected panic
[00:07:14] 
[00:07:14] note: the compiler unexpectedly panicked. this is a bug.
---
travis_time:end:0d54513e:start=1551371642111547774,finish=1551371642116531848,duration=4984074
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:068239b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:098209c8
travis_time:start:098209c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c6b7951
$ dmesg | grep -i kill
