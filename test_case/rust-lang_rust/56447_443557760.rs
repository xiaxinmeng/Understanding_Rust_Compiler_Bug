plain
travis_time:end:21b204be:start=1543791871065679542,finish=1543791926927771954,duration=55862092412
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
[00:59:30] 
[00:59:30] running 119 tests
[00:59:33] i..ii...iii..iiii.....i...i.........i..iii.............i......i....ii...i..i.ii..............i...ii. 100/119
[00:59:33] .ii.i.....iiii.....
[00:59:33] 
[00:59:33]  finished in 3.559
[00:59:33] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:48] 
[00:59:48] running 118 tests
[01:00:13] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:00:17] ......iii.i.....ii
[01:00:17] 
[01:00:17]  finished in 29.105
[01:00:17] travis_fold:end:test_debuginfo

