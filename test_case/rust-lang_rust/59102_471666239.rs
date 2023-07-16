plain
travis_time:end:0e3a0cba:start=1552323551164601104,finish=1552323553676602783,duration=2512001679
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
[01:18:24] 
[01:18:24] running 120 tests
[01:18:49] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:18:53] .i......iii.i.....ii
[01:18:53] 
[01:18:53]  finished in 28.838
[01:18:53] travis_fold:end:test_debuginfo

---
[01:33:31] .................................................................................................... 1900/2293
[01:33:44] .................................................................................................... 2000/2293
[01:33:59] .................................................................................................... 2100/2293
[01:34:17] .................................................................................................... 2200/2293
[01:34:30] i.....................................................................................F......
[01:34:30] 
[01:34:30] ---- time.rs - time::Duration::mul_f32 (line 636) stdout ----
[01:34:30] thread 'time.rs - time::Duration::mul_f32 (line 636)' panicked at 'test executable failed:
[01:34:30] 
[01:34:30] 
[01:34:30] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:34:30]   left: `847799.969120256s`,
[01:34:30]  right: `847800.064s`', time.rs:11:1
[01:34:30] 
[01:34:30] ', src/librustdoc/test.rs:332:17
[01:34:30] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:30] 
---
[01:34:30] 
[01:34:30] error: test failed, to rerun pass '--doc'
[01:34:30] 
[01:34:30] 
[01:34:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:34:30] 
[01:34:30] 
[01:34:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:30] Build completed unsuccessfully in 0:27:27
[01:34:30] Build completed unsuccessfully in 0:27:27
[01:34:30] make: *** [check] Error 1
[01:34:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01014330
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 11 18:33:54 UTC 2019
