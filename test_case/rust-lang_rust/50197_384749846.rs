plain
[00:07:56]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:08:20]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:15:52]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:15:52]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:15:58] error[E0599]: no method named `substs` found for type `&rustc::ty::TraitRef<'_>` in the current scope
[00:15:58]     --> librustc_typeck/check/method/probe.rs:1490:57
[00:15:58]      |
[00:15:58] 1490 |                             && !trait_ref.skip_binder().substs().has_skol()
[00:15:58]      |                                                         ^^^^^^ field, not a method
[00:15:58]      |
[00:15:58]      = help: did you mean to write `trait_ref.skip_binder().substs` instead of `trait_ref.skip_binder().substs(...)`?
[00:16:01] error: aborting due to previous error
[00:16:01] 
[00:16:01] For more information about this error, try `rustc --explain E0599`.
[00:16:01] error: Could not compile `rustc_typeck`.
[00:16:01] error: Could not compile `rustc_typeck`.
[00:16:01] 
[00:16:01] Caused by:
[00:16:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=22421d2ff61f4da9 -C extra-filename=-22421d2ff61f4da9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-6f259a2a9f59267f.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-959ba3a7ab7a2be3.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-550afbc40330c44b.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-648afcf1a0c2954c.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-cb643e904ecf9227.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-7f999938245933d1.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-21033e7d98b3b734.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-0b7f7b8330cb09ce.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-de29d22526477961.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
/llvm-emscripten/test
149024 ./.git/modules
149020 ./.git/modules/src
143664 ./obj/build/bootstrap/debug/incremental
143664 ./obj/build/bootstrap/debug/incremental
126608 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
122708 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka
122704 ./obj/build/bootstrap/debug/incremental/bootstrap-1sil8jgb030ka/s-f0hgex28vh-1b56voz-2f3cmz1knheyt
89696 ./src/llvm/test/CodeGen
82464 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
82460 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
70944 ./obj/build/x86_64-unknown-linux-gnu/native
---
11900 ./src/tools/lld
11284 ./.git/objects
10708 ./.git/objects/pack
10076 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
9304 ./.git/modules/src/tools/clippy/objects
travis_time:end:2177543a:start=1524768280137570666,finish=1524768280361682807,duration=224112141
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
