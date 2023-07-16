plain
   Compiling hashbrown v0.11.0
   Compiling miniz_oxide v0.4.0
[RUSTC-TIMING] rustc_std_workspace_alloc test:false 0.032
[RUSTC-TIMING] panic_abort test:false 0.215
rustc exited with signal: 11
error: could not compile `panic_abort`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name panic_abort --edition=2018 library/panic_abort/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C metadata=7a1100c4e225862e -C extra-filename=-7a1100c4e225862e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps --target arm-linux-androideabi -C linker=/android/ndk/arm-14/bin/arm-linux-androideabi-clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps/liballoc-845ee545bbe426da.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps/libcfg_if-9aa210456737e445.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps/libcompiler_builtins-e88007c334f43f3a.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps/libcore-474b270852438406.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/deps/liblibc-8d79781e4daf5d39.rmeta -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zsave-analysis -Cprefer-dynamic -Cembed-bitcode=yes -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/arm-linux-androideabi/release/build/compiler_builtins-4d7b48d5a2760790/out` (exit code: 254)
[RUSTC-TIMING] panic_unwind test:false 0.239
[RUSTC-TIMING] hashbrown test:false 0.762
[RUSTC-TIMING] alloc test:false 3.212
[RUSTC-TIMING] miniz_oxide test:false 2.523
[RUSTC-TIMING] miniz_oxide test:false 2.523
[RUSTC-TIMING] core test:false 16.513
[RUSTC-TIMING] gimli test:false 5.020
[RUSTC-TIMING] object test:false 9.645
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-linux-androideabi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host= --target arm-linux-androideabi,armv7-linux-androideabi,thumbv7neon-linux-androideabi,i686-linux-android,aarch64-linux-android,x86_64-linux-android
Build completed unsuccessfully in 0:13:33
