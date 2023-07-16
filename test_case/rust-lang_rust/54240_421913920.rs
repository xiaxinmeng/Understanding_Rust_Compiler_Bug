plain
[00:21:23]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:23]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:24]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:24]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:49] error[E0711]: feature `nonzero` is declared stable since 1.31.0, but was previously declared stable since 1.28.0
[00:21:49]     |
[00:21:49]     |
[00:21:49] 96  |               #[stable(feature = "nonzero", since = "1.31.0")]
[00:21:49] ...
[00:21:49] ...
[00:21:49] 110 | / nonzero_integers! {
[00:21:49] 111 | |     NonZeroU8(u8);
[00:21:49] 112 | |     NonZeroU16(u16);
[00:21:49] 113 | |     NonZeroU32(u32);
[00:21:49] 116 | |     NonZeroUsize(usize);
[00:21:49] 117 | | }
[00:21:49]     | |_- in this macro invocation
[00:21:49] 
---
[00:21:50] 
[00:21:50] To learn more, run the command again with --verbose.
[00:21:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:50] expected success, got: exit code: 101
[00:21:50] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:21:50] travis_fold:end:stage1-std

[00:21:50] travis_time:end:stage1-std:start=1537169272594519935,finish=1537169311025292907,duration=38430772972


[00:21:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:50] Build completed unsuccessfully in 0:17:02
[00:21:50] Makefile:28: recipe for target 'all' failed
[00:21:50] make: *** [all] Error 1
91228 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
74140 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
74136 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
74132 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
---
36908 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36336 ./.git/modules/src/libcompiler_builtins
35640 ./src/tools/clang/lib
35532 ./.git/modules/src/libcompiler_builtins/modules
35036 ./.git/modules/src/libcomp0K$ ls -lat $HOME/Library/Logs/DiagnosticReports/
travis_time:end:16ee5478:start=1537169311734268377,finish=1537169311741138714,duration=6870337
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2857c5c9
travis_time:start:2857c5c9
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2857c5c9:start=1537169311746753726,finish=1537169311752908473,duration=6154747
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ebe7cc9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:148ee130
travis_time:start:148ee130
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/buil
