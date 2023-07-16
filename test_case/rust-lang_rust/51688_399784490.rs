plain
[00:02:57]       Memory: 8 GB
[00:02:57]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:57]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:57]       SMC Version (system): 2.8f0
[00:02:57]       Serial Number (system): VMnSXrFHKqSe
[00:02:57] 
[00:02:57] hw.ncpu: 4
[00:02:57] hw.byteorder: 1234
[00:02:57] hw.memsize: 8589934592
---
  0     0    0     0    0     0      0      0 --:--:--  0:01:17 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:18 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:19 --:--:--     0
[00:53:12] curl: (35) LibreSSL SSL_connect: SSL_ERROR_SYSCALL in connection to s3-us-west-1.amazonaws.com:443 
[00:53:12] thread 'main' panicked at 'failed to download openssl source: exit code: 35', bootstrap/native.rs:589:17
[00:53:12] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:53:12] Build completed unsuccessfully in 0:40:24
[00:53:12] Build completed unsuccessfully in 0:40:24
[00:53:12] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c2afd54
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:01b1e939
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25 19:20 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:072a99c8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:072a99c8:start=1529871472452324000,finish=1529871472474290000,duration=21966000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:104ae678
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d123df9
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.5

Done. Your build exited with 1.
