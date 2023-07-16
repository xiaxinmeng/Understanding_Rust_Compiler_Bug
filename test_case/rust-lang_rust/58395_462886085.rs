plain
travis_time:end:1c26aff9:start=1549992238258355348,finish=1549992360566712373,duration=122308357025
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
[01:06:40] 
[01:06:40] running 119 tests
[01:07:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:07:09] i......iii.i.....ii
[01:07:09] 
[01:07:09]  finished in 29.025
[01:07:09] travis_fold:end:test_debuginfo

---
[01:25:05] .................................................................................................... 600/988
[01:25:11] .................................................................................................... 700/988
[01:25:20] .........iiii....................................................................................... 800/988
[01:25:30] .................................................................................................... 900/988
[01:25:36] ...................................iiii...................................F.............
[01:25:36] 
[01:25:36] ---- time.rs - time::Instant::checked_duration_since (line 226) stdout ----
[01:25:36] ---- time.rs - time::Instant::checked_duration_since (line 226) stdout ----
[01:25:36] error[E0658]: use of unstable library feature 'checked_duration_since' (see issue #58402)
[01:25:36]   --> time.rs:233:26
[01:25:36]    |
[01:25:36] 10 | println!("{:?}", new_now.checked_duration_since(now));
[01:25:36]    |
[01:25:36]    |
[01:25:36]    = help: add #![feature(checked_duration_since)] to the crate attributes to enable
[01:25:36] 
[01:25:36] error[E0658]: use of unstable library feature 'checked_duration_since' (see issue #58402)
[01:25:36]   --> time.rs:234:22
[01:25:36]    |
[01:25:36] 11 | println!("{:?}", now.checked_duration_since(new_now)); // None
[01:25:36]    |
[01:25:36]    |
[01:25:36]    = help: add #![feature(checked_duration_since)] to the crate attributes to enable
[01:25:36] thread 'time.rs - time::Instant::checked_duration_since (line 226)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:25:36] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:25:36] 
[01:25:36] 
