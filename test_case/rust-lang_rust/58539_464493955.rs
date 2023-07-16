plain
travis_time:end:2e348a1d:start=1550423806825871078,finish=1550423807581839013,duration=755967935
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
[01:12:36] 
[01:12:36] running 119 tests
[01:13:01] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:06] i......iii.i.....ii
[01:13:06] 
[01:13:06]  finished in 29.442
[01:13:06] travis_fold:end:test_debuginfo

---
[01:31:11] ..........iii......i......i...i......i.............................................................. 300/992
[01:31:15] .................................................................................................... 400/992
[01:31:24] ........................i.i.....................................iiii........ii...................... 500/992
[01:31:31] .................................................................................................... 600/992
[01:31:38] .............................................................................................F...... 700/992
[01:32:00] .................................................................................................... 900/992
[01:32:07] ......................................iiii..................................................
[01:32:07] failures:
[01:32:07] 
[01:32:07] 
[01:32:07] ---- path.rs - path::PathBuf::with_capacity (line 1153) stdout ----
[01:32:07] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:07]  --> path.rs:1156:12
[01:32:07] 6 | let path = PathBuf::with_capacity(10);
[01:32:07]   |            ^^^^^^^^^^^^^^^^^^^^^^
[01:32:07]   |
[01:32:07]   |
[01:32:07]   = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:07] 
[01:32:07] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:07]  --> path.rs:1157:21
[01:32:07]   |
[01:32:07] 7 | let capacity = path.capacity();
[01:32:07]   |
[01:32:07]   |
[01:32:07]   = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:07] 
[01:32:07] error[E0658]: use of unstable library feature 'path_buf_capacity' (see issue #58234)
[01:32:07]   --> path.rs:1162:27
[01:32:07]    |
[01:32:07] 12 | assert_eq!(capacity, path.capacity());
[01:32:07]    |
[01:32:07]    |
[01:32:07]    = help: add #![feature(path_buf_capacity)] to the crate attributes to enable
[01:32:07] thread 'path.rs - path::PathBuf::with_capacity (line 1153)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:32:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:32:07] 
[01:32:07] 
---
[01:32:07] 
[01:32:07] error: test failed, to rerun pass '--doc'
[01:32:07] 
[01:32:07] 
[01:32:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:07] 
[01:32:07] 
[01:32:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:07] Build completed unsuccessfully in 0:31:07
[01:32:07] Build completed unsuccessfully in 0:31:07
[01:32:07] Makefile:48: recipe for target 'check' failed
[01:32:07] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20481a4d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 18:49:06 UTC 2019
