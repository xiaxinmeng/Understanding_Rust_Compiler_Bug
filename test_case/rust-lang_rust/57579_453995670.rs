plain
travis_time:end:282f35c2:start=1547465115795318356,finish=1547465187629756629,duration=71834438273
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
[01:12:24] 
[01:12:24] running 118 tests
[01:12:48] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:53] ......iii.i.....ii
[01:12:53] 
[01:12:53]  finished in 29.111
[01:12:53] travis_fold:end:test_debuginfo

---
[01:25:13] running 2228 tests
[01:25:24] ......iiiii......................................................................................... 100/2228
[01:25:35] .................................................................................................... 200/2228
[01:25:48] .................................................................................................... 300/2228
[01:26:02] ......................................................i................F.F.......................... 400/2228
[01:26:26] .................................................................................................... 600/2228
[01:26:38] .................................................................................................... 700/2228
[01:26:50] .................................................................................................... 800/2228
[01:27:02] .................................................................................................... 900/2228
---
[01:29:47] ...........................................i.......................................................i 2200/2228
[01:29:51] ............................
[01:29:51] failures:
[01:29:51] 
[01:29:51] ---- iter/sources.rs - iter::sources::once_with (line 461) stdout ----
[01:29:51] error[E0658]: use of unstable library feature 'iter_once_with' (see issue #57581)
[01:29:51]   --> iter/sources.rs:473:14
[01:29:51]    |
[01:29:51] 15 | let config = iter::once_with(|| PathBuf::from(".foorc"));
[01:29:51]    |
[01:29:51]    = help: add #![feature(iter_once_with)] to the crate attributes to enable
[01:29:51] 
[01:29:51] thread 'iter/sources.rs - iter::sources::once_with (line 461)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:29:51] thread 'iter/sources.rs - iter::sources::once_with (line 461)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:29:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:51] 
[01:29:51] ---- iter/sources.rs - iter::sources::once_with (line 445) stdout ----
[01:29:51] error[E0658]: use of unstable library feature 'iter_once_with' (see issue #57581)
[01:29:51]  --> iter/sources.rs:449:15
[01:29:51]   |
[01:29:51] 7 | let mut one = iter::once_with(|| 1);
[01:29:51]   |
[01:29:51]   = help: add #![feature(iter_once_with)] to the crate attributes to enable
[01:29:51] 
[01:29:51] thread 'iter/sources.rs - iter::sources::once_with (line 445)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
---
[01:29:51] 
[01:29:51] error: test failed, to rerun pass '--doc'
[01:29:51] 
[01:29:51] 
[01:29:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:29:51] 
[01:29:51] 
[01:29:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:51] Build completed unsuccessfully in 0:29:16
[01:29:51] Build completed unsuccessfully in 0:29:16
[01:29:51] Makefile:48: recipe for target 'check' failed
[01:29:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2cf72b48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 14 12:56:28 UTC 2019
---
travis_time:end:2b2867c8:start=1547470590084017054,finish=1547470590088611563,duration=4594509
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06a2e0cc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:
