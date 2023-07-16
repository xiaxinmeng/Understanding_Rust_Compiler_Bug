plain
travis_time:end:00847854:start=1544869930868984920,finish=1544869988702588898,duration=57833603978
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
[00:51:46] 
[00:51:46] running 121 tests
[00:51:49] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:49] i..ii.i.....iiii.....
[00:51:49] 
[00:51:49]  finished in 3.405
[00:51:49] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:04] 
[00:52:04] running 119 tests
[00:52:26] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:30] i......iii.i.....ii
[00:52:30] 
[00:52:30]  finished in 26.472
[00:52:30] travis_fold:end:test_debuginfo

---
[01:02:36] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:36]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:02:40] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:40]      |
[01:02:40]      |
[01:02:40] 1097 |     let mut rng = XorShiftRng::from_entropy();
[01:02:40]      |
[01:02:40]      = note: `-D deprecated` implied by `-D warnings`
[01:02:40] 
[01:02:40] 
[01:02:40] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:40]      |
[01:02:40]      |
[01:02:40] 1095 |     use rand::{FromEntropy, Rng, XorShiftRng};
[01:02:40] 
4645780 .
3017408 ./obj
3002100 ./obj/build
---
182612 ./obj/build/x86_64-unknown-linux-gnu/test/ui
159932 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
143832 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
143828 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7lzj03z7i-s93mg4-2fhvkcimqfl5d
136964 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
128504 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128500 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gn/lib
68396 ./src/test
