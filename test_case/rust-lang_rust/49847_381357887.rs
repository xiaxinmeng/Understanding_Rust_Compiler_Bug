plain
[00:23:50] error: internal compiler error: librustc_save_analysis/lib.rs:117: no data for crate 7
---
[00:23:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=d9da2a2253530287 -C extra-filename=-d9da2a2253530287 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps --target arm-unknown-linux-gnueabihf -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/liblibc-9f0d372404db7b62.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libpanic_unwind-52658ef1dfd0bf87.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libpanic_abort-caf5925d568bdc6d.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/liballoc_jemalloc-43e4a879336eaba6.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libcore-88a4d043878d936a.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libunwind-edc4e9c4f1f2620d.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/liballoc-b9e1e80353ce065e.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libcompiler_builtins-fe0d984dfcb9746e.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/libstd_unicode-c4b88a51a5f32951.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/deps/liballoc_system-845a9d197171fd43.rlib -L native=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/arm-unknown-linux-gnueabihf/release/build/compiler_builtins-4bdac560af2d255d/out -L native=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/jemalloc/lib` (exit code: 101)
[00:23:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-unknown-linux-gnueabihf" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:51] expected success, got: exit code: 101
[00:23:51] travis_fold:end:stage1-std
[00:23:51] travis_time:end:stage1-std:start=1523737265129026717,finish=1523737319652099327,duration=54523072610
[00:23:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0d769038:start=1523737320225485012,finish=1523737320234810715,duration=9325703
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0df0e020
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0df0e020:start=1523737320239946248,finish=1523737320245556695,duration=5610447
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10a8135d
$ dmesg | grep -i kill
[   10.194420] init: failsafe main process (1093) killed by TERM signal
