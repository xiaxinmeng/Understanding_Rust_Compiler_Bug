plain
Building test helpers
[01:07:13] running: "clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-o" "/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers/rust_test_helpers.o" "-c" "/checkout/src/test/auxiliary/rust_test_helpers.c"
[01:07:13] thread 'main' panicked at '
[01:07:13] 
[01:07:13] Internal error occurred: Failed to find tool. Is `clang` installed?
[01:07:13] ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
[01:07:13] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:13] travis_fold:end:build_test_helpers

---
travis_time:end:11ed0a84:start=1554837301500082737,finish=1554837301511668123,duration=11585386
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b918ef8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2eb040c8
travis_time:start:2eb040c8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:106fcb1c
$ dmesg | grep -i kill
