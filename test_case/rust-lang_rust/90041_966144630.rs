plain
   Compiling cc v1.0.69
error: could not compile `log`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.14/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C metadata=9b2f7970571136de -C extra-filename=-9b2f7970571136de --out-dir /checkout/obj/build/bootstrap/debug/build/log-9b2f7970571136de -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-24f36aeef90a945b.so(+0x5141e3)[0x146bdb5cf1e3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x11390)[0x146bdab40390]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2dc44)[0x5650c6558c44]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f5ee)[0x5650c655a5ee]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x324e5)[0x5650c655d4e5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f2d5)[0x5650c655a2d5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16b0f)[0x5650c6541b0f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16d0a)[0x5650c6541d0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x11eac)[0x5650c653ceac]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x4eb0a)[0x5650c6579b0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x5093c)[0x5650c657b93c]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x6439)[0x146bdab35439]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x7870)[0x146bdab36870]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x146bda46051d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-24f36aeef90a945b.so(+0x5141e3)[0x14f5ae9d21e3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x11390)[0x14f5adf43390]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2dc44)[0x55accf04ec44]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f5ee)[0x55accf0505ee]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x32730)[0x55accf053730]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x2f2d5)[0x55accf0502d5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16b0f)[0x55accf037b0f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x16d0a)[0x55accf037d0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x11eac)[0x55accf032eac]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x4eb0a)[0x55accf06fb0a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc(+0x5093c)[0x55accf07193c]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x6439)[0x14f5adf38439]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x7870)[0x14f5adf39870]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x6d)[0x14f5ad86351d]
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:54
make: *** [prepare] Error 1
Command failed. Attempt 2/5:
---
[RUSTC-TIMING] rustc_metadata test:false 92.932
[RUSTC-TIMING] rustc_trait_selection test:false 95.271
[RUSTC-TIMING] rustc_codegen_llvm test:false 86.659
[RUSTC-TIMING] rustc_borrowck test:false 42.881
rustc exited with signal: 11 (core dumped)
error: could not compile `rustc_borrowck`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_borrowck --edition=2021 compiler/rustc_borrowck/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=711f9dd5d7e79303 -C extra-filename=-711f9dd5d7e79303 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-22689a2d6fcd3999.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-f4499dd7c94cbb44.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-031a7fdf571a7cc7.rmeta --extern rustc_const_eval=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_eval-765f1acbbec9e4a9.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-30a64d05055291c3.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-8003aed71033b8b3.rmeta --extern rustc_graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_graphviz-95cf7ef2d7540565.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-937d0446e843a197.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-64eee00a990e7e53.rmeta --extern rustc_infer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_infer-c4eb6c6895717d29.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-1a00f02c9b78b4a0.rmeta --extern rustc_middle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_middle-f34f602ead7b7a1d.rmeta --extern rustc_mir_dataflow=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir_dataflow-6587e38851962f4a.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-c2526c6a8b93609c.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-f061ae782a45fb6c.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-203f89f964b22f66.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-b0b23a8392f1fa97.rmeta --extern rustc_trait_selection=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trait_selection-db175926b974464a.rmeta --extern rustc_traits=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_traits-658d3e7953f07901.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-54b21886cbe00d4c.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-8f954d29b56e1ff8.rmeta --cfg=bootstrap -Zsymbol-mangling-version=v0 -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/psm-b85bcfe9d1b8d998/out` (exit status: 254)
[RUSTC-TIMING] rustc_middle test:false 131.416
[RUSTC-TIMING] rustc_interface test:false 47.260
[RUSTC-TIMING] rustc_mir_transform test:false 60.021
[RUSTC-TIMING] rustc_query_impl test:false 122.323
