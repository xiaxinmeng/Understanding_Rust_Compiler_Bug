plain
Resolving deltas: 100% (601375/601375), completed with 4722 local objects.
---
[00:00:45] configure: rust.quiet-tests     := True
---
[00:03:29] warning: this feature has been stable since 1.26.0. Attribute no longer needed
[00:03:29]    --> libstd/lib.rs:254:12
[00:03:29]     |
[00:03:29] 254 | #![feature(conservative_impl_trait)]
[00:03:29]     |            ^^^^^^^^^^^^^^^^^^^^^^^
[00:03:29]     |
[00:03:29]     = note: #[warn(stable_features)] on by default
---
[00:20:44] error: this feature has been stable since 1.26.0. Attribute no longer needed
[00:20:44]    --> libstd/lib.rs:254:12
---
[00:20:45]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=59f1365839a15336 -C extra-filename=-59f1365839a15336 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-f0c30809ac048014.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-ae9922ea98a7d2f5.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-9ca7a6a39fc271b4.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-0d5ca310002751eb.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-a7af859ebb7ffd34.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-22a7664bd150c80d.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-2c193ad75e997618.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d4e2de02cb1be3d7.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-2f9a4f2e814ed058.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-a1af8df7cb7f6288.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-cd51e4d8e5ddddce.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-ee8ce53bd96ab14e.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-64878136ec7adadb.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-dc103db2cc707fb1.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-c65e2a40a1e8fc25/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:20:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:45] expected success, got: exit code: 101
[00:20:45] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:20:45] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:20:45] travis_fold:end:stage1-std
[00:20:45] travis_time:end:stage1-std:start=1523903801128014818,finish=1523903870542734966,duration=69414720148
[00:20:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:45] Build completed unsuccessfully in 0:16:12
[00:20:45] Makefile:28: recipe for target 'all' failed
[00:20:45] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1486dd5e:start=1523903871083215396,finish=1523903871089983307,duration=6767911
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:29d094db
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:29d094db:start=1523903871096599730,finish=1523903871103229337,duration=6629607
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:044d04ff
$ dmesg | grep -i kill
[   11.022291] init: failsafe main process (1096) killed by TERM signal
