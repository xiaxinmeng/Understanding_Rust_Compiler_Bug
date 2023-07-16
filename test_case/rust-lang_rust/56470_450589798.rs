plain
travis_time:end:1afdc8e2:start=1546202850939503346,finish=1546202907856209913,duration=56916706567
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
[01:06:40] 
[01:06:40] running 118 tests
[01:07:03] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:07:06] ......iii.i.....ii
[01:07:06] 
[01:07:06]  finished in 26.990
[01:07:06] travis_fold:end:test_debuginfo

---
[01:12:10] 
[01:12:10] running 289 tests
[01:13:13] ..........................i......................................................................... 100/289
[01:14:06] .....................................i.............................................................. 200/289
[01:14:54] ................................F........................................................
[01:14:54] 
[01:14:54] ---- [rustdoc] rustdoc/process-termination.rs stdout ----
[01:14:54] 
[01:14:54] error: rustdoc failed!
[01:14:54] error: rustdoc failed!
[01:14:54] status: exit code: 101
[01:14:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/tes
