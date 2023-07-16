plain
[00:05:46]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:05:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:30]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:47]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:17] error[E0559]: variant `rustc_target::abi::Variants::Tagged` has no field named `tag`
[00:08:17]     --> librustc/ty/layout.rs:1060:25
[00:08:17]      |
[00:08:17] 1060 |                         tag: tag,
[00:08:17]      |                         ^^^ `rustc_target::abi::Variants::Tagged` does not have this field
[00:08:18] error[E0308]: mismatched types
[00:08:18]     --> librustc/ty/layout.rs:1061:25
[00:08:18]      |
[00:08:18] 1061 |                         variants
[00:08:18] 1061 |                         variants
[00:08:18]      |                         ^^^^^^^^ expected struct `rustc_target::abi::LayoutDetails`, found struct `std::vec::Vec`
[00:08:18]      |
[00:08:18]      = note: expected type `std::vec::Vec<rustc_target::abi::LayoutDetails>`
[00:08:18]                 found type `std::vec::Vec<std::vec::Vec<rustc_target::abi::TyLayout<'_, &ty::TyS<'_>>>>`
[00:08:18] 
[00:08:18] error[E0026]: variant `rustc_target::abi::Variants::Tagged` does not have a field named `tag`
[00:08:18]     --> librustc/ty/layout.rs:1221:40
[00:08:18]      |
[00:08:18] 1221 |                     Variants::Tagged { ref tag, .. } => Some(tag.value.size(self)),
[00:08:18]      |                                        ^^^^^^^ variant `rustc_target::abi::Variants::Tagged` does not have this field
[00:08:18] 
[00:08:18] error[E0026]: variant `rustc_target::abi::Variants::Tagged` does not have a field named `tag`
[00:08:18]     --> librustc/ty/layout.rs:1625:40
[00:08:18]      |
[00:08:18] 1625 |                     Variants::Tagged { ref tag, .. } |
[00:08:18]      |                                        ^^^^^^^ variant `rustc_target::abi::Variants::Tagged` does not have this field
[00:08:18] 
[00:08:18] error[E0026]: variant `rustc_target::abi::Variants::Tagged` does not have a field named `tag`
[00:08:18]     --> librustc/ty/layout.rs:1739:17
[00:08:18] 1739 |                 ref tag,
[00:08:18] 1739 |                 ref tag,
[00:08:18]      |                 ^^^^^^^ variant `rustc_target::abi::Variants::Tagged` does not have this field
[00:08:18] 
[00:08:18] error[E0027]: pattern does not mention field `discr`
[00:08:18]     --> librustc/ty/layout.rs:1738:13
[00:08:18] 1738 | /             Tagged {
[00:08:18] 1739 | |                 ref tag,
[00:08:18] 1740 | |                 ref variants,
[00:08:18] 1741 | |             } => {
[00:08:18] 1741 | |             } => {
[00:08:18]      | |_____________^ missing field `discr`
[00:08:18] 
obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-401bddd0d1809e53.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-c0908caf79d2e3f2.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-566a8d95e6a18781.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-19e2a5cf3e91d41b.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-21ce4bd19908f0cc.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-206e1aadbc3c6b8a.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-16776be762f4e8c2.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-31a3817325787acc/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-90c24e5cae9d047e/out` (exit code: 101)
[00:08:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:21] expected success, got: exit code: 101
[00:08:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:21] travis_fold:end:stage0-rustc

[00:08:21] travis_time:end:stage0-rustc:start=1525430685389975501,finish=1525430883739677117,duration=198349702310776 .
1423056 ./obj
---
149532 ./.git/modules/src
149112 ./src/llvm-emscripten/test
144684 ./obj/build/bootstrap/debug/incremental
123716 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123712 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0px3k31k2-1xgd6ur-1ujokh614sjd2
89684 ./src/llvm/test/CodeGen
83604 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71096 ./.git/modules/src/tools
70944 ./obj/build/x86_64-unknown-linux-gnu/native
