plain
travis_time:end:1480ff98:start=1542078454825909941,finish=1542078457084667265,duration=2258757324
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:33] .................................................................................................... 100/5017
[00:52:36] .................................................................................................... 200/5017
[00:52:39] .............................ii............................................ii...................ii.. 300/5017
[00:52:41] ..............................................................................................iii... 400/5017
[00:52:44] .....iiiiiiii.iii............................iii...........................................i........ 500/5017
[00:52:51] .................................................................................................... 700/5017
[00:52:58] .................................................................................i...........i...... 800/5017
[00:53:01] .................................................................................................... 900/5017
[00:53:05] iiiii..................ii.iiii...................................................................... 1000/5017
---
[00:53:41] .................................................................................................... 2200/5017
[00:53:46] .................................................................................................... 2300/5017
[00:53:50] .................................................................................................... 2400/5017
[00:53:53] .................................................................................................... 2500/5017
[00:53:57] .................................................................................iiiiiiiii.......... 2600/5017
[00:54:05] ..............................................ii.................................................... 2800/5017
[00:54:07] .................................................................................................... 2900/5017
[00:54:11] .................................................................................................... 3000/5017
[00:54:14] ........................................i........................................................... 3100/5017
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:22] 
[01:08:22] running 116 tests
[01:08:25] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:08:25] i.i....iiii.....
[01:08:25] 
[01:08:25]  finished in 3.583
[01:08:25] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:40] 
[01:08:40] running 118 tests
[01:09:06] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:09:10] ......iii.i.....ii
[01:09:10] 
[01:09:10]  finished in 30.049
[01:09:10] travis_fold:end:test_debuginfo

---
[01:29:33] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:652:13
[01:29:33] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:617:13
[01:29:33] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:629:13
[01:29:34] thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', libstd/sys_common/remutex.rs:241:13
[01:29:34] ..............................................................................F...F................. 700/784
[01:29:34] thread '<unnamed>' panicked at 'index 2 and/or 4 in `"aé 💩"` do not lie on character boundary', libstd/sys_common/wtf8.rs:784:5
[01:29:34] thread '<unnamed>' panicked at 'index 5 and/or 8 in `"aé 💩"` do not lie on character boundary', libstd/sys_common/wtf8.rs:784:5
[01:29:34] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', libstd/sys_common/wtf8.rs:329:9
[01:29:34] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', libstd/sys_common/wtf8.rs:329:9
---
[01:29:34] thread '<unnamed>' panicked at 'specified instant was later than self', libstd/sys/unix/time.rs:292:17
[01:29:43] ....................................................................................
[01:29:43] failures:
[01:29:43] 
[01:29:43] ---- sys::unix::fs_linux::tests::test_lseek_data stdout ----
[01:29:43] thread 'sys::unix::fs_linux::tests::test_lseek_data' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 38, kind: Other, message: "Function not implemented" }', libcore/result.rs:1009:5
[01:29:43] ---- sys::unix::fs_linux::tests::test_sparse_copy_middle stdout ----
[01:29:43] ---- sys::unix::fs_linux::tests::test_sparse_copy_middle stdout ----
[01:29:43] thread 'sys::unix::fs_linux::tests::test_sparse_copy_middle' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 38, kind: Other, message: "Function not implemented" }', libcore/result.rs:1009:5
[01:29:43] 
[01:29:43] failures:
[01:29:43] failures:
[01:29:43]     sys::unix::fs_linux::tests::test_lseek_data
[01:29:43]     sys::unix::fs_linux::tests::test_sparse_copy_middle
[01:29:43] test result: FAILED. 782 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:29:43] 
[01:29:43] error: test failed, to rerun pass '--lib'
[01:29:43] 
[01:29:43] 
[01:29:43] 
[01:29:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:29:43] 
[01:29:43] 
[01:29:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:43] Build completed unsuccessfully in 0:41:03
[01:29:43] Build completed unsuccessfully in 0:41:03
[01:29:43] Makefile:58: recipe for target 'check' failed
[01:29:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:042e792a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 04:37:31 UTC 2018
