plain
travis_time:end:20236aeb:start=1546710288980257518,finish=1546710289847796099,duration=867538581
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
[01:11:43] 
[01:11:43] running 118 tests
[01:12:09] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:14] ......iii.i.....ii
[01:12:14] 
[01:12:14]  finished in 31.168
[01:12:14] travis_fold:end:test_debuginfo

---
[01:25:19] 
[01:25:19]    Doc-tests core
[01:25:26] 
[01:25:26] running 2226 tests
[01:25:38] ......iiiii......................................................................................... 100/2226
[01:25:50] .......................................................F............................................ 200/2226
[01:26:23] ......................................................i............................................. 400/2226
[01:26:36] .................................................................................................... 500/2226
[01:26:50] .................................................................................................... 600/2226
[01:27:03] .................................................................................................... 700/2226
---
[01:31:03] ---- cmp.rs - cmp::PartialEq (line 98) stdout ----
[01:31:03] error[E0308]: mismatched types
[01:31:03]   --> cmp.rs:113:24
[01:31:03]    |
[01:31:03] 18 |         self.format == *other
[01:31:03]    |                        ^^^^^^ expected struct `main::Book`, found enum `main::BookFormat`
[01:31:03]    = note: expected type `main::Book`
[01:31:03]    = note: expected type `main::Book`
[01:31:03]               found type `main::BookFormat`
[01:31:03] 
[01:31:03] error[E0609]: no field `format` on type `&main::BookFormat`
[01:31:03]   --> cmp.rs:120:24
[01:31:03]    |
[01:31:03] 25 |         *other == self.format
[01:31:03] 
[01:31:03] thread 'cmp.rs - cmp::PartialEq (line 98)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:31:03] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:31:03] 
---
[01:31:03] 
[01:31:03] error: test failed, to rerun pass '--doc'
[01:31:03] 
[01:31:03] 
[01:31:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:31:03] 
[01:31:03] 
[01:31:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:03] Build completed unsuccessfully in 0:31:35
[01:31:03] Build completed unsuccessfully in 0:31:35
[01:31:03] Makefile:48: recipe for target 'check' failed
[01:31:03] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b7916c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan  5 19:16:04 UTC 2019
---
travis_time:end:022cc656:start=1546715766317601604,finish=1546715766323461094,duration=5859490
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:153102d7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!check
