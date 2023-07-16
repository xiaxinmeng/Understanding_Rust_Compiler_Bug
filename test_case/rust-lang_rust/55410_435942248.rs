plain
[00:03:20]       Memory: 8 GB
[00:03:20]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:20]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:20]       SMC Version (system): 2.8f0
[00:03:20]       Serial Number (system): VMoRtbJeDfbI
[00:03:20] 
[00:03:20] hw.ncpu: 4
[00:03:20] hw.byteorder: 1234
[00:03:20] hw.memsize: 8589934592
---
[00:05:13]    Compiling panic_unwind v0.0.0 (/Users/travis/build/rust-lang/rust/src/libpanic_unwind)
[00:05:14] [RUSTC-TIMING] rustc_tsan test:false 0.264
[00:05:14] [RUSTC-TIMING] rustc_asan test:false 0.264
[00:05:14] [RUSTC-TIMING] panic_unwind test:false 0.494
[00:05:15] error[E0412]: cannot find type `AtomicI128` in module `atomic`
[00:05:15]     |
[00:05:15]     |
[00:05:15] 269 | impl RefUnwindSafe for atomic::AtomicI128 {}
[00:05:15]     |                                ^^^^^^^^^^ did you mean `AtomicI16`?
[00:05:15] error[E0412]: cannot find type `AtomicU128` in module `atomic`
[00:05:15]    --> libstd/panic.rs:288:32
[00:05:15]     |
[00:05:15]     |
[00:05:15] 288 | impl RefUnwindSafe for atomic::AtomicU128 {}
[00:05:15]     |                                ^^^^^^^^^^ did you mean `AtomicU16`?
[00:05:20] error: aborting due to 2 previous errors
[00:05:20] 
[00:05:20] For more information about this error, try `rustc --explain E0412`.
[00:05:20] [RUSTC-TIMING] std test:false 5.866
[00:05:20] [RUSTC-TIMING] std test:false 5.866
[00:05:20] error: Could not compile `std`.
[00:05:20] 
[00:05:20] To learn more, run the command again with --verbose.
[00:05:20] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:20] expected success, got: exit code: 101
[00:05:20] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:05:20] travis_fold:end:stage0-std

[00:05:20] travis_time:end:stage0-std:start=1541435636340274000,finish=1541435692490151000,duration=56149877000


[00:05:20] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:05:20] Build completed unsuccessfully in 0:00:58
[00:05:20] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:084c0ae8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:00d4e75e
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04ed91f2
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:04ed91f2:start=1541435695482778000,finish=1541435695505612000,duration=22834000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a56e645
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c6d2412
travis_time:start:1c6d2412
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:266a7780
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
