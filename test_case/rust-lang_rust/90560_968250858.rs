plain
[RUSTC-TIMING] rustc_std_workspace_std test:false 0.047
   Compiling getopts v0.2.21
[RUSTC-TIMING] unicode_width test:false 0.069
[RUSTC-TIMING] proc_macro test:false 0.164
rustc exited with signal: 6 (core dumped)
error: could not compile `proc_macro`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name proc_macro --edition=2018 library/proc_macro/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C metadata=5c4876908862739f -C extra-filename=-5c4876908862739f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-c4fe96599cfce917.so --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-c4fe96599cfce917.rlib --cfg=bootstrap -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-399cb2af88d0d65a/out` (exit status: 254)
[RUSTC-TIMING] getopts test:false 1.844
error: build failed
Build completed unsuccessfully in 0:00:40
