plain
travis_time:end:002d769b:start=1546977835050774444,finish=1546977836096022281,duration=1045247837
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
[01:16:57] 
[01:16:57] running 118 tests
[01:17:23] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:17:27] ......iii.i.....ii
[01:17:27] 
[01:17:27]  finished in 29.667
[01:17:27] travis_fold:end:test_debuginfo

---
[01:37:59] .................................................................................................... 400/991
[01:38:08] ..........................i.i.....................................iiii........ii.................... 500/991
[01:38:15] .................................................................................................... 600/991
[01:38:22] .................................................................................................... 700/991
[01:38:31] .............iiii............F...................................................................... 800/991
 0:33:50
 0:33:50
[01:38:50] make: *** [check] Error 1
[01:38:50] Makefile:48: recipe for target 'check' failed
136260 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
124960 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
124956 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122052 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:134f4c18:start=1546983781478755336,finish=1546983781483668675,duration=4913339
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03477243
$ ln -s . checkout && for CORE in obj/cores/core.*; do Etravis_time:end:1a7569f0:start=1546983781558276453,finish=1546983781569075282,duration=10798829

Done. Your build exited with 1.
