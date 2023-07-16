plain
[00:21:18]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:21:18]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:21:18]    Compiling cc v1.0.15
[00:21:19]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:21:28] error[E0283]: type annotations required: cannot resolve `str: std::convert::AsRef<_>`
[00:21:28]    --> librustc_codegen_llvm/back/linker.rs:164:21
[00:21:28]     |
[00:21:28] 164 |         os.push(lib.as_ref());
[00:21:28] 
[00:21:28] 
 --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-ac6d20f88773f905.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-29f7cb771c179438.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-e8a1df2f3ad631bc.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-584193bd2f2f0208.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-4074300b8e2af006.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-03cd04183e7fa779.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-54cee618814524ce.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-ca7321e9b3ae1aa8.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-62d80197b9ec531e/out -Ltravis_time:start:0f5bcf98
[   11.022072] init: failsafe main process (1093) killed by TERM signal
travis_time:end:0f5bcf98:start=1527646904943453220,finish=1527646904960437869,duration=16984649
travis_fold:end:after_failure.4

