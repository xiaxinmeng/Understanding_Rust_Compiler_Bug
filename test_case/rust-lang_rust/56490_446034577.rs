plain
travis_time:end:2cb7d614:start=1544486003041141728,finish=1544486006705102857,duration=3663961129
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
[00:57:11] 
[00:57:11] running 120 tests
[00:57:14] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:57:14] ..ii.i.....iiii.....
[00:57:14] 
[00:57:14]  finished in 3.490
[00:57:14] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:29] 
[00:57:29] running 118 tests
[00:57:53] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:57:57] ......iii.i.....ii
[00:57:57] 
[00:57:57]  finished in 28.347
[00:57:57] travis_fold:end:test_debuginfo

