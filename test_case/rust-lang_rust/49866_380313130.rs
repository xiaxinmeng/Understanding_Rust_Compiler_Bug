plain
[00:00:07] error: Server does not allow request for unadvertised object e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f
[00:00:07] Fetched in submodule path 'src/libcompiler_builtins', but it did not contain e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f. Direct fetching of that commit failed.
---
[00:00:08] error: Server does not allow request for unadvertised object e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f
[00:00:08] Fetched in submodule path 'src/libcompiler_builtins', but it did not contain e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f. Direct fetching of that commit failed.
---
[00:00:09] error: Server does not allow request for unadvertised object e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f
[00:00:09] Fetched in submodule path 'src/libcompiler_builtins', but it did not contain e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f. Direct fetching of that commit failed.
---
[00:00:10] error: Server does not allow request for unadvertised object e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f
[00:00:10] Fetched in submodule path 'src/libcompiler_builtins', but it did not contain e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f. Direct fetching of that commit failed.
---
[00:00:11] error: Server does not allow request for unadvertised object e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f
[00:00:11] Fetched in submodule path 'src/libcompiler_builtins', but it did not contain e0bb71ae93695d01cbeaf5dd7a43f0a7b40fb32f. Direct fetching of that commit failed.
[00:00:11] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0fce8196:start=1523416322650234846,finish=1523416322656769057,duration=6534211
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2b17b0e2
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2b17b0e2:start=1523416322663415776,finish=1523416322669924092,duration=6508316
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:059b9345
$ dmesg | grep -i kill
[   10.607272] init: failsafe main process (1093) killed by TERM signal
