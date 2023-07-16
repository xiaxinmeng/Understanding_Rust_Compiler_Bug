plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:33:14] 189 |         emitter,
[00:33:14]     |         ^^^^^^^ expected trait `errors::emitter::Emitter + rustc_data_structures::sync::Send`, found trait `errors::emitter::Emitter`
[00:33:14]     |
[00:33:14]     = note: expected type `std::boxed::Box<errors::emitter::Emitter + rustc_data_structures::sync::Send + 'static>`
[00:33:14]                found type `std::boxed::Box<errors::emitter::Emitter>`
---
[00:33:20] Makefile:28: recipe for target 'all' failed
[00:33:20] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:090e2dc4:start=1523974141721054188,finish=1523974141735218647,duration=14164459
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0840ff99
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0840ff99:start=1523974141745405845,finish=1523974141755642511,duration=10236666
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ae47e9f
$ dmesg | grep -i kill
[   10.658621] init: failsafe main process (1096) killed by TERM signal
