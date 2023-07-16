plain
[00:47:26] .................................................................................................... 2200/4605
[00:47:30] ...................i................................................................................ 2300/4605
[00:47:34] .................................................................................................... 2400/4605
[00:47:37] .................................................................................................... 2500/4605
[00:47:41] ................................iiiiiiiii........................................................... 2600/4605
[00:47:47] .................................................................................................... 2800/4605
[00:47:51] .................................................................................................... 2900/4605
[00:47:54] ......................................................i............................................. 3000/4605
[00:47:57] .................................................................................................... 3100/4605
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:38] 
[01:00:38] running 111 tests
[01:00:41] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:00:41] ..iiii.....
[01:00:41] 
[01:00:41]  finished in 3.387
[01:00:41] travis_fold:end:test_codegen

---
[01:15:27]  right: `2`', libstd/sync/mutex.rs:661:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/once.rs:582:28
[01:15:27] thread '<unnamed>' panicked at 'Once instance has previously been poisoned', libstd/sync/once.rs:372:21
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/once.rs:610:28
[01:15:27] thread '<unnamed>' panicked at 'assertion failed: t1.join().is_ok()', libstd/sync/once.rs:638:9
[01:15:27] thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', libstd/sync/rwlock.rs:774:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:711:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:641:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:652:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:652:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:617:13
[01:15:27] thread '<unnamed>' panicked at 'explicit panic', libstd/sync/rwlock.rs:629:13
[01:15:28] thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', libstd/sys_common/remutex.rs:241:13
[01:15:28] .......................................F............................................................ 700/768
[01:15:28] thread '<unnamed>' panicked at 'index 2 and/or 4 in `"aé 💩"` do not lie on character boundary', libstd/sys_common/wtf8.rs:784:5
[01:15:28] thread '<unnamed>' panicked at 'index 5 and/or 8 in `"aé 💩"` do not lie on character boundary', libstd/sys_common/wtf8.rs:784:5
[01:15:28] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', libstd/sys_common/wtf8.rs:329:9
[01:15:28] thread '<unnamed>' panicked at 'assertion failed: is_code_point_boundary(self, new_len)', libstd/sys_common/wtf8.rs:329:9
---
[01:15:37] 
[01:15:37] error: test failed, to rerun pass '--lib'
[01:15:37] 
[01:15:37] 
[01:15:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:15:37] 
[01:15:37] 
[01:15:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:37] Build completed unsuccessfully in 0:32:49
[01:15:37] Build completed unsuccessfully in 0:32:49
[01:15:37] Makefile:58: recipe for target 'check' failed
[01:15:37] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2f0cee8e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1a7273f0:start=1539765920243640848,finish=1539765920364819527,duration=121178679
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1070e04b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12ccf678
$ dmesg | grep -i kill
