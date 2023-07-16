plain
travis_time:end:0be1858c:start=1544996170541545844,finish=1544996235516828042,duration=64975282198
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
[00:52:31] 
[00:52:31] running 121 tests
[00:52:34] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:52:34] i..ii.i.....iiii.....
[00:52:34] 
[00:52:34]  finished in 3.609
[00:52:34] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:49] 
[00:52:49] running 119 tests
[00:53:11] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:53:15] i......iii.i.....ii
[00:53:15] 
[00:53:15]  finished in 26.396
[00:53:15] travis_fold:end:test_debuginfo

---
[01:04:36] running 2213 tests
[01:04:46] .................................................................................................... 100/2213
[01:04:56] .................................................................................................... 200/2213
[01:05:08] .................................................................................................... 300/2213
[01:05:20] ..............................................i........................FFF.......................... 400/2213
[01:05:40] .................................................................................................... 600/2213
[01:05:50] .................................................................................................... 700/2213
[01:06:01] .................................................................................................... 800/2213
[01:06:11] .................................................................................................... 900/2213
