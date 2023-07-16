plain
Resolving deltas: 100% (611741/611741), completed with 4887 local objects.
---
[00:00:47] configure: rust.quiet-tests     := True
---
[00:05:32] tidy error: Found 1 features without a gate test.
[00:05:32] Expected a gate test for the feature 'irrefutable_let_pattern'.
[00:05:32] Hint: create a failing test file named 'feature-gate-irrefutable_let_pattern.rs'
[00:05:32]       in the 'ui' test suite, with its failures due to
[00:05:32]       missing usage of #![feature(irrefutable_let_pattern)].
[00:05:32] Hint: If you already have such a test and don't want to rename it,
[00:05:32]       you can also add a // gate-test-irrefutable_let_pattern line to the test file.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:004b7a0a:start=1523499212782001736,finish=1523499212790320834,duration=8319098
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:12aae698
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:12aae698:start=1523499212798265669,finish=1523499212806458768,duration=8193099
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:121a8c8c
$ dmesg | grep -i kill
[   11.682560] init: failsafe main process (1092) killed by TERM signal
