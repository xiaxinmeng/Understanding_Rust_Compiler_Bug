plain
[00:04:20]       Memory: 8 GB
[00:04:20]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:20]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:20]       SMC Version (system): 2.8f0
[00:04:20]       Serial Number (system): VMnxD3YpEhpP
[00:04:20] 
[00:04:20] hw.ncpu: 4
[00:04:20] hw.byteorder: 1234
[00:04:20] hw.memsize: 8589934592
---
[00:06:24]    Compiling panic_unwind v0.0.0 (/Users/travis/build/rust-lang/rust/src/libpanic_unwind)
[00:06:24] [RUSTC-TIMING] rustc_asan test:false 0.246
[00:06:24] [RUSTC-TIMING] rustc_tsan test:false 0.246
[00:06:24] [RUSTC-TIMING] panic_unwind test:false 0.470
[00:06:25] error[E0425]: cannot find function `execvpe` in module `libc`
[00:06:25]    --> libstd/sys/unix/process/process_unix.rs:267:27
[00:06:25]     |
[00:06:25] 267 |                     libc::execvpe(path.as_ptr(), self.get_argv().as_ptr(), envp.as_ptr());
[00:06:25]     |                           ^^^^^^^ did you mean `execve`?
[00:06:25] 
[00:06:25] error[E0425]: cannot find function `execvpe` in module `libc`
[00:06:25]    --> libstd/sys/unix/process/process_unix.rs:294:23
[00:06:25]     |
[00:06:25] 294 |                 libc::execvpe(
[00:06:25]     |                       ^^^^^^^ did you mean `execve`?
[00:06:30] error: aborting due to 2 previous errors
[00:06:30] 
[00:06:30] For more information about this error, try `rustc --explain E0425`.
[00:06:30] [RUSTC-TIMING] std test:false 5.848
[00:06:30] [RUSTC-TIMING] std test:false 5.848
[00:06:30] error: Could not compile `std`.
[00:06:30] 
[00:06:30] To learn more, run the command again with --verbose.
[00:06:30] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:30] expected success, got: exit code: 101
[00:06:30] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:06:30] travis_fold:end:stage0-std

[00:06:30] travis_time:end:stage0-std:start=1541015848599647000,finish=1541015909081582000,duration=60481935000


[00:06:30] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:06:30] Build completed unsuccessfully in 0:01:02
[00:06:30] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0161c9f6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:000a41b4
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:057931c0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:057931c0:start=1541015912640144000,finish=1541015912657808000,duration=17664000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c386145
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17906f8e
travis_time:start:17906f8e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01bd2e20
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
