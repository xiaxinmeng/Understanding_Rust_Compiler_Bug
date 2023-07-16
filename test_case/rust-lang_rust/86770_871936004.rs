plain
   Compiling tempfile v3.2.0
   Compiling synstructure v0.12.4
The following warnings were emitted during compilation:

warning: llvm-wrapper/RustWrapper.cpp: In function 'llvm::Attribute::AttrKind fromRust(LLVMRustAttribute)':
warning: llvm-wrapper/RustWrapper.cpp:155:8: error: reference to 'NoAlias' is ambiguous
warning:    case NoAlias:
warning: In file included from llvm-wrapper/RustWrapper.cpp:1:0:
warning: llvm-wrapper/LLVMWrapper.h:59:3: note: candidates are: LLVMRustAttribute NoAlias
warning:    NoAlias = 6,
warning:    ^~~~~~~
warning:    ^~~~~~~
warning: In file included from /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:22:0,
warning:                  from llvm-wrapper/RustWrapper.cpp:14:
warning: /usr/lib/llvm-10/include/llvm/Analysis/AliasAnalysis.h:84:3: note:                 llvm::AliasResult NoAlias
warning:    NoAlias = 0,
warning:    ^~~~~~~
warning: llvm-wrapper/RustWrapper.cpp: In function 'void LLVMRustUnsafeFPMathApplyOnFunctions(LLVMModuleRef)':
warning: llvm-wrapper/RustWrapper.cpp:499:38: error: no matching function for call to 'InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&)'
warning:          if (InlineFunction(*Call, IFI).isSuccess()) {
warning: In file included from llvm-wrapper/RustWrapper.cpp:14:0:
warning: In file included from llvm-wrapper/RustWrapper.cpp:14:0:
warning: /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:232:14: note: candidate: llvm::InlineResult llvm::InlineFunction(llvm::CallBase*, llvm::InlineFunctionInfo&, llvm::AAResults*, bool)
warning:  InlineResult InlineFunction(CallBase *CB, InlineFunctionInfo &IFI,
warning:               ^~~~~~~~~~~~~~
warning: /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:232:14: note:   no known conversion for argument 1 from 'llvm::CallBase' to 'llvm::CallBase*'
warning: /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:235:14: note: candidate: llvm::InlineResult llvm::InlineFunction(llvm::CallSite, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*)
warning:  InlineResult InlineFunction(CallSite CS, InlineFunctionInfo &IFI,
warning:               ^~~~~~~~~~~~~~
warning: /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:235:14: note:   no known conversion for argument 1 from 'llvm::CallBase' to 'llvm::CallSite'
warning: llvm-wrapper/RustWrapper.cpp:500:40: error: cannot convert 'llvm::CallSite' to 'llvm::CallBase*' in initialization
warning:            for (CallBase *SubCall : IFI.InlinedCallSites) {

error: failed to run custom build command for `rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/rustc_llvm-12391d1f6a2f725d/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=RUST_CHECK
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH_VAR
  cargo:rerun-if-env-changed=REAL_LIBRARY_PATH
  cargo:rerun-if-env-changed=LLVM_CONFIG
  cargo:rerun-if-changed=/usr/lib/llvm-10/bin/llvm-config
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
  cargo:rerun-if-changed=llvm-wrapper/README
  cargo:rerun-if-changed=llvm-wrapper/PassWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/CoverageMappingWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/LLVMWrapper.h
  cargo:rerun-if-changed=llvm-wrapper/.editorconfig
  cargo:rerun-if-changed=llvm-wrapper/RustWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/ArchiveWrapper.cpp
  cargo:rerun-if-changed=llvm-wrapper/Linker.cpp
  TARGET = Some("x86_64-unknown-linux-gnu")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-unknown-linux-gnu")
  CXX_x86_64-unknown-linux-gnu = Some("sccache c++")
  CXXFLAGS_x86_64-unknown-linux-gnu = Some("-ffunction-sections -fdata-sections -fPIC -m64")
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
  running: "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-10/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-f0d687bec95eca47/out/llvm-wrapper/PassWrapper.o" "-c" "llvm-wrapper/PassWrapper.cpp"
  exit status: 0
  running: "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-10/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-f0d687bec95eca47/out/llvm-wrapper/RustWrapper.o" "-c" "llvm-wrapper/RustWrapper.cpp"
  cargo:warning=llvm-wrapper/RustWrapper.cpp: In function 'llvm::Attribute::AttrKind fromRust(LLVMRustAttribute)':
  cargo:warning=llvm-wrapper/RustWrapper.cpp:155:8: error: reference to 'NoAlias' is ambiguous
  cargo:warning=   case NoAlias:
  cargo:warning=        ^~~~~~~
  cargo:warning=In file included from llvm-wrapper/RustWrapper.cpp:1:0:
  cargo:warning=llvm-wrapper/LLVMWrapper.h:59:3: note: candidates are: LLVMRustAttribute NoAlias
  cargo:warning=   NoAlias = 6,
  cargo:warning=   ^~~~~~~
  cargo:warning=In file included from /usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:22:0,
  cargo:warning=                 from llvm-wrapper/RustWrapper.cpp:14:
  cargo:warning=/usr/lib/llvm-10/include/llvm/Analysis/AliasAnalysis.h:84:3: note:                 llvm::AliasResult NoAlias
  cargo:warning=   NoAlias = 0,
  cargo:warning=   ^~~~~~~
  cargo:warning=llvm-wrapper/RustWrapper.cpp: In function 'void LLVMRustUnsafeFPMathApplyOnFunctions(LLVMModuleRef)':
  cargo:warning=llvm-wrapper/RustWrapper.cpp:499:38: error: no matching function for call to 'InlineFunction(llvm::CallBase&, llvm::InlineFunctionInfo&)'
  cargo:warning=         if (InlineFunction(*Call, IFI).isSuccess()) {
  cargo:warning=                                      ^
  cargo:warning=In file included from llvm-wrapper/RustWrapper.cpp:14:0:
  cargo:warning=/usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:232:14: note: candidate: llvm::InlineResult llvm::InlineFunction(llvm::CallBase*, llvm::InlineFunctionInfo&, llvm::AAResults*, bool)
  cargo:warning= InlineResult InlineFunction(CallBase *CB, InlineFunctionInfo &IFI,
  cargo:warning=              ^~~~~~~~~~~~~~
  cargo:warning=/usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:232:14: note:   no known conversion for argument 1 from 'llvm::CallBase' to 'llvm::CallBase*'
  cargo:warning=/usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:235:14: note: candidate: llvm::InlineResult llvm::InlineFunction(llvm::CallSite, llvm::InlineFunctionInfo&, llvm::AAResults*, bool, llvm::Function*)
  cargo:warning= InlineResult InlineFunction(CallSite CS, InlineFunctionInfo &IFI,
  cargo:warning=              ^~~~~~~~~~~~~~
  cargo:warning=/usr/lib/llvm-10/include/llvm/Transforms/Utils/Cloning.h:235:14: note:   no known conversion for argument 1 from 'llvm::CallBase' to 'llvm::CallSite'
  cargo:warning=llvm-wrapper/RustWrapper.cpp:500:40: error: cannot convert 'llvm::CallSite' to 'llvm::CallBase*' in initialization
  cargo:warning=           for (CallBase *SubCall : IFI.InlinedCallSites) {
  cargo:warning=                                        ^~~~~~~~~~~~~~~~

  --- stderr



  error occurred: Command "sccache" "c++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-I/usr/lib/llvm-10/include" "-std=c++14" "-fno-exceptions" "-D_GNU_SOURCE" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_AVR" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_BPF" "-DLLVM_COMPONENT_COVERAGE" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-DNDEBUG" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/rustc_llvm-f0d687bec95eca47/out/llvm-wrapper/RustWrapper.o" "-c" "llvm-wrapper/RustWrapper.cpp" with args "c++" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
