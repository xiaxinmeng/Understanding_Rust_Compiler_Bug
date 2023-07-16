plain
[00:17:27]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:17:27]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:17:27]    Compiling cc v1.0.15
[00:17:28]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:17:35] error[E0425]: cannot find value `cx` in this scope
[00:17:35]   --> librustc_codegen_llvm/debuginfo/source_loc.rs:88:32
[00:17:35]    |
[00:17:35] 88 |             let col_used =  if cx.sess().target.target.options.is_like_msvc {
[00:17:35]    |                                ^^ did you mean `bx`?
[00:17:35] 
[00:17:37] error[E0308]: if and else have incompatible types
[00:17:37]   --> librustc_codegen_llvm/debuginfo/source_loc.rs:88:29
[00:17:37]    |
[00:17:37] 88 |               let col_used =  if cx.sess().target.target.options.is_like_msvc {
[00:17:37]    |  _____________________________^
[00:17:37] 89 | |                 UNKNOWN_COLUMN_NUMBER
[00:17:37] 90 | |             } else {
[00:17:37] 92 | |             };
[00:17:37]    | |_____________^ expected u32, found usize
[00:17:37]    |
[00:17:37]    = note: expected type `u32`
[00:17:37]    = note: expected type `u32`
[00:17:37]               found type `usize`
[00:17:37] 
eps/librustc_errors-e036f8f5b9204e52.so --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4c6197a9c6b06d9c.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-42e74c2f020e6a01.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-8fc603afb8ea9b13.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-06dc5f95410eaad9.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-066098b54e835a1f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-05774896829b5d87.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a1b02e0d020520ac.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-f36f6cb494d373be.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-a2c79048744e2ad8.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-ea5907e223517cf2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-a59a4023b81a89b2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-c4b130070c8b640c/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
68792 ./src/llvm/lib
65412 ./src/llvm-emscripten/test/CodeGen
61228 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
60840 ./src/llvm-emscripten/lib
