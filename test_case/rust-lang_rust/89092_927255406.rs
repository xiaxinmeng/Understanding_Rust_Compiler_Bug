plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMvshtU76pxd

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
    Finished release [optimized] target(s) in 3m 11s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None }, tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 192.011
[TIMING] Tidy { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-apple-darwin", file: None } }, target: TargetSelection { triple: "x86_64-apple-darwin", file: None } } -- 0.008
tidy check
thread '<unnamed>' panicked at 'cmd.exec() failed with Error during execution of `cargo metadata`:     Updating git repository `https://github.com/bjorn3/rust-ar.git`
    Updating git submodule `https://github.com/WebAssembly/WASI`
    Updating git submodule `https://github.com/WebAssembly/wasm-c-api`
    Updating git submodule `https://github.com/WebAssembly/wasi-crypto.git`


    Updating git submodule `https://github.com/WebAssembly/wasi-nn`

warning: spurious network error (2 tries remaining): failed to resolve address for github.com: nodename nor servname provided, or not known; class=Net (12)
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/rust-tidy" "/Users/runner/work/rust/rust" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "/Users/runner/work/rust/rust/build" "3"
warning: spurious network error (1 tries remaining): failed to resolve address for github.com: nodename nor servname provided, or not known; class=Net (12)
expected success, got: exit status: 101
error: failed to get `cranelift-codegen` as a dependency of package `rustc_codegen_cranelift v0.1.0 (/Users/runner/work/rust/rust/compiler/rustc_codegen_cranelift)`


Caused by:
Build completed unsuccessfully in 0:04:44
Build completed unsuccessfully in 0:04:44
  failed to load source for dependency `cranelift-codegen`

Caused by:
  Unable to update https://github.com/bytecodealliance/wasmtime.git#9c550fcf

Caused by:
  failed to update submodule `crates/wasi-nn/spec`
Caused by:
Caused by:
  failed to fetch submodule `crates/wasi-nn/spec` from https://github.com/WebAssembly/wasi-nn
Caused by:
  network failure seems to have happened
  network failure seems to have happened
  if a proxy or similar is necessary `net.git-fetch-with-cli` may help here
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli
Caused by:
Caused by:
  failed to resolve address for github.com: nodename nor servname provided, or not known; class=Net (12)
', src/tools/tidy/src/deps.rs:294:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', src/tools/tidy/src/main.rs:66:9
