plain
Resolving deltas: 100% (613524/613524), completed with 4886 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:02:58] error: expected `,`, or `}`, found `>`
[00:02:58]     --> libcore/str/mod.rs:2608:45
[00:02:58]      |
[00:02:58] 2608 |     inner: SplitTerminator<'a, IsWhitespace>>,
[00:02:58]      |                                             ^
[00:02:58]      |
[00:02:58]      = help: struct fields should be separated by commas
[00:02:58]
[00:03:00]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:01] error: unused import: `Filter`
[00:03:01]   --> libcore/str/mod.rs:22:52
[00:03:01]    |
[00:03:01] 22 | use iter::{Map, Cloned, FusedIterator, TrustedLen, Filter};
---
[00:03:01]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:03:01]
[00:03:04]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:04]    Compiling cmake v0.1.29
[00:03:04]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:06] error[E0392]: parameter `'a` is never used
[00:03:06]     --> libcore/str/mod.rs:2607:28
[00:03:06]      |
[00:03:06] 2607 | pub struct SplitWhitespace<'a> {
[00:03:06]      |                            ^^ unused type parameter
[00:03:06]      |
[00:03:06]      = help: consider removing `'a` or using a marker such as `marker::PhantomData`
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1ccc4ac5:start=1523307667211641695,finish=1523307667217541214,duration=5899519
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0589dbd0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0589dbd0:start=1523307667222799634,finish=1523307667228358331,duration=5558697
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12d92bde
$ dmesg | grep -i kill
[   10.595163] init: failsafe main process (1094) killed by TERM signal
