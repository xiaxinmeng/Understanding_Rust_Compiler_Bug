plain
travis_time:end:2e195da0:start=1543953065982088445,finish=1543953138489016061,duration=72506927616
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
[00:54:54] 
[00:54:54] running 120 tests
[00:54:57] i..ii...iii...iiii....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:54:57] ..ii.i.....iiii.....
[00:54:57] 
[00:54:57]  finished in 3.374
[00:54:57] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:11] 
[00:55:11] running 118 tests
[00:55:34] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:38] ......iii.i.....ii
[00:55:38] 
[00:55:38]  finished in 26.892
[00:55:38] travis_fold:end:test_debuginfo

---
[01:04:36] error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[01:04:36] 
[01:04:36] 
[01:04:36] running 408 tests
[01:04:53] .............................................F...................................................... 100/408
[01:05:16] .................................................................................................... 300/408
[01:05:27] .................................................................................................... 400/408
' failed
' failed
[01:05:28] make: *** [check] Error 1
58684 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps
56896 ./src/llvm/test/MC
56108 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build
55740 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro
---
travis_time:end:2d0bca8d:start=1543957078483039607,finish=1543957078486971550,duration=3931943
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:122cfb59
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/
