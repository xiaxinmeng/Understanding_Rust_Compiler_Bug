plain
[01:33:45] died due to signal 11
[01:33:45] error: test failed, to rerun pass '--test coretests'
[01:33:45]
[01:33:45]
[01:33:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:018c9a0b:start=1523093342887998261,finish=1523093342904923268,duration=16925007
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:3045d9a8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:3045d9a8:start=1523093342910523349,finish=1523093342920816512,duration=10293163
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:035f6e3c
$ dmesg | grep -i kill
[   10.415132] init: failsafe main process (1094) killed by TERM signal
