plain
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00c61524:start=1523462589305727837,finish=1523462589311236929,duration=5509092
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:013423bb
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:013423bb:start=1523462589316487295,finish=1523462589321653722,duration=5166427
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:062f7100
$ dmesg | grep -i kill
[   11.259908] init: failsafe main process (1094) killed by TERM signal
