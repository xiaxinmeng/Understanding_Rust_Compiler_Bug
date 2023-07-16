plain
travis_time:end:12a58d2c:start=1549719733334205875,finish=1549719890558858916,duration=157224653041
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:47:24]    Compiling parking_lot_core v0.3.0
[00:47:25]    Compiling parking_lot v0.6.4
[00:47:28]    Compiling tempfile v3.0.5
[00:47:29]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:47:29] thread 'rustc' panicked at 'src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:47:29] 
[00:47:29] error: internal compiler error: unexpected panic
[00:47:29] 
[00:47:29] note: the compiler unexpectedly panicked. this is a bug.
---
[00:47:29] 
[00:47:29] To learn more, run the command again with --verbose.
[00:47:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:47:29] Build completed unsuccessfully in 0:43:57
[00:47:29] Makefile:18: recipe for target 'all' failed
[00:47:29] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0657ae76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 14:32:28 UTC 2019
---
travis_time:end:1222715c:start=1549722749831874971,finish=1549722749836800641,duration=4925670
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:265d8c68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04946d99
travis_time:start:04946d99
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:081d943a
$ dmesg | grep -i kill
