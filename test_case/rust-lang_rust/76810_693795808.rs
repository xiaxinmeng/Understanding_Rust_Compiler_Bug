
running: "ar" "crs" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/libllvm-wrapper.a" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/llvm-wrapper/PassWrapper.o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/llvm-wrapper/RustWrapper.o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/llvm-wrapper/ArchiveWrapper.o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/llvm-wrapper/CoverageMappingWrapper.o" "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out/llvm-wrapper/Linker.o"
  exit code: 0
  cargo:rustc-link-lib=static=llvm-wrapper
  cargo:rustc-link-search=native=/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7998fe2b8433c8ac/out
  cargo:rerun-if-env-changed=LLVM_LINK_SHARED

  --- stderr
  llvm-config: error: libLLVM-11-rust-1.48.0-nightly.dylib is missing
  thread 'main' panicked at 'command did not execute successfully: "/Users/runner/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/llvm-config" "--link-shared" "--libs" "--system-libs" "aarch64" "arm" "asmparser" "avr" "bitreader" "bitwriter" "coverage" "hexagon" "instrumentation" "ipo" "linker" "lto" "mips" "msp430" "nvptx" "powerpc" "riscv" "sparc" "systemz" "webassembly" "x86"
  expected success, got: exit code: 1', src/build_helper/lib.rs:139:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] indexmap test:false 0.940
[RUSTC-TIMING] serde test:false 9.905
error: build failed
command did not execute successfully: "/Users/runner/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/Users/runner/work/1/s/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /Users/runner/work/1/s/build/bootstrap/debug/bootstrap dist
