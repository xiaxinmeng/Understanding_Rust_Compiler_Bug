plain
[00:47:12] ....................................................................................................
[00:47:16] ....................................................................................................
[00:47:18] ..........i.........................................................................................
[00:47:21] ....................................................................................................
[00:47:23] ...........................................................iiiiiiiii................................
[00:47:29] ....................................................................................................
[00:47:32] ....................................................................................................
[00:47:35] .......................................i............................................................
[00:47:38] .........................................................................................i.i..ii....
---
[01:04:42] .................................................................................................... 1600/2152
[01:04:49] .................................................................................................... 1700/2152
[01:04:56] .................................................................................................... 1800/2152
[01:05:03] .................................................................................................... 1900/2152
[01:05:11] ...................................F................................................................ 2000/2152
[01:05:25] ............................i.......................
[01:05:25] failures:
[01:05:25] 
[01:05:25] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:05:25] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:05:25] error[E0282]: type annotations needed
[01:05:25]   --> result.rs:494:38
[01:05:25]    |
[01:05:25] 10 | assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);
[01:05:25]    |                                      ^ consider giving this closure parameter a type
[01:05:25]    = note: type must be known at this point
[01:05:25] 
[01:05:25] thread 'result.rs - result::Result<T, E>::map_or_else (line 487)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:05:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:05:25] 
[01:05:25] error: test failed, to rerun pass '--doc'
[01:05:25] 
[01:05:25] 
[01:05:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:05:25] 
[01:05:25] 
[01:05:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:25] Build completed unsuccessfully in 0:22:05
[01:05:25] Build completed unsuccessfully in 0:22:05
[01:05:25] make: *** [check] Error 1
[01:05:25] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9c96e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c2fe1e8:start=1535548620080251469,finish=1535548620088120664,duration=7869195
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:120d0f92
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c12d998
travis_time:start:0c12d998
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f813166
$ dmesg | grep -i kill
