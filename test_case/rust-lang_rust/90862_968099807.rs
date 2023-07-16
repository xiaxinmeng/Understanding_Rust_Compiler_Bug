plain
   Compiling once_cell v1.7.2
error: could not compile `version_check`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name version_check /cargo/registry/src/github.com-1ecc6299db9ec823/version_check-0.9.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=f9c226c5b81333d5 -C extra-filename=-f9c226c5b81333d5 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:55
make: *** [prepare] Error 1
---
[RUSTC-TIMING] build_script_build test:false 0.451
[RUSTC-TIMING] build_script_build test:false 0.475
[RUSTC-TIMING] ucd_trie test:false 0.504
[RUSTC-TIMING] build_script_build test:false 0.211
rustc exited with signal: 11 (core dumped)
error: could not compile `crossbeam-utils`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-utils-0.8.3/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="default"' --cfg 'feature="lazy_static"' --cfg 'feature="std"' -C metadata=6aa5a6e8a5ad6941 -C extra-filename=-6aa5a6e8a5ad6941 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/crossbeam-utils-6aa5a6e8a5ad6941 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern autocfg=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libautocfg-7f11345a43e868c8.rlib --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] ryu test:false 0.198
[RUSTC-TIMING] build_script_build test:false 0.337
[RUSTC-TIMING] walkdir test:false 0.691
[RUSTC-TIMING] proc_macro2 test:false 0.549
