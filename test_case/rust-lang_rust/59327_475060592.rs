plain
travis_time:end:0001d9a6:start=1553116679222868218,finish=1553116680578103403,duration=1355235185
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
[01:17:55] 
[01:17:55] running 9 tests
[01:17:55] iiiiiiiii
[01:17:55] 
[01:17:55]  finished in 0.157
[01:17:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:11] 
[01:18:11] running 120 tests
[01:18:37] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:18:42] .i......iii.i.....ii
[01:18:42] 
[01:18:42]  finished in 30.508
[01:18:42] travis_fold:end:test_debuginfo

---
[01:36:59]    Doc-tests std
[01:37:01] 
[01:37:01] running 996 tests
[01:37:24] i................................................................................................... 100/996
[01:37:37] ...................................................................................F................ 200/996
[01:37:51] .................................................................................................... 400/996
[01:38:00] ..........................i.i.....................................iiii........ii.................... 500/996
[01:38:08] .................................................................................................... 600/996
[01:38:16] .................................................................................................... 700/996
---
[01:38:46] ---- f64.rs - f64::f64::clamp (line 890) stdout ----
[01:38:46] error[E0308]: mismatched types
[01:38:46]  --> f64.rs:895:31
[01:38:46]   |
[01:38:46] 8 | assert!((std::f64::NAN).clamp(-2.0f32, 1.0f32).is_nan());
[01:38:46]   |                               ^^^^^^^ expected f64, found f32
[01:38:46] help: you can cast an `f32` to `f64` in a lossless way
[01:38:46]   |
[01:38:46] 8 | assert!((std::f64::NAN).clamp((-2.0f32).into(), 1.0f32).is_nan());
[01:38:46] 
[01:38:46] error[E0308]: mismatched types
[01:38:46]  --> f64.rs:895:40
[01:38:46]   |
[01:38:46]   |
[01:38:46] 8 | assert!((std::f64::NAN).clamp(-2.0f32, 1.0f32).is_nan());
[01:38:46]   |                                        ^^^^^^ expected f64, found f32
[01:38:46] help: change the type of the numeric literal from `f32` to `f64`
[01:38:46]   |
[01:38:46] 8 | assert!((std::f64::NAN).clamp(-2.0f32, 1.0f64).is_nan());
[01:38:46] 
[01:38:46] error: aborting due to 2 previous errors
[01:38:46] 
[01:38:46] For more information about this error, try `rustc --explain E0308`.
---
[01:38:46] 
[01:38:46] error: test failed, to rerun pass '--doc'
[01:38:46] 
[01:38:46] 
[01:38:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:38:46] 
[01:38:46] 
[01:38:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:46] Build completed unsuccessfully in 0:32:24
[01:38:46] Build completed unsuccessfully in 0:32:24
[01:38:46] make: *** [check] Error 1
[01:38:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:025ed713
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 22:56:57 UTC 2019
---
travis_time:end:15915ef4:start=1553122619268325861,finish=1553122619328820229,duration=60494368
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003baa38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16fb80a6
$ dmesg | grep -i kill
