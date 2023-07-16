plain
[00:47:18] 
[00:47:21] error[E0308]: mismatched types
[00:47:21]   --> libstd/sys/wasm/time.rs:55:28
[00:47:21]    |
[00:47:21] 55 |         self.0.checked_add(other).map(|d| SystemTime(d))
[00:47:21]    |                            |
[00:47:21]    |                            expected struct `core::time::Duration`, found reference
[00:47:21]    |                            expected struct `core::time::Duration`, found reference
[00:47:21]    |                            help: consider dereferencing the borrow: `*other`
[00:47:21]    = note: expected type `core::time::Duration`
[00:47:21]               found type `&core::time::Duration`
[00:47:21] 
[00:47:21] error: aborting due to previous error
---
[00:47:21] 
[00:47:21] To learn more, run the command again with --verbose.
[00:47:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:47:21] expected success, got: exit code: 101
[00:47:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:47:21] travis_fold:end:stage2-std

[00:47:21] travis_time:end:stage2-std:start=1542196390227746782,finish=1542196431045796594,duration=40818049812

---
travis_time:end:2340eba1:start=1542196432134229568,finish=1542196432142253004,duration=8023436
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:271807b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f8b6803
travis_time:start:0f8b6803
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:044cbe56
$ dmesg | grep -i kill
