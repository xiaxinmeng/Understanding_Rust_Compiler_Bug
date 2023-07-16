plain
travis_time:end:05ab8d68:start=1552858074727356956,finish=1552858147531147383,duration=72803790427
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
[01:21:25] 
[01:21:25] running 120 tests
[01:21:51] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:56] .i......iii.i.....ii
[01:21:56] 
[01:21:56]  finished in 30.702
[01:21:56] travis_fold:end:test_debuginfo

---
[01:33:29] ....iiiii........................................................................................... 100/2297
[01:33:42] ......................................................................ii............................ 200/2297
[01:33:56] ..........................................................................................i......... 300/2297
[01:34:13] .................................................................................................... 400/2297
[01:34:26] ......................i..i.F...F.................................................................... 500/2297
[01:34:53] .................................................................................................... 700/2297
[01:35:06] .................................................................................................... 800/2297
[01:35:20] .................................................................................................... 900/2297
[01:35:34] .................................................................................................... 1000/2297
---
[01:38:43] ....i............................................................................................
[01:38:43] failures:
[01:38:43] 
[01:38:43] ---- macros.rs - assert_matches (line 246) stdout ----
[01:38:43] error: cannot find macro `assert_matches!` in this scope
[01:38:43]  --> macros.rs:250:1
[01:38:43]   |
[01:38:43] 7 | assert_matches!(Err(3), Ok(x) | Err(x) if x >= 3);
[01:38:43] 
[01:38:43] 
[01:38:43] error: cannot find macro `assert_matches!` in this scope
[01:38:43]  --> macros.rs:249:1
[01:38:43]   |
[01:38:43] 6 | assert_matches!(Some(3), Some(3) | None | Some(2));
[01:38:43] 
[01:38:43] error: aborting due to 2 previous errors
[01:38:43] 
[01:38:43] thread 'macros.rs - assert_matches (line 246)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:38:43] thread 'macros.rs - assert_matches (line 246)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:38:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:43] 
[01:38:43] ---- macros.rs - debug_assert_matches (line 287) stdout ----
[01:38:43] error: cannot find macro `debug_assert_matches!` in this scope
[01:38:43]  --> macros.rs:291:1
[01:38:43]   |
[01:38:43] 7 | debug_assert_matches!(Err(3), Ok(x) | Err(x) if x >= 3);
[01:38:43]   | ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `debug_assert_eq`
[01:38:43] error: cannot find macro `debug_assert_matches!` in this scope
[01:38:43]  --> macros.rs:290:1
[01:38:43]   |
[01:38:43]   |
[01:38:43] 6 | debug_assert_matches!(Some(3), Some(_));
[01:38:43]   | ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `debug_assert_eq`
[01:38:43] error: aborting due to 2 previous errors
[01:38:43] 
[01:38:43] thread 'macros.rs - debug_assert_matches (line 287)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:38:43] 
---
[01:38:43] 
[01:38:43] error: test failed, to rerun pass '--doc'
[01:38:43] 
[01:38:43] 
[01:38:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:38:43] 
[01:38:43] 
[01:38:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:43] Build completed unsuccessfully in 0:29:26
[01:38:43] Build completed unsuccessfully in 0:29:26
[01:38:43] make: *** [check] Error 1
[01:38:43] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d516c44
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 17 23:08:00 UTC 2019
---
travis_time:end:056e778d:start=1552864082561173174,finish=1552864082623274644,duration=62101470
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3add40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:057d78e0
$ dmesg | grep -i kill
