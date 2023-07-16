plain
[00:46:42] ....................................................................................................
[00:46:46] ....................................................................................................
[00:46:48] ..........i.........................................................................................
[00:46:51] ....................................................................................................
[00:46:54] ...........................................................iiiiiiiii................................
[00:46:59] ....................................................................................................
[00:47:03] ....................................................................................................
[00:47:05] .......................................i............................................................
[00:47:08] .........................................................................................i.i..ii....
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:48] 
[00:53:48] running 106 tests
[00:53:51] i..ii..iii....i...i............iii...........i.....i....ii...i.i.ii..............i...ii..i.ii....iii
[00:53:51] test result: ok. 77 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out
[00:53:51] 
[00:53:51]  finished in 3.162
[00:53:51] travis_fold:end:test_codegen
---
[01:03:50] .................................................................................................... 1600/2152
[01:03:57] .................................................................................................... 1700/2152
[01:04:04] .................................................................................................... 1800/2152
[01:04:11] .................................................................................................... 1900/2152
[01:04:18] ...................................F................................................................ 2000/2152
[01:04:31] ............................i.......................
[01:04:31] failures:
[01:04:31] 
[01:04:31] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:04:31] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:04:31] error[E0282]: type annotations needed
[01:04:31]  --> result.rs:491:27
[01:04:31]   |
[01:04:31] 7 | assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
[01:04:31]   |                           ^ consider giving this closure parameter a type
[01:04:31] thread 'result.rs - result::Result<T, E>::map_or_else (line 487)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:04:31] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:31] 
[01:04:31] 
---
[01:04:31] 
[01:04:31] error: test failed, to rerun pass '--doc'
[01:04:31] 
[01:04:31] 
[01:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:04:31] 
[01:04:31] 
[01:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:31] Build completed unsuccessfully in 0:21:42
[01:04:31] Build completed unsuccessfully in 0:21:42
[01:04:31] make: *** [check] Error 1
[01:04:31] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2066c1f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0d9b3f44:start=1535559834271132324,finish=1535559834278133080,duration=7000756
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:117eaa61
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0350311f
travis_time:start:0350311f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12b17353
$ dmesg | grep -i kill
