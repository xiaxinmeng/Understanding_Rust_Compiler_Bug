plain
travis_time:end:08596814:start=1543945657198482195,finish=1543945719534306199,duration=62335824004
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:13] 
[00:54:13] running 120 tests
[00:54:16] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:54:16] ..ii.i.....iiii.....
[00:54:16] 
[00:54:16]  finished in 3.325
[00:54:16] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:30] 
[00:54:30] running 118 tests
[00:54:52] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:54:56] ......iii.i.....ii
[00:54:56] 
[00:54:56]  finished in 25.714
[00:54:56] travis_fold:end:test_debuginfo

---
[01:10:08] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:08]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:10:31] error: function is never used: `with_panic_count`
[01:10:31]     |
[01:10:31]     |
[01:10:31] 232 | fn with_panic_count<R>(f: impl FnOnce(&Cell<usize>) -> R) -> R {
[01:10:31]     |
[01:10:31]     = note: `-D dead-code` implied by `-D warnings`
[01:10:31] 
[01:10:32] error: aborting due to previous error
[01:10:32] error: aborting due to previous error
[01:10:32] 
[01:10:32] error: Could not compile `std`.
[01:10:32] 
[01:10:32] To learn more, run the command again with --verbose.
[01:10:32] 
[01:10:32] 
[01:10:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:10:32] 
[01:10:32] 
[01:10:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:32] Build completed unsuccessfully in 0:26:40
[01:10:32] Build completed unsuccessfully in 0:26:40
[01:10:32] make: *** [check] Error 1
[01:10:32] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ec432d7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec  4 18:59:20 UTC 2018
---
travis_time:end:02a809b2:start=1543949962367072489,finish=1543949962372219194,duration=5146705
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05dacfd7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0be5c1a4
travis_time:start:0be5c1a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:036fe9de
$ dmesg | grep -i kill
