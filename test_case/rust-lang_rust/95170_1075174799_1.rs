
cargo +beta build --bins && cargo +beta run check
    Finished dev [unoptimized] target(s) in 0.14s
    Finished dev [unoptimized] target(s) in 0.07s
     Running `target/debug/bootstrap check`
downloading https://ci-artifacts.rust-lang.org/rustc-builds/282778aee26166754315815552bae454fc968960/rust-dev-nightly-aarch64-apple-darwin.tar.xz
######################################################################################################################################################################################################################################################### 100.0%
extracting /Users/walther/git/rust/build/cache/llvm-282778aee26166754315815552bae454fc968960-false/rust-dev-nightly-aarch64-apple-darwin.tar.xz to /Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm
Checking stage0 std artifacts (aarch64-apple-darwin -> aarch64-apple-darwin)
    Finished release [optimized] target(s) in 0.07s
Checking stage0 std test/bench/example targets (aarch64-apple-darwin -> aarch64-apple-darwin)
    Finished release [optimized] target(s) in 0.07s
Checking stage0 compiler artifacts (aarch64-apple-darwin -> aarch64-apple-darwin)
   Compiling rustc_llvm v0.0.0 (/Users/walther/git/rust/compiler/rustc_llvm)
    Checking rustc_infer v0.0.0 (/Users/walther/git/rust/compiler/rustc_infer)
    Checking rustc_mir_dataflow v0.0.0 (/Users/walther/git/rust/compiler/rustc_mir_dataflow)
    Checking rustc_symbol_mangling v0.0.0 (/Users/walther/git/rust/compiler/rustc_symbol_mangling)
    Checking rustc_incremental v0.0.0 (/Users/walther/git/rust/compiler/rustc_incremental)
    Checking rustc_monomorphize v0.0.0 (/Users/walther/git/rust/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/Users/walther/git/rust/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/Users/walther/git/rust/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/Users/walther/git/rust/compiler/rustc_save_analysis)
    Checking rustc_middle v0.0.0 (/Users/walther/git/rust/compiler/rustc_middle)
    Checking rustc_metadata v0.0.0 (/Users/walther/git/rust/compiler/rustc_metadata)
    Checking rustc_trait_selection v0.0.0 (/Users/walther/git/rust/compiler/rustc_trait_selection)
    Checking rustc_codegen_ssa v0.0.0 (/Users/walther/git/rust/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/Users/walther/git/rust/compiler/rustc_resolve)
    Checking rustc_lint v0.0.0 (/Users/walther/git/rust/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/Users/walther/git/rust/compiler/rustc_ty_utils)
    Checking rustc_const_eval v0.0.0 (/Users/walther/git/rust/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/Users/walther/git/rust/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/Users/walther/git/rust/compiler/rustc_mir_build)
    Checking rustc_typeck v0.0.0 (/Users/walther/git/rust/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/Users/walther/git/rust/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/Users/walther/git/rust/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/Users/walther/git/rust/compiler/rustc_borrowck)
error: failed to run custom build command for `rustc_llvm v0.0.0 (/Users/walther/git/rust/compiler/rustc_llvm)`

Caused by:
  process didn't exit successfully: `/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/release/build/rustc_llvm-a87a5ba16107693a/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=RUST_CHECK
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
  cargo:rerun-if-env-changed=LLVM_CONFIG
  cargo:rerun-if-changed=/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/bin/llvm-config
  cargo:rustc-cfg=llvm_component="aarch64"
  cargo:rustc-cfg=llvm_component="arm"
  cargo:rustc-cfg=llvm_component="asmparser"
  cargo:rustc-cfg=llvm_component="avr"
  cargo:rustc-cfg=llvm_component="bitreader"
  cargo:rustc-cfg=llvm_component="bitwriter"
  cargo:rustc-cfg=llvm_component="bpf"
  cargo:rustc-cfg=llvm_component="coverage"
  cargo:rustc-cfg=llvm_component="hexagon"
  cargo:rustc-cfg=llvm_component="instrumentation"
  cargo:rustc-cfg=llvm_component="ipo"
  cargo:rustc-cfg=llvm_component="linker"
  cargo:rustc-cfg=llvm_component="lto"
  cargo:rustc-cfg=llvm_component="m68k"
  cargo:rustc-cfg=llvm_component="mips"
  cargo:rustc-cfg=llvm_component="msp430"
  cargo:rustc-cfg=llvm_component="nvptx"
  cargo:rustc-cfg=llvm_component="powerpc"
  cargo:rustc-cfg=llvm_component="riscv"
  cargo:rustc-cfg=llvm_component="sparc"
  cargo:rustc-cfg=llvm_component="systemz"
  cargo:rustc-cfg=llvm_component="webassembly"
  cargo:rustc-cfg=llvm_component="x86"
  cargo:rerun-if-env-changed=LLVM_RUSTLLVM
  cargo:rerun-if-env-changed=LLVM_NDEBUG
  cargo:rerun-if-changed=llvm-wrapper/CoverageMappingWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/ArchiveWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/PassWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/.editorconfig
  cargo:rerun-if-changed=llvm-wrapper/README
  cargo:rerun-if-changed=llvm-wrapper/RustWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/Linker.cpp
  cargo:rerun-if-changed=llvm-wrapper/LLVMWrapper.h
  TARGET = Some("aarch64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("aarch64-apple-darwin")
  CXX_aarch64-apple-darwin = Some("c++")
  CXXFLAGS_aarch64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC -arch arm64 -stdlib=libc++")
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("aes,crc,dit,dotprod,dpb,dpb2,fcma,fhm,flagm,fp,fp16,frintts,jsconv,llvm14-builtins-abi,lor,lse,neon,paca,pacg,pan,pmuv3,ras,rcpc,rcpc2,rdm,sb,sha2,sha3,ssbs,v8.1a,v8.2a,v8.3a,v8.4a,vh")
  running: "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-stdlib=libc++" "-I/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/include" "-std=c++14" "-stdlib=libc++" "-fno-exceptions" "-fno-rtti" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_M68K" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DLLVM_RUSTLLVM" "-DNDEBUG" "-o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/PassWrapper.o" "-c" "llvm-wrapper/PassWrapper.cpp"
  exit status: 0
  running: "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-stdlib=libc++" "-I/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/include" "-std=c++14" "-stdlib=libc++" "-fno-exceptions" "-fno-rtti" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_M68K" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DLLVM_RUSTLLVM" "-DNDEBUG" "-o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/RustWrapper.o" "-c" "llvm-wrapper/RustWrapper.cpp"
  exit status: 0
  running: "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-stdlib=libc++" "-I/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/include" "-std=c++14" "-stdlib=libc++" "-fno-exceptions" "-fno-rtti" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_M68K" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DLLVM_RUSTLLVM" "-DNDEBUG" "-o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/ArchiveWrapper.o" "-c" "llvm-wrapper/ArchiveWrapper.cpp"
  exit status: 0
  running: "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-stdlib=libc++" "-I/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/include" "-std=c++14" "-stdlib=libc++" "-fno-exceptions" "-fno-rtti" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_M68K" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DLLVM_RUSTLLVM" "-DNDEBUG" "-o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/CoverageMappingWrapper.o" "-c" "llvm-wrapper/CoverageMappingWrapper.cpp"
  exit status: 0
  running: "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-arch" "arm64" "-stdlib=libc++" "-I/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/include" "-std=c++14" "-stdlib=libc++" "-fno-exceptions" "-fno-rtti" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_M68K" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DLLVM_RUSTLLVM" "-DNDEBUG" "-o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/Linker.o" "-c" "llvm-wrapper/Linker.cpp"
  exit status: 0
  AR_aarch64-apple-darwin = Some("ar")
  running: "ar" "cq" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/libllvm-wrapper.a" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/PassWrapper.o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/RustWrapper.o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/ArchiveWrapper.o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/CoverageMappingWrapper.o" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/llvm-wrapper/Linker.o"
  exit status: 0
  running: "ar" "s" "/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out/libllvm-wrapper.a"
  exit status: 0
  cargo:rustc-link-lib=static=llvm-wrapper
  cargo:rustc-link-search=native=/Users/walther/git/rust/build/aarch64-apple-darwin/stage0-rustc/aarch64-apple-darwin/release/build/rustc_llvm-d53b94a11110e0f0/out
  cargo:rerun-if-env-changed=LLVM_LINK_SHARED

  --- stderr
  llvm-config: error: libLLVM-14-rust-1.61.0-nightly.dylib is missing
  thread 'main' panicked at 'command did not execute successfully: "/Users/walther/git/rust/build/aarch64-apple-darwin/ci-llvm/bin/llvm-config" "--link-shared" "--libs" "--system-libs" "aarch64" "arm" "asmparser" "avr" "bitreader" "bitwriter" "bpf" "coverage" "hexagon" "instrumentation" "ipo" "linker" "lto" "m68k" "mips" "msp430" "nvptx" "powerpc" "riscv" "sparc" "systemz" "webassembly" "x86"
  expected success, got: exit status: 1', compiler/rustc_llvm/build.rs:252:16
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
cargo +beta run check  100.36s user 6.56s system 473% cpu 22.602 total
