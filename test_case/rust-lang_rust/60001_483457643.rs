plain
[00:03:09]       Memory: 8 GB
[00:03:09]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:09]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:09]       SMC Version (system): 2.8f0
[00:03:09]       Serial Number (system): VMCDriMuXUZC
[00:03:09] 
[00:03:09] hw.ncpu: 4
[00:03:09] hw.byteorder: 1234
[00:03:09] hw.memsize: 8589934592
---
[01:17:40]    Compiling rustfix v0.4.4
[01:17:42] [RUSTC-TIMING] rustfix test:false 1.988
[01:17:42]    Compiling compiletest v0.0.0 (/Users/travis/build/rust-lang/rust/src/tools/compiletest)
[01:17:42] error: the item `libc` is imported redundantly
[01:17:42]   --> src/tools/compiletest/src/raise_fd_limit.rs:10:9
[01:17:42] 10 |     use libc;
[01:17:42]    |         ^^^^
[01:17:42]    | 
[01:17:42]   ::: src/tools/compiletest/src/main.rs:7:1
[01:17:42]   ::: src/tools/compiletest/src/main.rs:7:1
[01:17:42]    |
[01:17:42] 7  | extern crate libc;
[01:17:42]    | ------------------ the item `libc` is already imported here
[01:17:42]    |
[01:17:42] note: lint level defined here
[01:17:42]   --> src/tools/compiletest/src/main.rs:4:9
[01:17:42]    |
[01:17:42] 4  | #![deny(warnings, rust_2018_idioms)]
[01:17:42]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:17:42] 
[01:17:43] error: aborting due to previous error
[01:17:43] 
---
[01:17:43] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0/bin/cargo" "build" "--target" "i686-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/compiletest/Cargo.toml" "--message-format" "json"
[01:17:43] expected success, got: exit code: 101
[01:17:43] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:17:43] Build completed unsuccessfully in 0:01:50
[01:17:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0233886c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 15 23:39:26 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:00b3f24a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:05faaf98
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:05faaf98:start=1555371569734129000,finish=1555371569761930000,duration=27801000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c91d2b2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1611f047
travis_time:start:1611f047
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0224127c
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
