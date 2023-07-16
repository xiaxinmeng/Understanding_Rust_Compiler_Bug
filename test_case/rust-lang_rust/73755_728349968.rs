
> cat Xargo.toml 
[target.wasm32-unknown-unknown.dependencies.std]
features = []

> RUSTFLAGS="-C target-feature=+multivalue" xargo +nightly build --target wasm32-unknown-unknown
warning: Patch `rustc-std-workspace-std v1.99.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-std)` was not used in the crate graph.
Check that the patched package version and available features are compatible
with the dependency requirements. If the patch has a different version from
what is locked in the Cargo.lock file, run `cargo update` to use the new
version. This may also occur with an optional dependency that is not enabled.
   Compiling compiler_builtins v0.1.36
   Compiling core v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core)
   Compiling libc v0.2.79
   Compiling cc v1.0.60
   Compiling dlmalloc v0.1.4
   Compiling std v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std)
   Compiling unwind v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling alloc v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling rustc-demangle v0.1.18
   Compiling panic_abort v0.0.0 (/home/user/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/panic_abort)
error: could not compile `compiler_builtins`

Caused by:
  process didn't exit successfully: `rustc --crate-name compiler_builtins /home/user/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.36/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=635019da5e105c8b -C extra-filename=-635019da5e105c8b --out-dir /tmp/xargo.mX5YtAnFkwJX/target/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/tmp/xargo.mX5YtAnFkwJX/target/wasm32-unknown-unknown/release/deps -L dependency=/tmp/xargo.mX5YtAnFkwJX/target/release/deps --extern core=/tmp/xargo.mX5YtAnFkwJX/target/wasm32-unknown-unknown/release/deps/librustc_std_workspace_core-4718ce578524c9f5.rmeta --cap-lints allow -C target-feature=+multivalue --sysroot /home/user/.xargo -Z force-unstable-if-unmarked --cfg 'feature="unstable"' --cfg 'feature="mem"'` (signal: 7, SIGBUS: access to undefined memory)
warning: build failed, waiting for other jobs to finish...
error: build failed
error: `"cargo" "build" "--release" "--manifest-path" "/tmp/xargo.mX5YtAnFkwJX/Cargo.toml" "--target" "wasm32-unknown-unknown" "-p" "std"` failed with exit code: Some(101)
note: run with `RUST_BACKTRACE=1` for a backtrace
