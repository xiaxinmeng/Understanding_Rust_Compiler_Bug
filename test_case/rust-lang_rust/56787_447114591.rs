plain
travis_time:end:089c1140:start=1544727477699948191,finish=1544727578310518803,duration=100610570612
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
[01:24:59] 
[01:24:59] running 121 tests
[01:25:02] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[01:25:03] i..ii.i.....iiii.....
[01:25:03] 
[01:25:03]  finished in 3.667
[01:25:03] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:18] 
[01:25:18] running 119 tests
[01:25:44] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[01:25:48] i......iii.i.....ii
[01:25:48] 
[01:25:48]  finished in 29.612
[01:25:48] travis_fold:end:test_debuginfo

---
[01:45:05] thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1656:13
[01:45:05] thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1642:13
[01:45:05] thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1688:13
[01:45:05] thread '<unnamed>' panicked at 'specified instant was later than self', src/libstd/sys/unix/time.rs:298:17
[01:45:05] ........F.......F...........................................a: Instant { tv_sec: 6487, tv_nsec: 379536891 }
[01:45:05] dur: 92ns
_64-unknown-linux-gnu/release
128420 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
128416 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
