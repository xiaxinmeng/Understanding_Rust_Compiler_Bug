plain
[00:05:10]       Memory: 8 GB
[00:05:10]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:05:10]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:05:10]       SMC Version (system): 2.8f0
[00:05:10]       Serial Number (system): VMNvGv7MQMCs
[00:05:10] 
[00:05:10] hw.ncpu: 4
[00:05:10] hw.byteorder: 1234
[00:05:10] hw.memsize: 8589934592
---
travis_time:start:0a034991
make -j 4 tidy
[00:05:11]     Finished dev [unoptimized] target(s) in 0.51s
[00:05:13] fatal: unable to access 'https://github.com/rust-lang/rust.git/': Could not resolve host: github.com
[00:05:13] thread 'main' panicked at 'command did not execute successfully: "git" "ls-remote" "origin" "beta"
[00:05:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:13] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:13] Build completed unsuccessfully in 0:00:02
[00:05:13] Build completed unsuccessfully in 0:00:02
[00:05:13] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04ceb502
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:029fe515
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:06f1c132
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:06f1c132:start=1538405784833458000,finish=1538405784854431000,duration=20973000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c94915e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:006e9d54
travis_time:start:006e9d54
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02bdc73a
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
