plain
[00:00:54] configure: rust.quiet-tests     := True
---
[00:43:35] error[E0277]: the trait bound `syntax::ast::Symbol: std::ops::Deref` is not satisfied
[00:43:35]     --> librustdoc/clean/mod.rs:1797:24
[00:43:35]      |
[00:43:35] 1797 |             if tp.name == keywords::SelfType.name() {
[00:43:35]      |                        ^^ the trait `std::ops::Deref` is not implemented for `syntax::ast::Symbol`
[00:43:35]      |
[00:43:35]      = note: required because of the requirements on the impl of `std::cmp::PartialEq<syntax::ast::Symbol>` for `syntax::symbol::InternedString`
---
[00:43:50] Makefile:28: recipe for target 'all' failed
[00:43:50] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:13759634:start=1522946959557974590,finish=1522946959565989017,duration=8014427
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:13da83ca
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:13da83ca:start=1522946959573675738,finish=1522946959582376915,duration=8701177
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30ddbad8
$ dmesg | grep -i kill
[   11.080535] init: failsafe main process (1091) killed by TERM signal
