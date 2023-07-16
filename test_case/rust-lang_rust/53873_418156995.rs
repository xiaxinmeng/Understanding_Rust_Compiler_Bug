plain
[00:13:25]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:25]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:25]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:26]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:13:34] error[E0599]: no variant named `UserAssertTy` found for type `rustc::mir::StatementKind<'_>` in the current scope
[00:13:34]    --> librustc_mir/transform/qualify_min_const_fn.rs:233:11
[00:13:34]     |
[00:13:34] 233 |         | StatementKind::UserAssertTy(..)
[00:13:34] 
[00:13:37] error: aborting due to previous error
[00:13:37] 
[00:13:37] For more information about this error, try `rustc --explain E0599`.
[00:13:37] For more information about this error, try `rustc --explain E0599`.
[00:13:37] error: Could not compile `rustc_mir`.
[00:13:37] 
[00:13:37] Caused by:
[00:13:37]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=7952c237ec0d4952 -C extra-filename=-7952c237ec0d4952 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4ee92b74dcb65ca6.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-cfbc17aa3c766576.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4a211d9e23f5aeb5.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-61336079186baa43.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f882aab6100635ab.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-51fd1bd0441a9815.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-26b6009735d1b07c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ef45b71e578357b1.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
3652692 .
1401844 ./obj
1401808 ./obj/build
1185132 ./.git
---
151200 ./src/tools/clang
149128 ./src/llvm-emscripten/test
148964 ./obj/build/bootstrap/debug/incremental
134532 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134528 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4goosyi8j-ffgrad-29k09wkatlb53
111072 ./src/llvm/test/CodeGen
103868 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
