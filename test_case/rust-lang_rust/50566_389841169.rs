plain
[00:03:20]       Memory: 8 GB
[00:03:20]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:20]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:20]       SMC Version (system): 2.8f0
[00:03:20]       Serial Number (system): VMh02kWn2/xG
[00:03:20] 
[00:03:20] hw.ncpu: 4
[00:03:20] hw.byteorder: 1234
[00:03:20] hw.memsize: 8589934592
---
  0     0    0     0    0     0      0      0 --:--:--  0:01:15 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:16 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:17 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:01:18 --:--:--     0curl: (7) Failed to connect to s3-us-west-1.amazonaws.com port 443: Operation timed out
[01:01:04] thread 'main' panicked at 'failed to download openssl source: exit code: 7', bootstrap/native.rs:575:17
[01:01:04] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[01:01:04] Build completed unsuccessfully in 0:48:12
[01:01:04] Build completed unsuccessfully in 0:48:12
[01:01:04] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fdde1e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:0a4915e8
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25 19:20 ..
drwx------   2 travis  staff   68 Dec  6 11:00 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0301de80
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:0301de80:start=1526558106946739000,finish=1526558107008585000,duration=61846000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:360c0f00
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.4

Done. Your build exited with 1.
