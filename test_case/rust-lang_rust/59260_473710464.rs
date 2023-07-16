plain
travis_time:end:09e7ade8:start=1552847650688491171,finish=1552847724640775798,duration=73952284627
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:14] 
[01:23:14] running 120 tests
[01:23:40] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:23:45] .i......iii.i.....ii
[01:23:45] 
[01:23:45]  finished in 30.527
[01:23:45] travis_fold:end:test_debuginfo

---
[01:35:13] ....iiiii........................................................................................... 100/2297
[01:35:26] ......................................................................ii............................ 200/2297
[01:35:40] ..........................................................................................i......... 300/2297
[01:35:57] .................................................................................................... 400/2297
[01:36:09] ......................i..i.F...F.................................................................... 500/2297
[01:36:36] .................................................................................................... 700/2297
[01:36:49] .................................................................................................... 800/2297
[01:37:02] .................................................................................................... 900/2297
[01:37:15] .................................................................................................... 1000/2297
---
[01:40:24] ....i............................................................................................
[01:40:24] failures:
[01:40:24] 
[01:40:24] ---- macros.rs - assert_matches (line 246) stdout ----
[01:40:24] error: cannot find macro `assert_matches!` in this scope
[01:40:24]  --> macros.rs:250:1
[01:40:24]   |
[01:40:24] 7 | assert_matches!(Err(3), Ok(x) | Err(x) if x >= 3);
[01:40:24] 
[01:40:24] 
[01:40:24] error: cannot find macro `assert_matches!` in this scope
[01:40:24]  --> macros.rs:249:1
[01:40:24]   |
[01:40:24] 6 | assert_matches!(Some(3), Some(3) | None | Some(2));
[01:40:24] 
[01:40:24] error: aborting due to 2 previous errors
[01:40:24] 
[01:40:24] thread 'macros.rs - assert_matches (line 246)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:40:24] thread 'macros.rs - assert_matches (line 246)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:40:24] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:40:24] 
[01:40:24] ---- macros.rs - debug_assert_matches (line 287) stdout ----
[01:40:24] error: cannot find macro `debug_assert_matches!` in this scope
[01:40:24]  --> macros.rs:291:1
[01:40:24]   |
[01:40:24] 7 | debug_assert_matches!(Err(3), Ok(x) | Err(x) if x >= 3);
[01:40:24]   | ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `debug_assert_eq`
[01:40:24] error: cannot find macro `debug_assert_matches!` in this scope
[01:40:24]  --> macros.rs:290:1
[01:40:24]   |
[01:40:24]   |
[01:40:24] 6 | debug_assert_matches!(Some(3), Some(_));
[01:40:24]   | ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `debug_assert_eq`
[01:40:24] error: aborting due to 2 previous errors
[01:40:24] 
[01:40:24] thread 'macros.rs - debug_assert_matches (line 287)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:40:24] 
---
[01:40:24] 
[01:40:24] error: test failed, to rerun pass '--doc'
[01:40:24] 
[01:40:24] 
[01:40:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:40:24] 
[01:40:24] 
[01:40:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:24] Build completed unsuccessfully in 0:29:16
[01:40:24] Build completed unsuccessfully in 0:29:16
[01:40:24] Makefile:48: recipe for target 'check' failed
[01:40:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01dad884
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 17 20:15:58 UTC 2019
---
travis_time:end:0275fde6:start=1552853760392065028,finish=1552853760396841678,duration=4776650
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026a0411
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ee27140
travis_time:start:0ee27140
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2eb72f64
$ dmesg | grep -i kill
