plain
Resolving deltas: 100% (604637/604637), completed with 4783 local objects.
---
[00:03:30] configure: rust.quiet-tests     := True
---
[00:15:53] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1556: line longer than 100 chars
[00:15:53] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1583: line longer than 100 chars
[00:15:53] tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1611: line longer than 100 chars
---
116988 ./obj/build/bootstrap/debug/incremental/bootstrap-232achgbjnpqy/s-ezu81w52hb-ltna5o-23cdd79jjy90a
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:001cc8fa:start=1522946858677727392,finish=1522946858684283059,duration=6555667
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:36debfb4
$ dmesg | grep -i kill
[   10.355499] init: failsafe main process (1094) killed by TERM signal
