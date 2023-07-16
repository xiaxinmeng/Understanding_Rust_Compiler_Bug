plain
travis_time:end:05fba768:start=1550696384926645461,finish=1550696486158318672,duration=101231673211
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
[01:16:00] 
[01:16:00] running 119 tests
[01:16:25] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:16:29] i......iii.i.....ii
[01:16:29] 
[01:16:29]  finished in 28.512
[01:16:29] travis_fold:end:test_debuginfo

---
[01:30:53] .................................................................................................... 1900/2269
[01:31:07] .................................................................................................... 2000/2269
[01:31:23] .................................................................................................... 2100/2269
[01:31:40] .................................................................................i.................. 2200/2269
[01:31:49] ........................................FF.FF........................
[01:31:49] 
[01:31:49] ---- time.rs - time::Duration::MICROSECOND (line 90) stdout ----
[01:31:49] ---- time.rs - time::Duration::MICROSECOND (line 90) stdout ----
[01:31:49] error[E0658]: use of unstable library feature 'duration_constants' (see issue #57391)
[01:31:49]  --> time.rs:93:12
[01:31:49]   |
[01:31:49] 6 | assert_eq!(Duration::MICROSECOND, Duration::from_micros(1));
[01:31:49]   |
[01:31:49]   = help: add #![feature(duration_constants)] to the crate attributes to enable
[01:31:49] 
[01:31:49] thread 'time.rs - time::Duration::MICROSECOND (line 90)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] thread 'time.rs - time::Duration::MICROSECOND (line 90)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:31:49] 
[01:31:49] ---- time.rs - time::Duration::MILLISECOND (line 78) stdout ----
[01:31:49] error[E0658]: use of unstable library feature 'duration_constants' (see issue #57391)
[01:31:49]  --> time.rs:81:12
[01:31:49]   |
[01:31:49] 6 | assert_eq!(Duration::MILLISECOND, Duration::from_millis(1));
[01:31:49]   |
[01:31:49]   = help: add #![feature(duration_constants)] to the crate attributes to enable
[01:31:49] 
[01:31:49] thread 'time.rs - time::Duration::MILLISECOND (line 78)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] thread 'time.rs - time::Duration::MILLISECOND (line 78)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] 
[01:31:49] ---- time.rs - time::Duration::NANOSECOND (line 102) stdout ----
[01:31:49] error[E0658]: use of unstable library feature 'duration_constants' (see issue #57391)
[01:31:49]  --> time.rs:105:12
[01:31:49]   |
[01:31:49] 6 | assert_eq!(Duration::NANOSECOND, Duration::from_nanos(1));
[01:31:49]   |
[01:31:49]   = help: add #![feature(duration_constants)] to the crate attributes to enable
[01:31:49] 
[01:31:49] thread 'time.rs - time::Duration::NANOSECOND (line 102)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] thread 'time.rs - time::Duration::NANOSECOND (line 102)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:31:49] 
[01:31:49] ---- time.rs - time::Duration::SECOND (line 66) stdout ----
[01:31:49] error[E0658]: use of unstable library feature 'duration_constants' (see issue #57391)
[01:31:49]  --> time.rs:69:12
[01:31:49]   |
[01:31:49] 6 | assert_eq!(Duration::SECOND, Duration::from_secs(1));
[01:31:49]   |
[01:31:49]   = help: add #![feature(duration_constants)] to the crate attributes to enable
[01:31:49] 
[01:31:49] thread 'time.rs - time::Duration::SECOND (line 66)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
---
[01:31:49] 
[01:31:49] error: test failed, to rerun pass '--doc'
[01:31:49] 
[01:31:49] 
[01:31:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:31:49] 
[01:31:49] 
[01:31:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:49] Build completed unsuccessfully in 0:27:01
[01:31:49] Build completed unsuccessfully in 0:27:01
[01:31:49] make: *** [check] Error 1
[01:31:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2607f246
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 22:33:25 UTC 2019
---
travis_time:end:0919f182:start=1550702007407071609,finish=1550702007411498413,duration=4426804
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14deaea9
$ ln -s . checkout && for CORE 
