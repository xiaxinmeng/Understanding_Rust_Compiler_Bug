plain
Resolving deltas: 100% (613112/613112), completed with 4857 local objects.
---
[00:00:41] configure: rust.quiet-tests     := True
---
[00:18:45] make: *** [all] Error 1
[00:18:45] Makefile:28: recipe for target 'all' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:3901454a:start=1522409549192346845,finish=1522409549199528480,duration=7181635
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:105d11c0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:105d11c0:start=1522409549206175186,finish=1522409549213163157,duration=6987971
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0511bc9a
$ dmesg | grep -i kill
[   10.870252] init: failsafe main process (1092) killed by TERM signal
