plain
[01:30:00] died due to signal 11
[01:30:00] error: test failed, to rerun pass '--test coretests'
[01:30:00]
[01:30:00]
[01:30:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1a8a35c8:start=1522901767887091320,finish=1522901767907906714,duration=20815394
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:12f4eff8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:12f4eff8:start=1522901767914781100,finish=1522901767923181340,duration=8400240
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cd25d5a
$ dmesg | grep -i kill
[   11.394805] init: failsafe main process (1094) killed by TERM signal
