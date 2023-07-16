plain
Resolving deltas: 100% (601193/601193), completed with 4719 local objects.
---
[00:00:42] configure: rust.quiet-tests     := True
---
[00:04:35] 144 |             let _ = resolve_symname(*frame, |symname| {
[00:04:35]     |                                     ^^^^^^ expected struct `sys_common::backtrace::Frame`, found tuple
[00:04:35]     |
[00:04:35]     = note: expected type `sys_common::backtrace::Frame`
[00:04:35]                found type `(usize, &sys_common::backtrace::Frame)`
[00:04:35]
[00:04:35] error[E0599]: no method named `peek` found for type `core::iter::TakeWhile<core::iter::Peekable<core::iter::Enumerate<core::slice::Iter<'_, sys_common::backtrace::Frame>>>, [closure@libstd/sys_common/backtrace.rs:142:21: 154:10 context:_]>` in the current scope
[00:04:35]    --> libstd/sys_common/backtrace.rs:162:14
---
[00:04:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name std libstd/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -C metadata=e23f5ab1b35c3cbb -C extra-filename=-e23f5ab1b35c3cbb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libunwind-ea5547b02114e2c4.rlib --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-709ac4dccd81a82f.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-1b8789e893adb899.rlib --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-6f7b4911da25e333.rlib --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-059700c621bf940b.rlib --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-efb916052e2ce91d.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-a2d7f090df844ebf.rlib --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-9b401d4f1df5ac3a.rlib --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-71f5363a0f32984a.rlib --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-1f93143c583ac549.rlib --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc-6efa3f327e315680.rlib --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-209d69cbee1ef856.rlib --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-6843ee90cbd31c10.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/liblibc-e53cce9e670d242c.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/libbacktrace/.libs -l static=backtrace -l dl -l rt -l pthread -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-6789d0ad544f7553/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:04:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:35] expected success, got: exit code: 101
[00:04:35] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
[00:04:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:04:35] travis_fold:end:stage0-std
[00:04:35] travis_time:end:stage0-std:start=1523179519619591257,finish=1523179575945681380,duration=56326090123
[00:04:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:35] Build completed unsuccessfully in 0:00:57
[00:04:35] make: *** [tidy] Error 1
[00:04:35] Makefile:79: recipe for target 'tidy' failed
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:059091d8:start=1523179576443242280,finish=1523179576449101644,duration=5859364
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:000832c5
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:000832c5:start=1523179576454164755,finish=1523179576459553499,duration=5388744
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0251be3f
$ dmesg | grep -i kill
[   10.469385] init: failsafe main process (1093) killed by TERM signal
