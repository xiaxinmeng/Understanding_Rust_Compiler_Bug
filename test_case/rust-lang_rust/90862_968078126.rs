plain
   Compiling once_cell v1.7.2
error: could not compile `version_check`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name version_check /cargo/registry/src/github.com-1ecc6299db9ec823/version_check-0.9.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=f9c226c5b81333d5 -C extra-filename=-f9c226c5b81333d5 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:54
make: *** [prepare] Error 1
---
   Compiling libc v0.2.106
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
[RUSTC-TIMING] build_script_build test:false 0.232
rustc exited with signal: 11 (core dumped)
error: could not compile `libc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.106/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=1 -C debug-assertions=off --cfg 'feature="align"' --cfg 'feature="rustc-dep-of-std"' --cfg 'feature="rustc-std-workspace-core"' -C metadata=2b8b2ef58be86c38 -C extra-filename=-2b8b2ef58be86c38 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/libc-2b8b2ef58be86c38 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] build_script_build test:false 0.340
[RUSTC-TIMING] build_script_build test:false 0.345
[RUSTC-TIMING] cc test:false 0.775
[RUSTC-TIMING] core test:false 16.227
