plain
[00:48:26] ....................................................................................................
[00:48:29] ....................................................................................................
[00:48:31] ..........i.........................................................................................
[00:48:34] ....................................................................................................
[00:48:37] ...........................................................iiiiiiiii................................
[00:48:43] ....................................................................................................
[00:48:46] ....................................................................................................
[00:48:49] .......................................i............................................................
[00:48:52] .........................................................................................i.i..ii....
---
[01:06:18] .................................................................................................... 1600/2152
[01:06:25] .................................................................................................... 1700/2152
[01:06:33] .................................................................................................... 1800/2152
[01:06:41] .................................................................................................... 1900/2152
[01:06:49] ...................................F................................................................ 2000/2152
[01:07:03] ............................i.......................
[01:07:03] failures:
[01:07:03] 
[01:07:03] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:07:03] ---- result.rs - result::Result<T, E>::map_or_else (line 487) stdout ----
[01:07:03] error[E0282]: type annotations needed
[01:07:03]  --> result.rs:491:27
[01:07:03]   |
[01:07:03] 7 | assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);
[01:07:03]   |                           ^ consider giving this closure parameter a type
[01:07:03] thread 'result.rs - result::Result<T, E>::map_or_else (line 487)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:07:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:03] 
[01:07:03] 
---
[01:07:03] 
[01:07:03] error: test failed, to rerun pass '--doc'
[01:07:03] 
[01:07:03] 
[01:07:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:07:03] 
[01:07:03] 
[01:07:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:03] Build completed unsuccessfully in 0:22:40
[01:07:03] Build completed unsuccessfully in 0:22:40
[01:07:03] Makefile:58: recipe for target 'check' failed
[01:07:03] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ef3e815
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ae45d04:start=1535555343492462760,finish=1535555343499777622,duration=7314862
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049a2b90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a420338
travis_time:start:0a420338
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b80048
$ dmesg | grep -i kill
