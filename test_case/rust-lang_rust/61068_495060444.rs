plain
travis_time:end:15d7adb4:start=1558579290528273813,finish=1558579379765715640,duration=89237441827
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:12] 
[01:20:12] running 143 tests
[01:20:15] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:20:16] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:20:16] 
[01:20:16]  finished in 4.671
[01:20:16] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:19] 
[01:20:19] running 9 tests
[01:20:19] iiiiiiiii
[01:20:19] 
[01:20:19]  finished in 0.155
[01:20:19] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:35] 
[01:20:35] running 122 tests
[01:21:00] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:21:05] .i.i......iii.i.....ii
[01:21:05] 
[01:21:05]  finished in 30.508
[01:21:05] travis_fold:end:test_debuginfo

---
[01:28:40] travis_fold:start:test_stage1-test
travis_time:start:test_stage1-test
Testing test stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:28:40]    Compiling test v0.0.0 (/checkout/src/libtest)
[01:28:40] error: unused import: `TrFailed`
[01:28:40]      |
[01:28:40]      |
[01:28:40] 1781 |         ShouldPanic, StaticTestName, TestDesc, TestDescAndFn, TestOpts, TrFailed, TrFailedMsg,
[01:28:40]      |
[01:28:40]      = note: `-D unused-imports` implied by `-D warnings`
[01:28:40] 
[01:28:40] error[E0308]: mismatched types
[01:28:40] error[E0308]: mismatched types
[01:28:40]     --> src/libtest/lib.rs:1926:36
[01:28:40]      |
[01:28:40] 1926 |         assert!(res == TrFailedMsg("test did not panic as expected"));
[01:28:40]      |                                    |
[01:28:40]      |                                    expected struct `std::string::String`, found reference
[01:28:40]      |                                    expected struct `std::string::String`, found reference
[01:28:40]      |                                    help: try using a conversion method: `"test did not panic as expected".to_string()`
[01:28:40]      = note: expected type `std::string::String`
[01:28:40]                 found type `&'static str`
[01:28:40] 
[01:28:41] error: aborting due to 2 previous errors
[01:28:41] error: aborting due to 2 previous errors
[01:28:41] 
[01:28:41] For more information about this error, try `rustc --explain E0308`.
[01:28:41] error: Could not compile `test`.
[01:28:41] 
[01:28:41] To learn more, run the command again with --verbose.
[01:28:41] 
[01:28:41] 
[01:28:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:28:41] 
[01:28:41] 
[01:28:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:41] Build completed unsuccessfully in 0:20:14
[01:28:41] Build completed unsuccessfully in 0:20:14
[01:28:41] Makefile:48: recipe for target 'check' failed
[01:28:41] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:011dc570
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 23 04:11:50 UTC 2019
---
travis_time:end:001c7dd0:start=1558584711828073782,finish=1558584711832593293,duration=4519511
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fc9e31a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ee30c94
travis_time:start:1ee30c94
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e1b5ba5
$ dmesg | grep -i kill
