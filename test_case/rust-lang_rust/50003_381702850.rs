plain
Resolving deltas: 100% (616283/616283), completed with 4912 local objects.
---
[00:00:43] configure: rust.quiet-tests     := True
---
[00:30:57] error: this feature has been stable since 1.26.0. Attribute no longer needed
[00:30:57]   --> librustc_typeck/lib.rs:85:12
---
[00:30:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=5a70b5edf99aa8eb -C extra-filename=-5a70b5edf99aa8eb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-a7e4568ae4f6d310.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-7c8f07fcfb43dd0e.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3d30704c9bf94823.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-c4c7b11d5784a197.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4acf4151e5e9c3f9.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-4751eaacfa9b82b1.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e7f38b02feb371ba.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-dfd6e9728ffc83dd.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-caf863901ba00d4d.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-94d776423bcbc65e.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-306b41e2ad232afa/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-21ebc33766ad71e1/out` (exit code: 101)
[00:30:57] warning: build failed, waiting for other jobs to finish...
[00:32:27] error: build failed
[00:32:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:32:27] expected success, got: exit code: 101
[00:32:27] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:32:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:32:27] travis_fold:end:stage1-rustc
[00:32:27] travis_time:end:stage1-rustc:start=1523902617869707036,finish=1523903159683132825,duration=541813425789
[00:32:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:32:27] Build completed unsuccessfully in 0:27:15
[00:32:27] Makefile:28: recipe for target 'all' failed
[00:32:27] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0a0115c4:start=1523903160262982064,finish=1523903160269929049,duration=6946985
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:13920ce4
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:13920ce4:start=1523903160276530852,finish=1523903160282932519,duration=6401667
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02944794
$ dmesg | grep -i kill
[   10.660438] init: failsafe main process (1094) killed by TERM signal
