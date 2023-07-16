plain
Resolving deltas: 100% (613567/613567), completed with 4873 local objects.
---
[00:00:49] configure: rust.quiet-tests     := True
---
[00:21:53] error: internal compiler error: librustc/infer/region_constraints/mod.rs:695: cannot relate bound region: ReLateBound(DebruijnIndex { depth: 0 }, BrAnon(0)) <= ReStatic
[00:21:53]   --> libcore/panicking.rs:43:1
[00:21:53]    |
[00:21:53] 43 | / pub fn panic(expr_file_line_col: &(&'static str, &'static str, u32, u32)) -> ! {
[00:21:53] 44 | |     // Use Arguments::new_v1 instead of format_args!("{}", expr) to potentially
[00:21:53] 45 | |     // reduce size overhead. The format_args! macro uses str's Display trait to
[00:21:53] 46 | |     // write expr, which calls Formatter::pad, which must accommodate string
[00:21:53] ...  |
[00:21:53] 51 | |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]), &(file, line, col))
---
[00:21:53]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:53] expected success, got: exit code: 101
[00:21:53] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:21:53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:21:53] travis_fold:end:stage1-std
[00:21:53] travis_time:end:stage1-std:start=1523376692769816631,finish=1523376709307185145,duration=16537368514
[00:21:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:53] Build completed unsuccessfully in 0:16:29
[00:21:53] Makefile:28: recipe for target 'all' failed
[00:21:53] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:01c35728:start=1523376709848438196,finish=1523376709854123791,duration=5685595
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:03fab481
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:03fab481:start=1523376709859608763,finish=1523376709865107600,duration=5498837
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2624761f
$ dmesg | grep -i kill
[   10.383662] init: failsafe main process (1097) killed by TERM signal
