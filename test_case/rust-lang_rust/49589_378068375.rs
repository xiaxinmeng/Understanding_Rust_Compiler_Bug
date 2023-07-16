plain
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:158f2496:start=1522708520163151910,finish=1522708520170017480,duration=6865570
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c52c69c
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0c52c69c:start=1522708520176119604,finish=1522708520182731592,duration=6611988
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000c8ce4
$ dmesg | grep -i kill
[   11.082471] init: failsafe main process (1095) killed by TERM signal
