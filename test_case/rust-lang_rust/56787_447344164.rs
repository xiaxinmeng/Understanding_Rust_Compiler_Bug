plain
travis_time:end:248ed1c3:start=1544794194650923229,finish=1544794196186367140,duration=1535443911
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
[00:50:58] 
[00:50:58] running 121 tests
[00:51:00] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:01] i..ii.i.....iiii.....
[00:51:01] 
[00:51:01]  finished in 3.304
[00:51:01] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:15] 
[00:51:15] running 119 tests
[00:51:37] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[00:51:41] i......iii.i.....ii
[00:51:41] 
[00:51:41]  finished in 26.291
[00:51:41] travis_fold:end:test_debuginfo

---
[01:08:24] thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1656:13
[01:08:24] thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1642:13
[01:08:24] thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1688:13
[01:08:24] thread '<unnamed>' panicked at 'specified instant was later than self', src/libstd/sys/unix/time.rs:286:17
[01:08:24] ........F.......F...........................................a: Instant { tv_sec: 4203, tv_nsec: 596951049 }
[01:08:24] dur: 135ns
travis_time:start:1ae1ea3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 14 14:38:38 UTC 2018
---
travis_time:end:11749350:start=1544798319788907819,finish=1544798319795240539,duration=6332720
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0615b694
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE
