
   Compiling core v0.0.0 (/Volumes/Data/rustup/toolchains/nightly-2020-05-22-x86_64-apple-darwin/lib/rustlib/src/rust/src/libcore)
   Compiling compiler_builtins v0.1.28
   Compiling rustc-std-workspace-core v1.99.0 (/Volumes/Data/rustup/toolchains/nightly-2020-05-22-x86_64-apple-darwin/lib/rustlib/src/rust/src/tools/rustc-std-workspace-core)
error: could not compile `core`.

Caused by:
  process didn't exit successfully: `rustc --crate-name core --edition=2018 /Volumes/Data/rustup/toolchains/nightly-2020-05-22-x86_64-apple-darwin/lib/rustlib/src/rust/src/libcore/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=602fa404d53a4f68 -C extra-filename=-602fa404d53a4f68 --out-dir /Volumes/Data/temp/poc/target/aarch64-unknown-none-softfloat/debug/deps --target aarch64-unknown-none-softfloat -Zforce-unstable-if-unmarked -L dependency=/Volumes/Data/temp/poc/target/aarch64-unknown-none-softfloat/debug/deps -L dependency=/Volumes/Data/temp/poc/target/debug/deps --cap-lints allow` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
