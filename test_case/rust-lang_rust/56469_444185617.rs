plain
travis_time:end:2f0a4f2e:start=1543937980588088074,finish=1543938069088120753,duration=88500032679
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
[00:57:54] 
[00:57:54] running 120 tests
[00:57:57] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:57:57] ..ii.i.....iiii.....
[00:57:57] 
[00:57:57]  finished in 3.587
[00:57:57] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:13] 
[00:58:13] running 118 tests
[00:58:39] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:58:43] ......iii.i.....ii
[00:58:43] 
[00:58:43]  finished in 30.396
[00:58:43] travis_fold:end:test_debuginfo

---
[01:17:03] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1009:5
[01:17:03] .................................................................................................... 600/768
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:637:13
[01:17:03] thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:592:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: m.is_poisoned()', src/libstd/sync/mutex.rs:595:9
[01:17:03] thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:569:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: m.is_poisoned()', src/libstd/sync/mutex.rs:572:9
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:17:03]   left: `1`,
[01:17:03]  right: `2`', src/libstd/sync/mutex.rs:661:13
[01:17:03]  right: `2`', src/libstd/sync/mutex.rs:661:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: arc.lock().is_err()', src/libstd/sync/mutex.rs:663:9
[01:17:03] thread '<unnamed>' panicked at 'Once instance has previously been poisoned', src/libstd/sync/once.rs:372:21
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/once.rs:610:28
[01:17:03] thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:797:13
[01:17:03] thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:797:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: m.is_poisoned()', src/libstd/sync/rwlock.rs:800:9
[01:17:03] thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:774:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: m.is_poisoned()', src/libstd/sync/rwlock.rs:777:9
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:641:13
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:652:13
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:617:13
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:617:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: arc.read().is_err()', src/libstd/sync/rwlock.rs:619:9
[01:17:03] thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:629:13
[01:17:03] thread '<unnamed>' panicked at 'assertion failed: arc.write().is_err()', src/libstd/sync/rwlock.rs:631:9
[01:17:04] thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:241:13
[01:17:04] ..........................F..F...F.........F..F....FF..............................F................ 700/768
[01:17:04] thread '<unnamed>' panicked at 'index 2 and/or 4 in `"aé 💩"` do not lie on character boundary', src/libstd/sys_common/wtf8.rs:784:5
[01:17:04] thread '<unnamed>' panicked at 'index 5 and/or 8 in `"aé 💩"` do not lie on character boundary', src/libstd/sys_common/wtf8.rs:784:5
[01:17:04] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', src/libstd/sys_common/wtf8.rs:329:9
[01:17:04] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', src/libstd/sys_common/wtf8.rs:329:9
