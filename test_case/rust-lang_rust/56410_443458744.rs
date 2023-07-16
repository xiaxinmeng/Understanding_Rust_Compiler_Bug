plain
travis_time:end:1f2f2f49:start=1543692321136569642,finish=1543692323399010557,duration=2262440915
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:54] 
[00:56:54] running 119 tests
[00:56:57] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:56:57] .ii.i.....iiii.....
[00:56:57] 
[00:56:57]  finished in 3.461
[00:56:57] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:12] 
[00:57:12] running 118 tests
[00:57:36] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:57:40] ......iii.i.....ii
[00:57:40] 
[00:57:40]  finished in 28.444
[00:57:40] travis_fold:end:test_debuginfo

