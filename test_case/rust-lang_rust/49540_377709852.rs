plain
Resolving deltas: 100% (613923/613923), completed with 4858 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:35:49] .........................................................................i..........................
[00:35:55] ................iF..................................................................................
---
[00:36:29] ............................................................................................i.......
[00:36:35] ................................................................i...................................
---
[00:36:56] Makefile:58: recipe for target 'check' failed
[00:36:56] make: *** [check] Error 1
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:090e6c4d:start=1522517618136701569,finish=1522517618143477157,duration=6775588
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0112bb8b
$ dmesg | grep -i kill
[   10.604492] init: failsafe main process (1095) killed by TERM signal
