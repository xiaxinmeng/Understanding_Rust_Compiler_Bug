plain
   Compiling thread_local v1.0.1
error: could not compile `regex-automata`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name regex_automata /cargo/registry/src/github.com-1ecc6299db9ec823/regex-automata-0.1.10/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=8e700951c9869a66 -C extra-filename=-8e700951c9869a66 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:54
make: *** [prepare] Error 1
---
   Compiling ignore v0.4.17
error: could not compile `ignore`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name ignore /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.17/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=6eab7523daa887b2 -C extra-filename=-6eab7523daa887b2 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern crossbeam_utils=/checkout/obj/build/bootstrap/debug/deps/libcrossbeam_utils-217392e621d9698b.rmeta --extern globset=/checkout/obj/build/bootstrap/debug/deps/libglobset-0925db85937ab6c7.rmeta --extern lazy_static=/checkout/obj/build/bootstrap/debug/deps/liblazy_static-1f5d3923e02f6398.rmeta --extern log=/checkout/obj/build/bootstrap/debug/deps/liblog-ed7a0d1137dcadc8.rmeta --extern memchr=/checkout/obj/build/bootstrap/debug/deps/libmemchr-9d65e5e3abdde0ba.rmeta --extern regex=/checkout/obj/build/bootstrap/debug/deps/libregex-b0c184cb911fb168.rmeta --extern same_file=/checkout/obj/build/bootstrap/debug/deps/libsame_file-9a5e3ddd89cfe599.rmeta --extern thread_local=/checkout/obj/build/bootstrap/debug/deps/libthread_local-ae9887d7ee5a3068.rmeta --extern walkdir=/checkout/obj/build/bootstrap/debug/deps/libwalkdir-d47f0bb27827f0f7.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:04
make: *** [prepare] Error 1
---
   Compiling remove_dir_all v0.5.3
[RUSTC-TIMING] smallvec test:false 0.288
   Compiling arrayvec v0.7.0
[RUSTC-TIMING] build_script_build test:false 0.248
rustc exited with signal: 11 (core dumped)
error: could not compile `bitflags`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/bitflags-1.2.1/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="default"' -C metadata=5b2516889b0556c8 -C extra-filename=-5b2516889b0556c8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/bitflags-5b2516889b0556c8 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] remove_dir_all test:false 0.045
[RUSTC-TIMING] build_script_build test:false 0.346
[RUSTC-TIMING] build_script_build test:false 0.379
[RUSTC-TIMING] version_check test:false 0.360
