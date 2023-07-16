plain
travis_time:end:1bbb413e:start=1546746815281139193,finish=1546746887192540144,duration=71911400951
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
[01:09:06] 
[01:09:06] running 118 tests
[01:09:30] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:09:34] ......iii.i.....ii
[01:09:34] 
[01:09:34]  finished in 28.252
[01:09:34] travis_fold:end:test_debuginfo

---
[01:21:49] 
[01:21:49]    Doc-tests core
[01:21:54] 
[01:21:54] running 2226 tests
[01:22:05] ......iiiii......................................................................................... 100/2226
[01:22:17] ........................................................F........................................... 200/2226
[01:22:45] ......................................................i............................................. 400/2226
[01:22:57] .................................................................................................... 500/2226
[01:23:09] .................................................................................................... 600/2226
[01:23:21] .................................................................................................... 700/2226
