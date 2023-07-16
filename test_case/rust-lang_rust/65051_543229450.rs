
TARGET = Some("x86_64-unknown-linux-musl")
OPT_LEVEL = Some("2")
HOST = Some("x86_64-unknown-linux-musl")
CXX_x86_64-unknown-linux-musl = Some("clang++")
CXXFLAGS_x86_64-unknown-linux-musl = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-musl")
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
running: "clang++" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-musl" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-unknown-linux-musl" "-I//include" "-std=c++11" "-stdlib=libc++" "-D__STDC_CONSTANT_MACROS" "-D__STDC_FORMAT_MACROS" "-D__STDC_LIMIT_MACROS" "-DLLVM_COMPONENT_AARCH64" "-DLLVM_COMPONENT_AMDGPU" "-DLLVM_COMPONENT_ARM" "-DLLVM_COMPONENT_ASMPARSER" "-DLLVM_COMPONENT_BITREADER" "-DLLVM_COMPONENT_BITWRITER" "-DLLVM_COMPONENT_HEXAGON" "-DLLVM_COMPONENT_INSTRUMENTATION" "-DLLVM_COMPONENT_INTERPRETER" "-DLLVM_COMPONENT_IPO" "-DLLVM_COMPONENT_LINKER" "-DLLVM_COMPONENT_LTO" "-DLLVM_COMPONENT_MCJIT" "-DLLVM_COMPONENT_MIPS" "-DLLVM_COMPONENT_MSP430" "-DLLVM_COMPONENT_NVPTX" "-DLLVM_COMPONENT_POWERPC" "-DLLVM_COMPONENT_RISCV" "-DLLVM_COMPONENT_SPARC" "-DLLVM_COMPONENT_SYSTEMZ" "-DLLVM_COMPONENT_WEBASSEMBLY" "-DLLVM_COMPONENT_X86" "-o" "/src/vanilla/lang/rust/rustc-1.37.0-src/build/x86_64-unknown-linux-musl/stage0-codegen/x86_64-unknown-linux-musl/release/build/rustc_llvm-73aad4fac499ea8f/out/../rustllvm/PassWrapper.o" "-c" "../rustllvm/PassWrapper.cpp"
cargo:warning=../rustllvm/PassWrapper.cpp:917:3: error: no matching function for call to 'thinLTOResolvePrevailingInIndex'
cargo:warning=  thinLTOResolvePrevailingInIndex(Ret->Index, isPrevailing, recordNewLinkage);
cargo:warning=  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
cargo:warning=//include/llvm/LTO/LTO.h:49:6: note: candidate function not viable: requires 4 arguments, but 3 were provided
cargo:warning=void thinLTOResolvePrevailingInIndex(
cargo:warning=     ^
cargo:warning=1 error generated.
exit code: 1
