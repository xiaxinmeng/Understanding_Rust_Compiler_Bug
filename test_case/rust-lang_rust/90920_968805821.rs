plain
   Compiling ignore v0.4.17
error: could not compile `syn`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name syn --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.80/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="clone-impls"' --cfg 'feature="default"' --cfg 'feature="derive"' --cfg 'feature="full"' --cfg 'feature="parsing"' --cfg 'feature="printing"' --cfg 'feature="proc-macro"' --cfg 'feature="quote"' -C metadata=26603e844be531e8 -C extra-filename=-26603e844be531e8 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern proc_macro2=/checkout/obj/build/bootstrap/debug/deps/libproc_macro2-2398ddf6b8c4f5ba.rmeta --extern quote=/checkout/obj/build/bootstrap/debug/deps/libquote-8a2da347e4bf40b5.rmeta --extern unicode_xid=/checkout/obj/build/bootstrap/debug/deps/libunicode_xid-71b37fbec4a03338.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings --cfg syn_disable_nightly_tests` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:57
make: *** [prepare] Error 1
---
   Compiling crossbeam-epoch v0.8.2
[RUSTC-TIMING] cc test:false 1.628
   Compiling num-traits v0.2.12
[RUSTC-TIMING] build_script_build test:false 0.189
rustc exited with signal: 11 (core dumped)
error: could not compile `memoffset`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/memoffset-0.5.5/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="default"' -C metadata=20f7ea1250f8a47e -C extra-filename=-20f7ea1250f8a47e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/memoffset-20f7ea1250f8a47e -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern autocfg=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/libautocfg-7f11345a43e868c8.rlib --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] punycode test:false 0.742
[RUSTC-TIMING] unicode_script test:false 0.858
[RUSTC-TIMING] build_script_build test:false 0.305
[RUSTC-TIMING] build_script_build test:false 0.301
---
[RUSTC-TIMING] rustc_demangle test:false 1.427
[RUSTC-TIMING] itertools test:false 1.071
[RUSTC-TIMING] getopts test:false 1.076
[RUSTC-TIMING] sharded_slab test:false 1.201
rustc exited with signal: 11 (core dumped)
[RUSTC-TIMING] regex_syntax test:false 4.440
error: build failed
Build completed unsuccessfully in 0:02:56
