plain
   Compiling toml v0.5.7
error: could not compile `bootstrap`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name sccache_plus_cl --edition=2021 src/bootstrap/bin/sccache-plus-cl.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=1 -C metadata=27e3ea28a7db2c9c -C extra-filename=-27e3ea28a7db2c9c --out-dir /checkout/obj/build/bootstrap/debug/deps -C incremental=/checkout/obj/build/bootstrap/debug/incremental -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern bootstrap=/checkout/obj/build/bootstrap/debug/deps/libbootstrap-e9b4854153dc8f58.rlib --extern build_helper=/checkout/obj/build/bootstrap/debug/deps/libbuild_helper-80dc2551c9f938e1.rlib --extern cc=/checkout/obj/build/bootstrap/debug/deps/libcc-798fd2f715e7473d.rlib --extern cmake=/checkout/obj/build/bootstrap/debug/deps/libcmake-d356a367916a0290.rlib --extern filetime=/checkout/obj/build/bootstrap/debug/deps/libfiletime-f59674a1674a3a9b.rlib --extern getopts=/checkout/obj/build/bootstrap/debug/deps/libgetopts-2bcbd4bdc08a7fa6.rlib --extern ignore=/checkout/obj/build/bootstrap/debug/deps/libignore-fbd7a9f95f8d14f2.rlib --extern lazy_static=/checkout/obj/build/bootstrap/debug/deps/liblazy_static-cdaa53bf1539807c.rlib --extern libc=/checkout/obj/build/bootstrap/debug/deps/liblibc-cf3c40d4327da022.rlib --extern merge=/checkout/obj/build/bootstrap/debug/deps/libmerge-b8020cf330871c37.rlib --extern num_cpus=/checkout/obj/build/bootstrap/debug/deps/libnum_cpus-4cc91832fc79957e.rlib --extern once_cell=/checkout/obj/build/bootstrap/debug/deps/libonce_cell-9fd1f48b4eb1d8e6.rlib --extern opener=/checkout/obj/build/bootstrap/debug/deps/libopener-5dfb998b2def0df0.rlib --extern serde=/checkout/obj/build/bootstrap/debug/deps/libserde-3350323baf42840b.rlib --extern serde_json=/checkout/obj/build/bootstrap/debug/deps/libserde_json-562e09b696b9c612.rlib --extern time=/checkout/obj/build/bootstrap/debug/deps/libtime-25b82c3a3f31c474.rlib --extern toml=/checkout/obj/build/bootstrap/debug/deps/libtoml-bc2270171bca3c27.rlib -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:01:24
make: *** [prepare] Error 1
---
[RUSTC-TIMING] panic_abort test:false 0.067
[RUSTC-TIMING] panic_unwind test:false 0.204
[RUSTC-TIMING] std_detect test:false 0.181
[RUSTC-TIMING] alloc test:false 2.243
rustc exited with signal: 11 (core dumped)
error: could not compile `alloc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name alloc --edition=2018 library/alloc/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 --cfg 'feature="compiler-builtins-c"' -C metadata=0af08cfec9ed4d64 -C extra-filename=-0af08cfec9ed4d64 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-4a1040813ee62119.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-8cd117af8e93b26c.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/beta/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-061c741d495f62b5/out` (exit status: 254)
[RUSTC-TIMING] hashbrown test:false 0.603
[RUSTC-TIMING] miniz_oxide test:false 2.023
[RUSTC-TIMING] core test:false 17.717
[RUSTC-TIMING] gimli test:false 4.017
