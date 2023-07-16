plain
[01:16:20] 
[01:16:20] ..............F.
[01:16:20] failures:
[01:16:20] 
[01:16:20] ---- util::common::test_to_readable_str stdout ----
[01:16:20] thread 'util::common::test_to_readable_str' panicked at 'assertion failed: `(left == right)`
[01:16:20]   left: `"1"`,
[01:16:20]  right: `"0_001"`', librustc/util/common.rs:390:5
[01:16:20] 
[01:16:20] 
[01:16:20] failures:
[01:16:20]     util::common::test_to_readable_str
[01:16:20]     util::common::test_to_readable_str
[01:16:20] 
[01:16:20] test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:16:20] 
[01:16:20] error: test failed, to rerun pass '--lib'
[01:16:20] 
[01:16:20] 
[01:16:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:16:20] 
[01:16:20] 
[01:16:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:20] Build completed unsuccessfully in 0:27:42
[01:16:20] Build completed unsuccessfully in 0:27:42
[01:16:20] Makefile:58: recipe for target 'check' failed
[01:16:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1adb39dc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0545067f:start=1532633446224908825,finish=1532633446232158145,duration=7249320
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30728344
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08a6b2bc
travis_time:start:08a6b2bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0107aff4
$ dmesg | grep -i kill
