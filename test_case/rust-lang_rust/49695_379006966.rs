plain
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:02:06] configure: rust.quiet-tests     := True
---
[00:50:19] error[E0277]: the trait bound `syntax::ast::Symbol: std::ops::Deref` is not satisfied
[00:50:19]     --> librustdoc/clean/mod.rs:1797:24
[00:50:19]      |
[00:50:19] 1797 |             if tp.name == keywords::SelfType.name() {
[00:50:19]      |                        ^^ the trait `std::ops::Deref` is not implemented for `syntax::ast::Symbol`
[00:50:19]      |
[00:50:19]      = note: required because of the requirements on the impl of `std::cmp::PartialEq<syntax::ast::Symbol>` for `syntax::symbol::InternedString`
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1500bdcb:start=1522947430748714534,finish=1522947430758013029,duration=9298495
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:02032a11
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:02032a11:start=1522947430768928875,finish=1522947430777800838,duration=8871963
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034b1330
$ dmesg | grep -i kill
[   10.629479] init: failsafe main process (1092) killed by TERM signal
