plain
[01:31:25] died due to signal 11
[01:31:25] error: test failed, to rerun pass '--test coretests'
[01:31:25]
[01:31:25]
[01:31:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:305cebc8:start=1523051333028400675,finish=1523051333058232563,duration=29831888
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03ca3308
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:03ca3308:start=1523051333064462498,finish=1523051333086751318,duration=22288820
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0098e425
$ dmesg | grep -i kill
[   10.707881] init: failsafe main process (1097) killed by TERM signal
