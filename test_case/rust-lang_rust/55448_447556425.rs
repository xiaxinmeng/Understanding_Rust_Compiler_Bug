plain
travis_time:end:014fe288:start=1544864775536698405,finish=1544864832701652752,duration=57164954347
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
[00:51:28] 
[00:51:28] running 121 tests
[00:51:30] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:31] i..ii.i.....iiii.....
[00:51:31] 
[00:51:31]  finished in 3.278
[00:51:31] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:45] 
[00:51:45] running 119 tests
[00:52:07] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:11] i......iii.i.....ii
[00:52:11] 
[00:52:11]  finished in 26.281
[00:52:11] travis_fold:end:test_debuginfo

---
[01:02:11] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:11]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:02:16] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:16]      |
[01:02:16]      |
[01:02:16] 1097 |     let mut rng = XorShiftRng::from_entropy();
[01:02:16]      |
[01:02:16]      = note: `-D deprecated` implied by `-D warnings`
[01:02:16] 
[01:02:16] 
[01:02:16] error: use of deprecated item 'rand::XorShiftRng': import from rand_xorshift crate instead
[01:02:16]      |
[01:02:16]      |
[01:02:16] 1095 |     use rand::{FromEntropy, Rng, XorShiftRng};
[01:02:16] 
