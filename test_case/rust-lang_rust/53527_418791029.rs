plain
[00:49:49] ....................................................................................................
[00:49:53] ....................................................................................................
[00:49:55] ................................i...................................................................
[00:49:58] ....................................................................................................
[00:50:01] .................................................................................iiiiiiiii..........
[00:50:06] ...ii...............................................................................................
[00:50:10] ....................................................................................................
[00:50:13] ..............................................................i.....................................
[00:50:16] ....................................................................................................
---
[00:50:46] .......................................................................................i............
[00:50:48] ....................................................................................................
[00:50:52] ....................................................................................................
[00:50:54] ....................................................................................................
[00:50:57] .i.ii.ii.ii.............................i...........................................................
[00:50:57] test result: ok. 4137 passed; 0 failed; 64 ignored; 0 measured; 0 filtered out
[00:50:57] 
[00:50:57]  finished in 122.203
[00:50:57] travis_fold:end:test_ui_nll
---
[01:03:07]     Finished release [optimized] target(s) in 5.27s
[01:03:07]      Running build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/test-56e9d567d6eb6f83
[01:03:07] 
[01:03:07] running 38 tests
[01:03:07] ............................F.........
[01:03:07] 
[01:03:07] ---- tests::parse_include_ignored_flag stdout ----
[01:03:07] ---- tests::parse_include_ignored_flag stdout ----
[01:03:07] thread 'tests::parse_include_ignored_flag' panicked at 'called `Result::unwrap()` on an `Err` value: "The \"include-ignored\" flag is only accepted on the nightly compiler"', libcore/result.rs:983:5
[01:03:07] 
[01:03:07] 
[01:03:07] failures:
[01:03:07]     tests::parse_include_ignored_flag
[01:03:07]     tests::parse_include_ignored_flag
[01:03:07] 
[01:03:07] test result: FAILED. 37 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:07] 
[01:03:07] error: test failed, to rerun pass '--lib'
[01:03:07] 
[01:03:07] 
[01:03:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test" "--" "--quiet"
[01:03:07] 
[01:03:07] 
[01:03:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:07] Build completed unsuccessfully in 0:17:18
[01:03:07] Build completed unsuccessfully in 0:17:18
[01:03:07] Makefile:58: recipe for target 'check' failed
[01:03:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f315da2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:113bef9c:start=1536164222337873463,finish=1536164222345825199,duration=7951736
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00aab8e2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1524f070
travis_time:start:1524f070
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1308e61f
$ dmesg | grep -i kill
