plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMTdPjYKNVxK

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
The following warnings were emitted during compilation:

warning: error: Connection to server timed out

error: failed to run custom build command for `psm v0.1.11`
Caused by:
Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/build/psm-d1f698f3a1071e45/build-script-build` (exit code: 1)
  --- stdout
  cargo:rustc-cfg=asm
  cargo:rustc-cfg=switchable_stack
  OPT_LEVEL = Some("3")
  TARGET = Some("x86_64-apple-darwin")
  HOST = Some("x86_64-apple-darwin")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/756679f9b2a7ac45714ffa94b48985f6e569aa7b")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/756679f9b2a7ac45714ffa94b48985f6e569aa7b" "-xassembler-with-cpp" "-DCFG_TARGET_OS_macos" "-DCFG_TARGET_ARCH_x86_64" "-DCFG_TARGET_ENV_" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/psm-210b72712d640ca3/out/src/arch/x86_64.o" "-c" "src/arch/x86_64.s"
  cargo:warning=error: Connection to server timed out

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/756679f9b2a7ac45714ffa94b48985f6e569aa7b" "-xassembler-with-cpp" "-DCFG_TARGET_OS_macos" "-DCFG_TARGET_ARCH_x86_64" "-DCFG_TARGET_ENV_" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/psm-210b72712d640ca3/out/src/arch/x86_64.o" "-c" "src/arch/x86_64.s" with args "clang" did not execute successfully (status code exit code: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] typenum test:false 54.939
error: build failed
error: build failed
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/Users/runner/work/rust/rust/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 3:58:19
