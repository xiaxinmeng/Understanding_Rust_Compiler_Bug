plain
travis_time:end:0ebb2427:start=1546791004728620765,finish=1546791084487855236,duration=79759234471
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
[01:10:39] 
[01:10:39] running 118 tests
[01:11:04] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:11:09] ......iii.i.....ii
[01:11:09] 
[01:11:09]  finished in 29.811
[01:11:09] travis_fold:end:test_debuginfo

---
[01:23:40] 
[01:23:40]    Doc-tests core
[01:23:46] 
[01:23:46] running 2226 tests
[01:23:58] ......iiiii......................................................................................... 100/2226
[01:24:09] .......................................................F............................................ 200/2226
[01:24:39] ......................................................i............................................. 400/2226
[01:24:51] .................................................................................................... 500/2226
[01:25:04] .................................................................................................... 600/2226
[01:25:17] .................................................................................................... 700/2226
