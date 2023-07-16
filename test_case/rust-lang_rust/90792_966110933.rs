plain
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
error: could not compile `autocfg`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name autocfg /cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-1.0.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=fa5fb2f81f71684d -C extra-filename=-fa5fb2f81f71684d --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:53
make: *** [prepare] Error 1
---
   Compiling getopts v0.2.21
error: could not compile `thread_local`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name thread_local /cargo/registry/src/github.com-1ecc6299db9ec823/thread_local-1.0.1/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=ae9887d7ee5a3068 -C extra-filename=-ae9887d7ee5a3068 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern lazy_static=/checkout/obj/build/bootstrap/debug/deps/liblazy_static-1f5d3923e02f6398.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:01
make: *** [prepare] Error 1
---
   Compiling syn v1.0.80
error: could not compile `quote`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name quote --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/quote-1.0.7/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' --cfg 'feature="proc-macro"' -C metadata=8a2da347e4bf40b5 -C extra-filename=-8a2da347e4bf40b5 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern proc_macro2=/checkout/obj/build/bootstrap/debug/deps/libproc_macro2-2398ddf6b8c4f5ba.rmeta --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:03
make: *** [prepare] Error 1
---
   Compiling opaque-debug v0.3.0
[RUSTC-TIMING] build_script_build test:false 0.393
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] either test:false 0.305
rustc exited with signal: 11 (core dumped)
error: could not compile `either`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name either /cargo/registry/src/github.com-1ecc6299db9ec823/either-1.6.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' --cfg 'feature="use_std"' -C metadata=22689a2d6fcd3999 -C extra-filename=-22689a2d6fcd3999 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=v0 -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] opaque_debug test:false 0.059
[RUSTC-TIMING] build_script_build test:false 0.472
[RUSTC-TIMING] build_script_build test:false 0.457
[RUSTC-TIMING] autocfg test:false 0.511
