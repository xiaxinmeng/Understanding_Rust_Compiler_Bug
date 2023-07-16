plain
travis_time:end:05a20520:start=1545099560596306402,finish=1545099561662433965,duration=1066127563
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
[00:56:29] 
[00:56:29] running 119 tests
[00:56:53] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:56:57] i......iii.i.....ii
[00:56:57] 
[00:56:57]  finished in 28.026
[00:56:57] travis_fold:end:test_debuginfo

---
[01:09:23] ......iiiii......................................................................................... 100/2218
[01:09:34] .................................................................................................... 200/2218
[01:09:48] .................................................................................................... 300/2218
[01:10:02] .....................................................i.............................................. 400/2218
[01:10:12] ...........................F........................................................................ 500/2218
[01:10:35] .................................................................................................... 700/2218
[01:10:46] .................................................................................................... 800/2218
[01:10:57] .................................................................................................... 900/2218
[01:11:08] .................................................................................................... 1000/2218
---
[01:12:15] .................................................................................................... 1600/2218
[01:12:27] .................................................................................................... 1700/2218
[01:12:39] .................................................................................................... 1800/2218
[01:12:51] .................................................................................................... 1900/2218
[01:13:03] .........F.......................................................................................... 2000/2218
[01:13:35] ..................
[01:13:35] failures:
[01:13:35] 
[01:13:35] ---- marker.rs - marker::Unpin (line 623) stdout ----
[01:13:35] ---- marker.rs - marker::Unpin (line 623) stdout ----
[01:13:35] error: the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable
[01:13:35]  --> marker.rs:623:12
[01:13:35] 3 | #![feature(pin)]
[01:13:35]   |            ^^^
[01:13:35]   |
[01:13:35] note: lint level defined here
---
[01:13:35] 
[01:13:35] thread 'marker.rs - marker::Unpin (line 623)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:13:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:13:35] 
[01:13:35] ---- pin.rs - pin (line 45) stdout ----
[01:13:35] error[E0599]: no function or associated item named `get_mut_unchecked` found for type `std::pin::Pin<_>` in the current scope
[01:13:35]   --> pin.rs:81:13
[01:13:35]    |
[01:13:35] 39 |             Pin::get_mut_unchecked(mut_ref).slice = slice;
[01:13:35]    |             ^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `std::pin::Pin<_>`
[01:13:35] 
[01:13:35] thread 'pin.rs - pin (line 45)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:13:35] 
[01:13:35] failures:
[01:13:35]     marker.rs - marker::Unpin (line 623)
[01:13:35]     pin.rs - pin (line 45)
---
183724 ./obj/build/x86_64-unknown-linux-gnu/test/ui
160388 ./obj/build/bootstrap/debug/incremental
153292 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7owzz0jdy-pxvx5k-2sxk29axi4qzn
137420 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
128696 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128692 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
126304 ./obj/build/x86_64-unknown-linux-gnu/stage1-rust
