plain
travis_time:end:086168f9:start=1546818570894539900,finish=1546818643800232979,duration=72905693079
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
[01:12:19] 
[01:12:19] running 118 tests
[01:12:46] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:51] ......iii.i.....ii
[01:12:51] 
[01:12:51]  finished in 31.394
[01:12:51] travis_fold:end:test_debuginfo

---
travis_time:end:1c17a7e0:start=1546824872330960908,finish=1546824872335806369,duration=4845461
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03358ddf
$ ln -s . checkout && for CORE 
