plain
travis_time:end:0b17cf95:start=1554844080182348266,finish=1554844081220549697,duration=1038201431
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:17] 
[01:17:17] running 9 tests
[01:17:17] iiiiiiiii
[01:17:17] 
[01:17:17]  finished in 0.164
[01:17:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:35] 
[01:17:35] running 121 tests
[01:18:05] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:18:11] i.i......iii.i.....ii
[01:18:11] 
[01:18:11]  finished in 36.180
[01:18:11] travis_fold:end:test_debuginfo

---
[01:38:46] i................................................................................................... 100/999
[01:39:01] .................................................................................................... 200/999
[01:39:11] ............iii......i......i...i......i............................................................ 300/999
[01:39:16] .................................................................................................... 400/999
[01:39:26] ............................i.i.................................F....iiii........ii................. 500/999
[01:39:43] .................................................................................................... 700/999
[01:39:53] .....................iiii........................................................................... 800/999
[01:40:08] .................................................................................................... 900/999
[01:40:17] .............................................iiii..................................................
[01:40:17] .............................................iiii..................................................
[01:40:17] failures:
[01:40:17] 
[01:40:17] ---- macros.rs - dbg (line 320) stdout ----
[01:40:17] error[E0423]: expected function, found macro `assert_eq`
[01:40:17]  --> macros.rs:321:1
[01:40:17]   |
[01:40:17] 4 | assert_eq(dbg!(1usize, 2usize), (1, 2));
[01:40:17]   | ^^^^^^^^^ help: use `!` to invoke the macro: `assert_eq!`
[01:40:17] error: aborting due to previous error
[01:40:17] 
[01:40:17] For more information about this error, try `rustc --explain E0423`.
[01:40:17] thread 'macros.rs - dbg (line 320)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[01:40:17] 
[01:40:17] error: test failed, to rerun pass '--doc'
[01:40:17] 
[01:40:17] 
[01:40:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:40:17] 
[01:40:17] 
[01:40:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:17] Build completed unsuccessfully in 0:36:00
[01:40:17] Build completed unsuccessfully in 0:36:00
[01:40:17] Makefile:48: recipe for target 'check' failed
[01:40:17] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12cecbf8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  9 22:48:29 UTC 2019
---
travis_time:end:04718703:start=1554850111921619544,finish=1554850111927409688,duration=5790144
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b70c7e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:138b2998
travis_time:start:138b2998
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c9b233f
$ dmesg | grep -i kill
