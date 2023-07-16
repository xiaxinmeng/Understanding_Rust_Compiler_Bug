plain
Resolving deltas: 100% (611428/611428), completed with 4859 local objects.
---
[00:00:48] configure: rust.quiet-tests     := True
---
[00:16:29] error: unused import: `syntax::parse::token`
[00:16:29]   --> librustc_passes/ast_validation.rs:24:5
---
[00:16:29]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
---
[00:16:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_passes librustc_passes/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d6aeea102ad4d097 -C extra-filename=-d6aeea102ad4d097 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-069d1433ad059ff1.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-b061046bb39a474d.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1b4c3b4ba4ec5832.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-2cbdd37d611532dc.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d823127e323fa243.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-7eebd272275b441b.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1e725c22de2888b8.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-67023beb7edf290b.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-751752dec0960570/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-07c608814ba8d6ba/out` (exit code: 101)
[00:16:30] warning: build failed, waiting for other jobs to finish...
[00:16:51] error: build failed
[00:16:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
[00:16:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:16:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:51] expected success, got: exit code: 101
[00:16:51] travis_fold:end:stage0-rustc
[00:16:51] travis_time:end:stage0-rustc:start=1522860389190946911,finish=1522861126535917553,duration=737344970642
[00:16:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:51] Build completed unsuccessfully in 0:12:30
[00:16:51] Makefile:28: recipe for target 'all' failed
[00:16:51] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:065d2f3e:start=1522861127184583367,finish=1522861127192401713,duration=7818346
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:00093b00
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:00093b00:start=1522861127200332776,finish=1522861127209253374,duration=8920598
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17f6e5e3
$ dmesg | grep -i kill
[   11.461824] init: failsafe main process (1097) killed by TERM signal
