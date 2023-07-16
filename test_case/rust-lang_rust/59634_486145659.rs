plain
travis_time:end:043e439b:start=1556092060950429235,finish=1556092154490757877,duration=93540328642
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
[01:17:43] 
[01:17:43] running 9 tests
[01:17:43] iiiiiiiii
[01:17:43] 
[01:17:43]  finished in 0.148
[01:17:43] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:59] 
[01:17:59] running 121 tests
[01:18:23] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:27] i.i......iii.i.....ii
[01:18:27] 
[01:18:27]  finished in 28.706
[01:18:27] travis_fold:end:test_debuginfo

---
[01:45:10] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347) stdout ----
[01:45:10] error[E0578]: cannot find module `foo` in the crate root
[01:45:10]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11349:12
[01:45:10]   |
[01:45:10] 4 |     pub(in foo) struct Bar {
[01:45:10]   |            ^^^ not found in the crate root
[01:45:10] error: aborting due to previous error
[01:45:10] 
[01:45:10] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0704 (line 11347)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:45:10] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:45:10] 
[01:45:10] 
[01:45:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:10] Build completed unsuccessfully in 0:38:52
[01:45:10] Makefile:48: recipe for target 'check' failed
[01:45:10] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3163c752
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 09:34:33 UTC 2019
---
travis_time:end:1961ef2c:start=1556098475596423106,finish=1556098475600931425,duration=4508319
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02b612a7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fe78065
travis_time:start:0fe78065
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04c4d9d2
$ dmesg | grep -i kill
