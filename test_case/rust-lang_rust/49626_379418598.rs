plain
Resolving deltas: 100% (611592/611592), completed with 4857 local objects.
---
[00:00:55] configure: rust.quiet-tests     := True
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:046c5333:start=1523061092073389157,finish=1523061092080636946,duration=7247789
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22b74af9
$ dmesg | grep -i kill
[   10.964306] init: failsafe main process (1092) killed by TERM signal
