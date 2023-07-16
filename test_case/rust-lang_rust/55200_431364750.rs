plain
[00:48:10] .................................................................................................... 2200/4630
[00:48:15] ................................i................................................................... 2300/4630
[00:48:18] .................................................................................................... 2400/4630
[00:48:22] .................................................................................................... 2500/4630
[00:48:25] ..............................................iiiiiiiii............................................. 2600/4630
[00:48:31] .................................................................................................... 2800/4630
[00:48:35] .................................................................................................... 2900/4630
[00:48:38] ...........................................................................i........................ 3000/4630
[00:48:40] .................................................................................................... 3100/4630
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:55] 
[01:00:55] running 111 tests
[01:00:58] i..ii...iii.......i....i........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:00:58] ..iiii.....
[01:00:58] 
[01:00:58]  finished in 3.330
[01:00:58] travis_fold:end:test_codegen

---
[01:19:07] .................................................................................................... 400/959
[01:19:18] ...................................iiii........ii................................................... 500/959
[01:19:23] .................................................................................................... 600/959
[01:19:33] ...................................................................................iiii............. 700/959
[01:19:47] ....................................................F............................................... 800/959
[01:20:00] ........iiii...............................................
[01:20:00] failures:
[01:20:00] 
[01:20:00] ---- process.rs - process::Stdio::from (line 1086) stdout ----
[01:20:00] ---- process.rs - process::Stdio::from (line 1086) stdout ----
[01:20:00] thread 'process.rs - process::Stdio::from (line 1086)' panicked at 'test executable failed:
[01:20:00] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 30, kind: Other, message: "Read-only file system" }', libcore/result.rs:1009:5
[01:20:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:00] 
[01:20:00] ', librustdoc/test.rs:367:17
---
[01:20:00] 
[01:20:00] error: test failed, to rerun pass '--doc'
[01:20:00] 
[01:20:00] 
[01:20:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:20:00] 
[01:20:00] 
[01:20:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:00] Build completed unsuccessfully in 0:36:20
[01:20:00] Build completed unsuccessfully in 0:36:20
[01:20:00] Makefile:58: recipe for target 'check' failed
[01:20:00] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18496908
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:17600cc3:start=1539956037532587825,finish=1539956037536521195,duration=3933370
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014842d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07892790
travis_time:start:07892790
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:067c6e7f
$ dmesg | grep -i kill
