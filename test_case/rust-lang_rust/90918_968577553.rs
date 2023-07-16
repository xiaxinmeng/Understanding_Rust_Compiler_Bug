plain
   Compiling merge v0.1.0
error: could not compile `serde_derive`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name serde_derive /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' -C metadata=dc5c18f1dfc7942d -C extra-filename=-dc5c18f1dfc7942d --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern proc_macro2=/checkout/obj/build/bootstrap/debug/deps/libproc_macro2-2398ddf6b8c4f5ba.rlib --extern quote=/checkout/obj/build/bootstrap/debug/deps/libquote-8a2da347e4bf40b5.rlib --extern syn=/checkout/obj/build/bootstrap/debug/deps/libsyn-26603e844be531e8.rlib --extern proc_macro --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings --cfg underscore_consts` (signal: 11, SIGSEGV: invalid memory reference)
Build completed unsuccessfully in 0:00:59
make: *** [prepare] Error 1
Makefile:60: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
---
[RUSTC-TIMING] remove_dir_all test:false 0.050
[RUSTC-TIMING] either test:false 0.167
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling opaque-debug v0.3.0
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-24f36aeef90a945b.so(+0x5141e3)[0x7f0dacd061e3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x11390)[0x7f0dac277390]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2dc44)[0x560416fd8c44]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f5ee)[0x560416fda5ee]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x324e5)[0x560416fdd4e5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f2d5)[0x560416fda2d5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16b0f)[0x560416fc1b0f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16d0a)[0x560416fc1d0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x11eac)[0x560416fbceac]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x4eb0a)[0x560416ff9b0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x5093c)[0x560416ffb93c]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x6439)[0x7f0dac26c439]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x7870)[0x7f0dac26d870]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x7f0dabb9751d]
   Compiling stable_deref_trait v1.2.0
[RUSTC-TIMING] stable_deref_trait test:false 0.048
   Compiling cpuid-bool v0.1.2
[RUSTC-TIMING] build_script_build test:false 0.474
---
   Compiling annotate-snippets v0.8.0
[RUSTC-TIMING] cpuid_bool test:false 0.067
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] build_script_build test:false 0.518
rustc exited with signal: 11 (core dumped)
error: could not compile `getrandom`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.0/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="std"' -C metadata=301bf632479934c1 -C extra-filename=-301bf632479934c1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/getrandom-301bf632479934c1 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] build_script_build test:false 0.546
[RUSTC-TIMING] arrayvec test:false 0.226
[RUSTC-TIMING] unicode_width test:false 0.101
[RUSTC-TIMING] scoped_tls test:false 0.108
