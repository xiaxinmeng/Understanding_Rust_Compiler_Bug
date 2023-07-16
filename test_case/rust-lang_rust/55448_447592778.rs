plain
travis_time:end:0013ac16:start=1544898895283853520,finish=1544898948700520062,duration=53416666542
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:37] 
[00:51:37] running 121 tests
[00:51:40] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:40] i..ii.i.....iiii.....
[00:51:40] 
[00:51:40]  finished in 3.364
[00:51:40] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:54] 
[00:51:54] running 119 tests
[00:52:17] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:21] i......iii.i.....ii
[00:52:21] 
[00:52:21]  finished in 26.476
[00:52:21] travis_fold:end:test_debuginfo

---
[01:02:30] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:30]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:02:34] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:34]      |
[01:02:34]      |
[01:02:34] 1097 |     let mut rng = XorShiftRng::from_entropy();
[01:02:34]      |
[01:02:34]      = note: `-D deprecated` implied by `-D warnings`
[01:02:34] 
[01:02:34] 
[01:02:34] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:34]      |
[01:02:34]      |
[01:02:34] 1095 |     use rand::{FromEntropy, Rng, XorShiftRng};
[01:02:34] 
4647684 .
3019384 ./obj
3004056 ./obj/build
---
182732 ./obj/build/x86_64-unknown-linux-gnu/test/ui
160528 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
144428 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144424 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7mctrj43q-11zvdmy-4zt1yysir9st
137068 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
128608 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128604 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gn
