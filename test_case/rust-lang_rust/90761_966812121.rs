plain
   Compiling cmake v0.1.44
error: could not compile `crossbeam-utils`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.3/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' --cfg 'feature="lazy_static"' --cfg 'feature="std"' -C metadata=7d86092267134ed5 -C extra-filename=-7d86092267134ed5 --out-dir /checkout/obj/build/bootstrap/debug/build/crossbeam-utils-7d86092267134ed5 -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern autocfg=/checkout/obj/build/bootstrap/debug/deps/libautocfg-fa5fb2f81f71684d.rlib --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 6, SIGABRT: process abort signal)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:56
make: *** [prepare] Error 1
---
[RUSTC-TIMING] build_script_build test:false 0.381
[RUSTC-TIMING] build_script_build test:false 0.672
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.185
rustc exited with signal: 11 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_std_workspace_core --edition=2018 library/rustc-std-workspace-core/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C metadata=fd44d8b17c2359b1 -C extra-filename=-fd44d8b17c2359b1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-2a8f88aa3a5cb424.rmeta --cfg=bootstrap -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] libc test:false 0.682
[RUSTC-TIMING] compiler_builtins test:false 0.709
[RUSTC-TIMING] core test:false 16.299
error: build failed
