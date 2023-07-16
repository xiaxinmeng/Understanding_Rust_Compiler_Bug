plain
Resolving deltas: 100% (612819/612819), completed with 4858 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:04:37] tidy error: /checkout/src/librustc_traits/lowering.rs:168: line longer than 100 chars
[00:04:38] tidy error: /checkout/src/librustc/traits/mod.rs:293: line longer than 100 chars
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:197a917b:start=1523260137824011439,finish=1523260137830230222,duration=6218783
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b2fe99f
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:cras
