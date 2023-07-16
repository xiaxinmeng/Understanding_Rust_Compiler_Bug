plain
travis_time:end:1c325484:start=1548762597386845809,finish=1548762599546598697,duration=2159752888
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:48] 
[01:07:48] running 119 tests
[01:08:11] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:08:15] i......iii.i.....ii
[01:08:15] 
[01:08:15]  finished in 27.384
[01:08:15] travis_fold:end:test_debuginfo

---
[01:17:09] 
[01:17:09] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[01:17:09]     --> src/libcore/../libcore/tests/iter.rs:1317:46
[01:17:09]      |
[01:17:09] 1317 |             CALL_COUNT.store(CALL_COUNT.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
[01:17:09]      |                                              ^^^^^^^^ use of undeclared type or module `Ordering`
[01:17:09] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[01:17:09]     --> src/libcore/../libcore/tests/iter.rs:1317:70
[01:17:09]      |
[01:17:09]      |
[01:17:09] 1317 |             CALL_COUNT.store(CALL_COUNT.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
[01:17:09]      |                                                                      ^^^^^^^^ use of undeclared type or module `Ordering`
[01:17:09] error[E0433]: failed to resolve: use of undeclared type or module `Ordering`
[01:17:09]     --> src/libcore/../libcore/tests/iter.rs:1323:32
[01:17:09]      |
[01:17:09]      |
[01:17:09] 1323 |     assert_eq!(CALL_COUNT.load(Ordering::Relaxed), 2);
[01:17:09]      |                                ^^^^^^^^ use of undeclared type or module `Ordering`
[01:17:09] error[E0412]: cannot find type `AtomicUsize` in this scope
[01:17:09]     --> src/libcore/../libcore/tests/iter.rs:1313:24
[01:17:09]      |
[01:17:09] 1313 |     static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
---
[01:17:19] 
[01:17:19] To learn more, run the command again with --verbose.
[01:17:19] 
[01:17:19] 
[01:17:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:17:19] 
[01:17:19] 
[01:17:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:19] Build completed unsuccessfully in 0:20:30
[01:17:19] Build completed unsuccessfully in 0:20:30
[01:17:19] make: *** [check] Error 1
[01:17:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a00b65
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 29 13:07:28 UTC 2019
