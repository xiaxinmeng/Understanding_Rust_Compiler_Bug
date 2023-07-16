plain
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] compile::StdLink { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.001
[TIMING] compile::Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu } } -- 52.611
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_BUILD_INSTRUMENTED=CSIR" "-DLLVM_VP_COUNTERS_PER_SITE=6" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.63.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_RANLIB=/rustroot/bin/llvm-ranlib" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 14.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
[ 16%] Building MipsGenCallingConv.inc...
[ 16%] Building BPFGenInstrInfo.inc...
[ 16%] Building MSP430GenInstrInfo.inc...
[ 16%] Building AArch64GenAsmWriter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building HexagonGenCallingConv.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building SparcGenInstrInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building PPCGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building SystemZGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building RISCVGenAsmWriter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/OrcError.cpp.o
[ 16%] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/OrcError.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building BPFGenMCCodeEmitter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building MipsGenDAGISel.inc...
[ 16%] Building MipsGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building ARMGenCallingConv.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building BPFGenRegisterInfo.inc...
[ 16%] Building BPFGenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building SparcGenMCCodeEmitter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building MSP430GenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building HexagonGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building BPFGenSubtargetInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building SparcGenRegisterInfo.inc...
[ 16%] Building SparcGenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building ARMGenDAGISel.inc...
[ 16%] Building ARMGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building SystemZGenDisassemblerTables.inc...
[ 16%] Building X86GenAsmMatcher.inc...
[ 16%] Building X86GenAsmMatcher.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Building WebAssemblyGenInstrInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 16%] Built target MSP430CommonTableGen
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building SparcGenSubtargetInfo.inc...
[ 17%] Building AVRGenAsmMatcher.inc...
[ 17%] Building AVRGenAsmMatcher.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building AArch64GenAsmWriter1.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building NVPTXGenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building MipsGenDisassemblerTables.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/BitstreamRemarkSerializer.cpp.o
[ 17%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/BitstreamRemarkSerializer.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
Scanning dependencies of target M68kCommonTableGen
[ 17%] Building M68kGenGlobalISel.inc...
[ 17%] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/OrcRTBridge.cpp.o
[ 17%] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/OrcRTBridge.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building SystemZGenInstrInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building AVRGenCallingConv.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building NVPTXGenSubtargetInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building PPCGenDisassemblerTables.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building RISCVGenCompressInstEmitter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building WebAssemblyGenMCCodeEmitter.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building AVRGenDAGISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building M68kGenRegisterInfo.inc...
[ 17%] Building M68kGenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building MipsGenFastISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building WebAssemblyGenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building M68kGenRegisterBank.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building AVRGenDisassemblerTables.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building AArch64GenCallingConv.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 17%] Building PPCGenFastISel.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
---
Scanning dependencies of target LLVMM68kDesc
[ 25%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kAsmBackend.cpp.o
[ 25%] Linking CXX static library ../libLLVMRemarks.a
[ 25%] Built target LLVMRemarks
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 25%] Linking CXX static library ../../../libLLVMPowerPCInfo.a
[ 25%] Building CXX object lib/Target/MSP430/MCTargetDesc/CMakeFiles/LLVMMSP430Desc.dir/MSP430MCTargetDesc.cpp.o
[ 25%] Built target LLVMPowerPCInfo
Scanning dependencies of target LLVMCore
---
[ 25%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCAsmBackend.cpp.o
Scanning dependencies of target LLVMHexagonDesc
[ 25%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonAsmBackend.cpp.o
[ 25%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kELFObjectWriter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 25%] Building AArch64GenMCCodeEmitter.inc...
[ 25%] Linking CXX static library ../../../libLLVMMSP430Desc.a
[ 25%] Built target LLVMMSP430Desc
[ 26%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
[ 26%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
Scanning dependencies of target LLVMMipsDesc
[ 26%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsABIInfo.cpp.o
[ 26%] Linking CXX static library ../../../libLLVMSparcDesc.a
[ 26%] Built target LLVMSparcDesc
[ 26%] Building CXX object lib/Target/ARM/MCTargetDesc/CMakeFiles/LLVMARMDesc.dir/ARMELFObjectWriter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCAsmInfo.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCCodeEmitter.cpp.o
[ 26%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCExpr.cpp.o
[ 26%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCExpr.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building X86GenRegisterBank.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kInstPrinter.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonELFObjectWriter.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonELFObjectWriter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCObjectWriter.cpp.o
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCObjectWriter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building AArch64GenRegisterInfo.inc...
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Built target RISCVCommonTableGen
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
Scanning dependencies of target LLVMMipsDisassembler
[ 26%] Building CXX object lib/Target/Mips/Disassembler/CMakeFiles/LLVMMipsDisassembler.dir/MipsDisassembler.cpp.o
[ 26%] Building CXX object lib/Target/Mips/Disassembler/CMakeFiles/LLVMMipsDisassembler.dir/MipsDisassembler.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonInstPrinter.cpp.o
[ 26%] Building CXX object lib/Target/ARM/MCTargetDesc/CMakeFiles/LLVMARMDesc.dir/ARMELFStreamer.cpp.o
[ 26%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCCodeEmitter.cpp.o
[ 26%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCCodeEmitter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCTargetDesc.cpp.o
[ 26%] Building CXX object lib/Target/SystemZ/MCTargetDesc/CMakeFiles/LLVMSystemZDesc.dir/SystemZMCTargetDesc.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
Scanning dependencies of target LLVMPowerPCDesc
[ 26%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCAsmBackend.cpp.o
[ 26%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsAsmBackend.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCAsmInfo.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCAsmInfo.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCChecker.cpp.o
[ 26%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCTargetDesc.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCCodeEmitter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCCompound.cpp.o
[ 26%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCCompound.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
/checkout/src/llvm-project/llvm/lib/Target/M68k/MCTargetDesc/M68kMCCodeEmitter.cpp:192:8: warning: variable 'NoExpr' set but not used [-Wunused-but-set-variable]
  bool NoExpr = false;
       ^
1 warning generated.
1 warning generated.
[ 26%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCTargetDesc.cpp.o
[ 26%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCInstPrinter.cpp.o
[ 26%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsELFObjectWriter.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 27%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRTargetStreamer.cpp.o
[ 27%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Assumptions.cpp.o
[ 27%] Linking CXX static library ../../../libLLVMSystemZDesc.a
[ 27%] Built target LLVMSystemZDesc
---
[ 27%] Built target LLVMMipsDisassembler
[ 27%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCExpr.cpp.o
Scanning dependencies of target LLVMPowerPCDisassembler
[ 27%] Building CXX object lib/Target/PowerPC/Disassembler/CMakeFiles/LLVMPowerPCDisassembler.dir/PPCDisassembler.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 27%] Building CXX object lib/Target/ARM/MCTargetDesc/CMakeFiles/LLVMARMDesc.dir/ARMMachORelocationInfo.cpp.o
[ 27%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCInstrInfo.cpp.o
[ 27%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsELFStreamer.cpp.o
[ 27%] Linking CXX static library ../../../libLLVMM68kDesc.a
[ 27%] Linking CXX static library ../../../libLLVMM68kDesc.a
Scanning dependencies of target LLVMRISCVInfo
[ 27%] Built target LLVMM68kDesc
[ 27%] Building CXX object lib/Target/PowerPC/MCTargetDesc/CMakeFiles/LLVMPowerPCDesc.dir/PPCMCAsmInfo.cpp.o
[ 27%] Building CXX object lib/Target/RISCV/TargetInfo/CMakeFiles/LLVMRISCVInfo.dir/RISCVTargetInfo.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 27%] Building CXX object lib/Target/ARM/MCTargetDesc/CMakeFiles/LLVMARMDesc.dir/ARMMCAsmInfo.cpp.o
[ 27%] Linking CXX static library ../../../libLLVMAVRDesc.a
[ 27%] Built target LLVMAVRDesc
Scanning dependencies of target LLVMSparcAsmParser
---
[ 27%] Built target LLVMPowerPCDisassembler
[ 27%] Linking CXX static library ../../../libLLVMRISCVInfo.a
[ 27%] Built target LLVMRISCVInfo
[ 27%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCTargetDesc.cpp.o
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[ 27%] Built target AArch64CommonTableGen
Scanning dependencies of target LLVMSystemZDisassembler
[ 27%] Building CXX object lib/Target/SystemZ/AsmParser/CMakeFiles/LLVMSystemZAsmParser.dir/SystemZAsmParser.cpp.o
[ 27%] Building CXX object lib/Target/SystemZ/Disassembler/CMakeFiles/LLVMSystemZDisassembler.dir/SystemZDisassembler.cpp.o
---
[  3%] Building Options.inc...
[  3%] Building Options.inc...
[  4%] Building Options.inc...
[  4%] Generating VCSVersion.inc
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[  4%] Built target WasmOptionsTableGen
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[  4%] Built target MinGWOptionsTableGen
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
LLVM Profile Warning: Unable to track new values: Running out of static counters.  Consider using option -mllvm -vp-counters-per-site=<n> to allocate more value profile counters at compile time. 
[  4%] Built target ELFOptionsTableGen
[  4%] Built target MachOOptionsTableGen
Scanning dependencies of target lldCommon
[  5%] Building CXX object Common/CMakeFiles/lldCommon.dir/Args.cpp.o
---
6 normal benchmarks remaining
Preparing clap-3.1.6
[2022-05-18T21:43:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T21:43:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T21:43:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBDZc7U#clap:3.1.6" "--release" "--" "--skip-this-rustc"
[2022-05-18T21:43:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZYFnba#clap:3.1.6" "--" "--skip-this-rustc"
[2022-05-18T21:43:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T21:43:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T21:43:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFvEx4t#clap:3.1.6" "--" "--wrap-rustc-with" "Eprintln"
Running clap-3.1.6: Opt + [Full]
Running clap-3.1.6: Opt + [Full]
[2022-05-18T21:43:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T21:43:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T21:43:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDZpUGk#clap:3.1.6" "--release" "--" "--wrap-rustc-with" "Eprintln"
Preparing hyper-0.14.18
[2022-05-18T21:44:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T21:44:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T21:44:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkdZ326#hyper:0.14.18" "--features" "client,http1,http2,server,stream" "--" "--skip-this-rustc"
---
3 normal benchmarks remaining
Preparing ripgrep-13.0.0
[2022-05-18T21:45:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T21:45:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T21:45:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEv3vVT#ripgrep:13.0.0" "--" "--skip-this-rustc"
[2022-05-18T21:45:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCkbfql#ripgrep:13.0.0" "--release" "--" "--skip-this-rustc"
[2022-05-18T21:45:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T21:45:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T21:45:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvNV2E6#ripgrep:13.0.0" "--" "--wrap-rustc-with" "Eprintln"
Running ripgrep-13.0.0: Opt + [Full]
---
1 normal benchmark remaining
Preparing syn-1.0.89
[2022-05-18T21:45:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T21:45:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T21:45:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2b3f9b#syn:1.0.89" "--" "--skip-this-rustc"
[2022-05-18T21:45:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwUcnDI#syn:1.0.89" "--release" "--" "--skip-this-rustc"
[2022-05-18T21:45:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T21:45:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T21:45:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T21:45:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEvsPtN#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-18T21:45:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T21:45:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T21:45:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T21:45:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvJmDTD#syn:1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
error: ./build/x86_64-unknown-linux-gnu/llvm/build/profiles: No such file or directory
