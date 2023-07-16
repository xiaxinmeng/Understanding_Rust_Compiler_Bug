plain
   Compiling cc v1.0.69
error: could not compile `unicode-width`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name unicode_width /cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.8/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="default"' -C metadata=58a25f3a30d31e30 -C extra-filename=-58a25f3a30d31e30 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:54
make: *** [prepare] Error 1
---
   Compiling getopts v0.2.21
error: could not compile `proc-macro-error-attr`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro-error-attr-1.0.4/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C metadata=4f6b81b2163c9b70 -C extra-filename=-4f6b81b2163c9b70 --out-dir /checkout/obj/build/bootstrap/debug/build/proc-macro-error-attr-4f6b81b2163c9b70 -L dependency=/checkout/obj/build/bootstrap/debug/deps --extern version_check=/checkout/obj/build/bootstrap/debug/deps/libversion_check-f9c226c5b81333d5.rlib --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 6, SIGABRT: process abort signal)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:01
make: *** [prepare] Error 1
---
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
[RUSTC-TIMING] cfg_if test:false 0.036
[RUSTC-TIMING] adler test:false 0.270
rustc exited with signal: 11 (core dumped)
error: could not compile `adler`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name adler /cargo/registry/src/github.com-1ecc6299db9ec823/adler-0.2.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 --cfg 'feature="compiler_builtins"' --cfg 'feature="core"' --cfg 'feature="rustc-dep-of-std"' -C metadata=2de8c5ba493bc641 -C extra-filename=-2de8c5ba493bc641 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-6bdc4f8396c65e15.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-fd44d8b17c2359b1.rmeta --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=legacy -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-399cb2af88d0d65a/out` (exit status: 254)
[RUSTC-TIMING] libc test:false 0.699
[RUSTC-TIMING] compiler_builtins test:false 0.749
[RUSTC-TIMING] memchr test:false 1.341
[RUSTC-TIMING] rustc_demangle test:false 1.522
