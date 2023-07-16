plain
[00:14:48]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:14:48]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:14:48]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:14:49]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:14:50] error[E0532]: expected tuple struct/variant, found unit variant `CastTy::Float`
[00:14:50]    --> librustc_mir/transform/qualify_min_const_fn.rs:170:35
[00:14:50]     |
[00:14:50] 170 |                 (CastTy::RPtr(_), CastTy::Float(_)) => bug!(),
[00:14:50] help: possible better candidates are found in other modules, you can import them into scope
[00:14:50]     |
[00:14:50] 1   | use rustc::hir::Float;
[00:14:50]     |
[00:14:50]     |
[00:14:50] 1   | use rustc::hir::PrimTy::Float;
[00:14:50] 1   | use rustc::ty::Float;
[00:14:50]     |
[00:14:50] 1   | use rustc::ty::TyKind::Float;
[00:14:50]     |
[00:14:50]     |
[00:14:50] and 6 other candidates
[00:14:50] 
[00:14:58] error[E0599]: no variant named `Rptr` found for type `rustc::ty::cast::CastTy<'_>` in the current scope
[00:14:58]    --> librustc_mir/transform/qualify_min_const_fn.rs:172:34
[00:14:58]     |
[00:14:58] 172 |                 (CastTy::Ptr(_), CastTy::Rptr(_)) => bug!(),
[00:14:58]     |                                  ^^^^^^^^^^^^^^^ variant not found in `rustc::ty::cast::CastTy<'_>`
[00:14:58]     |
[00:14:58]     = note: did you mean `rustc::ty::cast::CastTy<'_>::RPtr`?
[00:15:03] error: aborting due to 2 previous errors
[00:15:03] 
[00:15:03] Some errors occurred: E0532, E0599.
[00:15:03] For more information about an error, try `rustc --explain E0532`.
[00:15:03] For more information about an error, try `rustc --explain E0532`.
[00:15:03] error: Could not compile `rustc_mir`.
[00:15:03] 
[00:15:03] Caused by:
[00:15:03]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_mir librustc_mir/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=7952c237ec0d4952 -C extra-filename=-7952c237ec0d4952 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4ee92b74dcb65ca6.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5a39798fe03e47f4.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-0a515e87c8afea9e.rlib --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-87ec950697a15ed0.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-218f3033f29f5493.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-cfbc17aa3c766576.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4a211d9e23f5aeb5.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-61336079186baa43.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f882aab6100635ab.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-51fd1bd0441a9815.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cb741677cd0e0351.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-26b6009735d1b07c.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ef45b71e578357b1.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:16:22] error: build failed
[00:16:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:22] expected success, got: exit code: 101
[00:16:22] expected success, got: exit code: 101
[00:16:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:16:22] travis_fold:end:stage0-rustc

[00:16:22] travis_time:end:stage0-rustc:start=1535457890369294562,finish=1535458501240623262,duration=610871328700


[00:16:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:22] Build completed unsuccessfully in 0:11:10
[00:16:22] Makefile:28: recipe for target 'all' failed
[00:16:22] make: *** [all] Error 1
63796 ./src/test
60840 ./src/llvm-emscripten/lib
56324 ./src/llvm/test/MC
55548 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
