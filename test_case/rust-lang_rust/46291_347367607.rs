
[0Ktravis_fold:end:after_failure.1
[0Ktravis_fold:start:after_failure.2
[0Ktravis_time:start:28f0a230
[0K$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })

travis_time:end:28f0a230:start=1511825893488996918,finish=1511825893494620866,duration=5623948
[0Ktravis_fold:end:after_failure.2
[0Ktravis_fold:start:after_failure.3
[0Ktravis_time:start:093c2a7c
[0K$ cat /tmp/sccache.log
cat: /tmp/sccache.log: No such file or directory

travis_time:end:093c2a7c:start=1511825893499958946,finish=1511825893504677327,duration=4718381
[0Ktravis_fold:end:after_failure.3
[0Ktravis_fold:start:after_failure.4
[0Ktravis_time:start:024aa040
[0K$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory

travis_time:end:024aa040:start=1511825893510452059,finish=1511825893516475163,duration=6023104
[0Ktravis_fold:end:after_failure.4
[0Ktravis_fold:start:after_failure.5
[0Ktravis_time:start:04998e31
[0K$ find $HOME/Library/Logs/DiagnosticReports -type f -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \;
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory

travis_time:end:04998e31:start=1511825893522040560,finish=1511825893528102089,duration=6061529
[0Ktravis_fold:end:after_failure.5
[0Ktravis_fold:start:after_failure.6
[0Ktravis_time:start:1095a09c
[0K$ dmesg | grep -i kill
[   17.681956] init: failsafe main process (1133) killed by TERM signal

travis_time:end:1095a09c:start=1511825893532877544,finish=1511825893540881585,duration=8004041
[0Ktravis_fold:end:after_failure.6
[0K
