plain
[01:27:17] thread '<unnamed>' panicked at 'isize::MAX + 1 should trigger an OOM!', libstd/collections/hash/map.rs:3640:22
---
[01:27:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--"
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:02ede5a2:start=1523473051582066794,finish=1523473051595728633,duration=13661839
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04ee99b9
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:04ee99b9:start=1523473051602017630,finish=1523473051608247498,duration=6229868
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1029a65e
$ dmesg | grep -i kill
[   10.878622] init: failsafe main process (1093) killed by TERM signal
