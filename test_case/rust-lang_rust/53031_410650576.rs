plain
[00:20:26]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:20:26]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:20:26]    Compiling cc v1.0.18
[00:20:27]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:20:34] error[E0425]: cannot find value `CrateTypeRlib` in module `config`
[00:20:34]     --> librustc_codegen_llvm/back/write.rs:2358:70
[00:20:34]      |
[00:20:34] 2358 |         tcx.sess.crate_types.borrow().iter().any(|ct| *ct == config::CrateTypeRlib) &&
[00:20:34]      |                                                                      ^^^^^^^^^^^^^ not found in `config`
[00:20:34] 
cremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-4f182245385237b5.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-5ed58e76c7dace45.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-1cc3798595d9708a.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-2e835ba951928190.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-1ed4d2103c0d7730.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-899ce576c6b4bcbf.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-45767bac74e332c7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/'s/Date: //g' || true)
Mon, 06 Aug 2018 09:42:35 GMT
travis_time:end:1e5ee39f:start=1533548555399164652,finish=1533548555458225254,duration=59060602

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
