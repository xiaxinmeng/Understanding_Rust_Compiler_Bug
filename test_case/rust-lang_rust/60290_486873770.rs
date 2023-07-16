plain
travis_time:end:0c2cfbf8:start=1556228218032630430,finish=1556228342819215173,duration=124786584743
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:07] 
[01:24:07] running 9 tests
[01:24:07] iiiiiiiii
[01:24:07] 
[01:24:07]  finished in 0.148
[01:24:07] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:23] 
[01:24:23] running 121 tests
[01:24:46] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:24:51] i.i......iii.i.....ii
[01:24:51] 
[01:24:51]  finished in 28.260
[01:24:51] travis_fold:end:test_debuginfo

---
[01:53:03] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0671 (line 11021) stdout ----
[01:53:03] error[E0671]: const parameters cannot depend on type parameters
[01:53:03]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11022:25
[01:53:03]   |
[01:53:03] 3 | fn const_id<T, const N: T>() -> T {
[01:53:03]   |                         ^ const parameter depends on type parameter
[01:53:03] error[E0658]: const generics are unstable
[01:53:03]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11022:22
[01:53:03]   |
[01:53:03]   |
[01:53:03] 3 | fn const_id<T, const N: T>() -> T {
[01:53:03]   |
[01:53:03]   = note: for more information, see https://github.com/rust-lang/rust/issues/44580
[01:53:03]   = help: add #![feature(const_generics)] to the crate attributes to enable
[01:53:03] 
---
[01:53:03] 
[01:53:03] 
[01:53:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:03] Build completed unsuccessfully in 0:40:46
[01:53:03] Makefile:48: recipe for target 'check' failed
[01:53:03] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:029d4806
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 23:32:15 UTC 2019
---
travis_time:end:07e91521:start=1556235136697359428,finish=1556235136702259573,duration=4900145
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00192032
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:033b914b
travis_time:start:033b914b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a13e4aa
$ dmesg | grep -i kill
