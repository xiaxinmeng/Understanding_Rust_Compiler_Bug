plain
[00:02:37]       Memory: 8 GB
[00:02:37]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:37]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:37]       SMC Version (system): 2.8f0
[00:02:37]       Serial Number (system): VMUg+b+dtFYW
[00:02:37] 
[00:02:37] hw.ncpu: 4
[00:02:37] hw.byteorder: 1234
[00:02:37] hw.memsize: 8589934592
---
[01:21:32] 
[01:21:32] error: test failed, to rerun pass '--lib'
[01:21:32] 
[01:21:32] 
[01:21:32] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0/bin/cargo" "test" "--target" "i686-apple-darwin" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "-p" "std" "--"
[01:21:32] 
[01:21:32] 
[01:21:32] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:21:32] Build completed unsuccessfully in 0:32:25
[01:21:32] Build completed unsuccessfully in 0:32:25
[01:21:32] make: *** [check] Error 1
travis_time:end:054176cd:start=1525838044726357000,finish=1525842936815789000,duration=4892089432000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0007335c
---
travis_fold:start:after_failure.2
travis_time:start:11e69f83
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  21 travis  staff    714 May  9 04:53 .
-rw-------@  1 travis  staff  38005 May  9 04:53 stack-probes-lto.stage2-i686-apple-darwin_2018-05-09-045338-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63281 May  9 04:53 stack-probes-lto.stage2-i686-apple-darwin_2018-05-09-045338_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59321 May  9 04:53 stack-probes.stage2-i686-apple-darwin_2018-05-09-045333-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  36744 May  9 04:53 stack-probes.stage2-i686-apple-darwin_2018-05-09-045333_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9716 May  9 04:53 simd-target-feature-mixup.stage2-i686-apple-darwin_2018-05-09-045321_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9467 May  9 04:53 signal-exit-status.stage2-i686-apple-darwin_2018-05-09-045318_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9532 May  9 04:53 segfault-no-out-of-stack.stage2-i686-apple-darwin_2018-05-09-045308_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9331 May  9 04:53 running-with-no-runtime.stage2-i686-apple-darwin_2018-05-09-045307_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9473 May  9 04:52 lto-abort.stage2-i686-apple-darwin_2018-05-09-045233_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9676 May  9 04:52 abort.stage2-i686-apple-darwin_2018-05-09-045230_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10383 May  9 04:52 abort-link-to-unwinding-crates.stage2-i686-apple-darwin_2018-05-09-045229_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  61948 May  9 04:52 out-of-stack.stage2-i686-apple-darwin_2018-05-09-045223_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63188 May  9 04:52 out-of-stack.stage2-i686-apple-darwin_2018-05-09-045218-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62842 May  9 04:52 out-of-stack.stage2-i686-apple-darwin_2018-05-09-045218-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64049 May  9 04:52 out-of-stack.stage2-i686-apple-darwin_2018-05-09-045218_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11076 May  9 04:50 issue-24313.stage2-i686-apple-darwin_2018-05-09-045016_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9935 May  9 04:47 backtrace.stage2-i686-apple-darwin_2018-05-09-044733-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9966 May  9 04:47 backtrace.stage2-i686-apple-darwin_2018-05-09-044733_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9658 May  9 04:47 abort-on-c-abi.stage2-i686-apple-darwin_2018-05-09-044731_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25 19:20 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1f2a5028
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:1f2a5028:start=1525842938357485000,finish=1525842938389890000,duration=32405000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0231c719
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.4

Done. Your build exited with 1.
