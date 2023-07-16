plain
travis_time:end:33bc9f16:start=1543817953872979004,finish=1543818009818555471,duration=55945576467
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
[00:57:00] 
[00:57:00] running 119 tests
[00:57:03] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:57:04] .ii.i.....iiii.....
[00:57:04] 
[00:57:04]  finished in 3.356
[00:57:04] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:18] 
[00:57:18] running 118 tests
[00:57:41] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:57:45] ......iii.i.....ii
[00:57:45] 
[00:57:45]  finished in 27.413
[00:57:45] travis_fold:end:test_debuginfo

