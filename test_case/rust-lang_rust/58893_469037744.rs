plain
travis_time:end:14626d1c:start=1551623687870688286,finish=1551623688862577225,duration=991888939
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
[01:20:17] 
[01:20:17] running 119 tests
[01:20:46] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:20:51] i......iii.i.....ii
[01:20:51] 
[01:20:51]  finished in 34.471
[01:20:51] travis_fold:end:test_debuginfo

---
[01:40:26] .................................................................................................... 600/994
[01:40:34] .................................................................................................... 700/994
[01:40:43] ..............iiii.................................................................................. 800/994
[01:40:56] .................................................................................................... 900/994
[01:41:04] ........................................iiii.F................................................
[01:41:04] 
[01:41:04] ---- thread/local.rs - thread::local::LocalKey (line 31) stdout ----
[01:41:04] ---- thread/local.rs - thread::local::LocalKey (line 31) stdout ----
[01:41:04] error[E0615]: attempted to take value of method `join` on type `std::thread::JoinHandle<()>`
[01:41:04]   --> thread/local.rs:51:3
[01:41:04]    |
[01:41:04] 23 | t.join.unwrap();
[01:41:04]    |   ^^^^ help: use parentheses to call the method: `join()`
[01:41:04] thread 'thread/local.rs - thread::local::LocalKey (line 31)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:352:13
[01:41:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:41:04] 
[01:41:04] 
---
[01:41:04] 
[01:41:04] error: test failed, to rerun pass '--doc'
[01:41:04] 
[01:41:04] 
[01:41:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:41:04] 
[01:41:04] 
[01:41:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:04] Build completed unsuccessfully in 0:33:20
[01:41:04] Build completed unsuccessfully in 0:33:20
[01:41:04] Makefile:48: recipe for target 'check' failed
[01:41:04] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001009aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar  3 16:16:05 UTC 2019
