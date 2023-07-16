plain
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:111f63b0:start=1523463323129364932,finish=1523463323139017390,duration=9652458
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:33269af8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:33269af8:start=1523463323147430575,finish=1523463323156014950,duration=8584375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3370a7f0
$ dmesg | grep -i kill
[   11.520591] init: failsafe main process (1094) killed by TERM signal
