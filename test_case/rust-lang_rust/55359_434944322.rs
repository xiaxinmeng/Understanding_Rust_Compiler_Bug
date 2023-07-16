plain
[00:46:03] [RUSTC-TIMING] panic_unwind test:false 0.277
[00:46:09] error: method is never used: `get_program`
[00:46:09]    --> libstd/sys/unix/process/process_common.rs:144:5
[00:46:09]     |
[00:46:09] 144 |     pub fn get_program(&self) -> &CString {
[00:46:09]     |
[00:46:09]     = note: `-D dead-code` implied by `-D warnings`
[00:46:09] 
[00:46:09] error: method is never used: `get_items`
[00:46:09] error: method is never used: `get_items`
[00:46:09]    --> libstd/sys/unix/process/process_common.rs:250:5
[00:46:09]     |
[00:46:09] 250 |     pub fn get_items(&self) -> &[CString] {
[00:46:09] 
[00:46:10] error: aborting due to 2 previous errors
[00:46:10] 
[00:46:10] [RUSTC-TIMING] std test:false 7.001
[00:46:10] [RUSTC-TIMING] std test:false 7.001
[00:46:10] error: Could not compile `std`.
[00:46:10] 
[00:46:10] To learn more, run the command again with --verbose.
[00:46:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:46:10] expected success, got: exit code: 101
[00:46:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:46:10] travis_fold:end:stage2-std

[00:46:10] travis_time:end:stage2-std:start=1541053494051581636,finish=1541053548448841551,duration=54397259915

---
travis_time:end:0431e49a:start=1541053549758025496,finish=1541053549765891037,duration=7865541
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:179b2398
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:016ce597
travis_time:start:016ce597
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2df64b54
$ dmesg | grep -i kill
