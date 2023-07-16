plain
[00:05:36]    Compiling backtrace v0.3.6
[00:05:43]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:14]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:30]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:35] error: unused import: `middle::const_val::ConstVal`
[00:07:35]   --> librustc/ty/util.rs:18:5
[00:07:35]    |
[00:07:35] 18 | use middle::const_val::ConstVal;
[00:07:35]    |
[00:07:35]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:35] 
[00:07:35] 
[00:07:35] error: unused import: `ty::fold::TypeVisitor`
[00:07:35]   --> librustc/ty/util.rs:21:5
[00:07:35]    |
[00:07:35] 21 | use ty::fold::TypeVisitor;
[00:07:35] 
[00:07:35] 
[00:07:35] error: unused imports: `PrimVal`, `Value`
[00:07:35]   --> librustc/ty/util.rs:28:22
[00:07:35]    |
[00:07:35] 28 | use mir::interpret::{Value, PrimVal};
[00:07:35]    |                      ^^^^^  ^^^^^^^
[00:07:35] error: unused import: `std::hash::Hash`
[00:07:35]   --> librustc/ty/util.rs:34:5
[00:07:35]    |
[00:07:35] 34 | use std::hash::Hash;
---
[00:07:35]    |
[00:07:35] 35 | use std::intrinsics;
[00:07:35]    |     ^^^^^^^^^^^^^^^
[00:07:35] 
8781.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-33787dcdac3a4dd2.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-21ce4bd19908f0cc.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b9901acb1e9e6766.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-b7c66a9cab3ff5a6.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-16776be762f4e8c2.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-dd375c0772cf1a0d.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4f0866e958f59455.rlib --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-19e2a5cf3e91d41b.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-c0908caf79d2e3f2.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-b789a86e1ab64d11.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f456f53371aa074c.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-206e1aadbc3c6b8a.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-ee923a086d887011.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-8d928be2ff984c7f.so --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-fbe97ade1df6b749.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-31a3817325787acc/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-90c24e5cae9d047e/out` (exit code: 101)
[00:08:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:01] expected success, got: exit code: 101
[00:08:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:01] travis_fold:end:stage0-rustc

[00:08:01] travis_time:end:stage0-rustc:start=1525787127228306089,finish=1525787311442790903,duration=184214484814


[00:08:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:01] Build completed unsuccessfully in 0:03:17
[00:08:01] Makefile:28: recipe for target 'all' failed
[00:08:01] make: *** [all] Error 1
314860 ./src/llvm
256636 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
241192 ./src/llvm-emscripten
210064 ./src/llvm/test
---
149688 ./.git/modules/src
149128 ./src/llvm-emscripten/test
144928 ./obj/build/bootstrap/debug/incremental
123944 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123940 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0ugujw9bx-1vphh6e-3a8ftmql7i7fg
89700 ./src/llvm/test/CodeGen
83600 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
71424 ./.git/modules/src/tools
70944 ./obj/build/x86_64-unknown-linux-gnu/native
---
11788 ./src/doc
10996 ./.git/objects
10696 ./.git/objects/pack
10080 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
travis_time:end:3cb567a0:start=1525787311731590867,finish=1525787311897770281,duration=166179414
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:22f4da0b
