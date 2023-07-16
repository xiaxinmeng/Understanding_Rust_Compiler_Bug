plain
[00:02:23]       Memory: 8 GB
[00:02:23]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:23]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:23]       SMC Version (system): 2.8f0
[00:02:23]       Serial Number (system): VMCwqzKbYhNn
[00:02:23] 
[00:02:23] hw.ncpu: 4
[00:02:23] hw.byteorder: 1234
[00:02:23] hw.memsize: 8589934592
---
Building stage2 tool cargo (x86_64-apple-darwin)
[00:46:20]  Downloading crates ...
[00:46:20] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[00:46:20] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[00:46:20] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[00:46:20] Caused by:
[00:46:20]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[00:46:20] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[00:46:20] expected success, got: exit code: 101
[00:46:20] expected success, got: exit code: 101
[00:46:20] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:46:20] Build completed unsuccessfully in 0:42:56
[00:46:20] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a619118
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 14 20:35:32 GMT 2018
---
travis_fold:start:after_failure.2
travis_time:start:09820e60
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0f312d1a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0f312d1a:start=1544819740476893000,finish=1544819740492970000,duration=16077000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1631322c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:023a9b32
travis_time:start:023a9b32
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05e6f9fe
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
