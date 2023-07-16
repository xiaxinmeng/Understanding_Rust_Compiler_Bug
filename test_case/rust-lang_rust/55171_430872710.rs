plain
[00:45:49]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:45:49] [RUSTC-TIMING] panic_unwind test:false 0.268
[00:45:56] warning: redundant linker flag specified for library `zircon`
[00:45:56] 
[00:46:04] error: linker `rust-lld` not found
[00:46:04]   = note: No such file or directory (os error 2)
[00:46:04] 
[00:46:04] error: aborting due to previous error
[00:46:04] 
[00:46:04] 
[00:46:04] [RUSTC-TIMING] std test:false 14.423
[00:46:04] error: Could not compile `std`.
[00:46:04] 
[00:46:04] To learn more, run the command again with --verbose.
[00:46:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-fuchsia" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:46:04] expected success, got: exit code: 101
[00:46:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:46:04] travis_fold:end:stage2-std

[00:46:04] travis_time:end:stage2-std:start=1539837344098452871,finish=1539837399792131420,duration=55693678549

---
travis_time:end:00431a2a:start=1539837401006859023,finish=1539837401013978524,duration=7119501
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0592f34c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08bdc61e
travis_time:start:08bdc61e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e2f7b60
$ dmesg | grep -i kill
