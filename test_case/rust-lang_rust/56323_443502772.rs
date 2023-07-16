plain
travis_time:end:08778aef:start=1543749022951712085,finish=1543749025413624378,duration=2461912293
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
[00:58:13] 
[00:58:13] running 119 tests
[00:58:16] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:58:17] .ii.i.....iiii.....
[00:58:17] 
[00:58:17]  finished in 3.552
[00:58:17] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:32] 
[00:58:32] running 118 tests
[00:58:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:59:02] ......iii.i.....ii
[00:59:02] 
[00:59:02]  finished in 29.602
[00:59:02] travis_fold:end:test_debuginfo

