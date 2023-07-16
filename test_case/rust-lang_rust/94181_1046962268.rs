plain
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.76.0
The following warnings were emitted during compilation:

warning: llvm-wrapper/RustWrapper.cpp: In function ‘llvm::DICompileUnit::DebugNameTableKind fromRust(LLVMRustDebugNameTableKind)’:
warning: llvm-wrapper/RustWrapper.cpp:717:47: error: ‘Gnu’ is not a member of ‘llvm::DICompileUnit::DebugNameTableKind’
warning:   717 |     return DICompileUnit::DebugNameTableKind::Gnu;
warning:       |                                               ^~~
warning: llvm-wrapper/RustWrapper.cpp: In function ‘LLVMOpaqueMetadata* LLVMRustDIBuilderCreateCompileUnit(LLVMRustDIBuilderRef, unsigned int, LLVMMetadataRef, const char*, size_t, bool, const char*, unsigned int, const char*, size_t, LLVMRustDebugEmissionKind, uint64_t, bool, LLVMRustDebugNameTableKind)’:
warning: llvm-wrapper/RustWrapper.cpp:802:85: error: expected ‘)’ before ‘;’ token
warning:   802 |                                          DebugInfoForProfiling, fromRust(TableKind));
warning:       |                                                                                     )
warning: llvm-wrapper/RustWrapper.cpp:798:14: note: to match this ‘(’
warning: llvm-wrapper/RustWrapper.cpp:798:14: note: to match this ‘(’
warning:   798 |   return wrap(Builder->createCompileUnit(Lang, File, StringRef(Producer, ProducerLen),

error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-c5c973c13747b49c/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=RUST_CHECK
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
  cargo:rerun-if-env-changed=LLVM_CONFIG
  cargo:rerun-if-changed=/usr/lib/llvm-12/bin/llvm-config
  cargo:rustc-cfg=llvm_component="aarch64"
  cargo:rustc-cfg=llvm_component="amdgpu"
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
  cargo:rerun-if-changed=llvm-wrapper/PassWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/ArchiveWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/CoverageMappingWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/LLVMWrapper.h
  cargo:rerun-if-changed=llvm-wrapper/RustWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/.editorconfig
  cargo:rerun-if-changed=llvm-wrapper/README
  cargo:rerun-if-changed=llvm-wrapper/Linker.cpp
  TARGET = Some("x86_64-unknown-linux-gnu")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-unknown-linux-gnu")
  CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
  CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
  running: "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-12/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-a07adcddf35006a8/out/llvm-wrapper/PassWrapper.o" "-c" "llvm-wrapper/PassWrapper.cpp"
  exit status: 0
  running: "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-12/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-a07adcddf35006a8/out/llvm-wrapper/RustWrapper.o" "-c" "llvm-wrapper/RustWrapper.cpp"
  cargo:warning=llvm-wrapper/RustWrapper.cpp: In function ‘llvm::DICompileUnit::DebugNameTableKind fromRust(LLVMRustDebugNameTableKind)’:
  cargo:warning=llvm-wrapper/RustWrapper.cpp:717:47: error: ‘Gnu’ is not a member of ‘llvm::DICompileUnit::DebugNameTableKind’
  cargo:warning=  717 |     return DICompileUnit::DebugNameTableKind::Gnu;
  cargo:warning=      |                                               ^~~
  cargo:warning=llvm-wrapper/RustWrapper.cpp: In function ‘LLVMOpaqueMetadata* LLVMRustDIBuilderCreateCompileUnit(LLVMRustDIBuilderRef, unsigned int, LLVMMetadataRef, const char*, size_t, bool, const char*, unsigned int, const char*, size_t, LLVMRustDebugEmissionKind, uint64_t, bool, LLVMRustDebugNameTableKind)’:
  cargo:warning=llvm-wrapper/RustWrapper.cpp:802:85: error: expected ‘)’ before ‘;’ token
  cargo:warning=  802 |                                          DebugInfoForProfiling, fromRust(TableKind));
  cargo:warning=      |                                                                                     ^
  cargo:warning=      |                                                                                     )
  cargo:warning=llvm-wrapper/RustWrapper.cpp:798:14: note: to match this ‘(’
  cargo:warning=  798 |   return wrap(Builder->createCompileUnit(Lang, File, StringRef(Producer, ProducerLen),
  cargo:warning=      |              ^

  --- stderr



  error occurred: Command "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-12/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-a07adcddf35006a8/out/llvm-wrapper/RustWrapper.o" "-c" "llvm-wrapper/RustWrapper.cpp" with args "c++" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:01
