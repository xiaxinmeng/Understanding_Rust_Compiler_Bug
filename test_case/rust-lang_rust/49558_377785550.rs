plain
Receiving objects: 100% (226/226), 21.90 KiB | 21.90 MiB/s, done.
---
Resolving deltas: 100% (195/195), completed with 52 local objects.
---
[00:00:49] configure: rust.quiet-tests     := True
---
113524 ./obj/build/bootstrap/debug/incremental/bootstrap-qe1dy92fk757/s-ezpmt7a8rw-1a89vxu-2mcuujfx21hz4
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:083c2119:start=1522587957061991907,finish=1522587957068012656,duration=6020749
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1130154f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1130154f:start=1522587957073424224,finish=1522587957079213184,duration=5788960
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2f2caba8
$ dmesg | grep -i kill
[   10.100584] init: failsafe main process (1093) killed by TERM signal
