plain
Resolving deltas: 100% (614347/614347), completed with 4877 local objects.
---
[00:00:11] error: Server does not allow request for unadvertised object 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e
[00:00:11] Fetched in submodule path 'src/tools/miri', but it did not contain 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e. Direct fetching of that commit failed.
---
[00:00:13] error: Server does not allow request for unadvertised object 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e
[00:00:13] Fetched in submodule path 'src/tools/miri', but it did not contain 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e. Direct fetching of that commit failed.
---
[00:00:15] error: Server does not allow request for unadvertised object 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e
[00:00:15] Fetched in submodule path 'src/tools/miri', but it did not contain 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e. Direct fetching of that commit failed.
---
[00:00:17] error: Server does not allow request for unadvertised object 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e
[00:00:17] Fetched in submodule path 'src/tools/miri', but it did not contain 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e. Direct fetching of that commit failed.
---
[00:00:18] error: Server does not allow request for unadvertised object 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e
[00:00:18] Fetched in submodule path 'src/tools/miri', but it did not contain 3f1b2bdd6895f9aedb22601bd28bc690d5a55a1e. Direct fetching of that commit failed.
[00:00:18] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:35898fee:start=1523630087062695198,finish=1523630087069131913,duration=6436715
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:3b914e40
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:3b914e40:start=1523630087075375511,finish=1523630087081524122,duration=6148611
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ad3767c
$ dmesg | grep -i kill
[   11.828926] init: failsafe main process (1096) kille
