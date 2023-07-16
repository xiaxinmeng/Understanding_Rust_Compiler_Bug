plain
travis_time:end:33bdd66e:start=1554246474995518721,finish=1554246550537942024,duration=75542423303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:48] 
[01:22:48] running 9 tests
[01:22:48] iiiiiiiii
[01:22:48] 
[01:22:48]  finished in 0.150
[01:22:48] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:04] 
[01:23:04] running 121 tests
[01:23:29] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:34] i.i......iii.i.....ii
[01:23:34] 
[01:23:34]  finished in 29.946
[01:23:34] travis_fold:end:test_debuginfo

---
[01:42:57] ............................i.i.....................................iiii........ii.................. 500/998
[01:43:05] .................................................................................................... 600/998
[01:43:13] .................................................................................................... 700/998
[01:43:21] ....................iiii............................................................................ 800/998
[01:43:35] ..........................F......................................................................... 900/998
[01:43:43] failures:
[01:43:43] 
[01:43:43] ---- sync/mpsc/mod.rs - sync::mpsc::SyncSender<T>::try_send (line 989) stdout ----
[01:43:43] ---- sync/mpsc/mod.rs - sync::mpsc::SyncSender<T>::try_send (line 989) stdout ----
[01:43:43] error: unused return value of `std::result::Result::<T, E>::is_err` that must be used
[01:43:43]   --> sync/mpsc/mod.rs:1008:5
[01:43:43]    |
[01:43:43] 22 |     sync_sender2.try_send(3).is_err();
[01:43:43]    |
[01:43:43] note: lint level defined here
[01:43:43]   --> sync/mpsc/mod.rs:987:9
[01:43:43]    |
[01:43:43]    |
[01:43:43] 1  | #![deny(warnings)]
[01:43:43]    |         ^^^^^^^^
[01:43:43]    = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:43:43] error: aborting due to previous error
[01:43:43] 
[01:43:43] thread 'sync/mpsc/mod.rs - sync::mpsc::SyncSender<T>::try_send (line 989)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:43:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
[01:43:43] 
[01:43:43] error: test failed, to rerun pass '--doc'
[01:43:43] 
[01:43:43] 
[01:43:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:43:43] 
[01:43:43] 
[01:43:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:43] Build completed unsuccessfully in 0:32:29
[01:43:43] Build completed unsuccessfully in 0:32:29
[01:43:43] Makefile:48: recipe for target 'check' failed
[01:43:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:195bab52
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 00:53:03 UTC 2019
