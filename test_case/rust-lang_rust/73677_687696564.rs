
   Compiling compiler_builtins v0.1.32
   Compiling core v0.0.0 (/Volumes/Data/rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling rustc-std-workspace-core v1.99.0 (/Volumes/Data/rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/rustc-std-workspace-core)
error: could not compile `core`.

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2018 /Volumes/Data/rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=acd030e704ce6404 -C extra-filename=-acd030e704ce6404 --out-dir /Volumes/Data/temp/poc/target/aarch64-unknown-none-softfloat/debug/deps --target aarch64-unknown-none-softfloat -Zforce-unstable-if-unmarked -L dependency=/Volumes/Data/temp/poc/target/aarch64-unknown-none-softfloat/debug/deps -L dependency=/Volumes/Data/temp/poc/target/debug/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
error: build failed

