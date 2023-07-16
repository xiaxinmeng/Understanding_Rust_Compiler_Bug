plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e5484cec0e3f60b2a0010505cd355639f75a2973 and d108b0f769786f7916152bf364dcef30f43ab4cd
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 1m 07s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.56.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 5.4.0
-- The ASM compiler identification is GNU
-- Found assembler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/cc
---
[1671/2846] Building AVRGenRegisterInfo.inc...
[1672/2846] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ISelDAGToDAG.cpp.o
[1673/2846] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ISelLowering.cpp.o
[1674/2846] Building AVRGenSubtargetInfo.inc...
[1675/2846] Building M68kGenMCPseudoLowering.inc...
[1676/2846] Building M68kGenMCCodeBeads.inc...
[1678/2846] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64LowerHomogeneousPrologEpilog.cpp.o
[1679/2846] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64LoadStoreOptimizer.cpp.o
[1680/2846] Building M68kGenInstrInfo.inc...
[1681/2846] Building M68kGenSubtargetInfo.inc...
---
[2314/2846] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCExpr.cpp.o
[2315/2846] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCTargetDesc.cpp.o
[2316/2846] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRTargetStreamer.cpp.o
[2317/2846] Building CXX object lib/Target/AVR/TargetInfo/CMakeFiles/LLVMAVRInfo.dir/AVRTargetInfo.cpp.o
[2318/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GlSel/M68kCallLowering.cpp.o
[2319/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GlSel/M68kInstructionSelector.cpp.o
[2320/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GlSel/M68kLegalizerInfo.cpp.o
[2321/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kAsmPrinter.cpp.o
[2322/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/GlSel/M68kRegisterBankInfo.cpp.o
[2323/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kExpandPseudo.cpp.o
[2324/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kCollapseMOVEMPass.cpp.o
[2325/2846] Linking CXX static library lib/libLLVMX86Desc.a
[2326/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kFrameLowering.cpp.o
[2327/2846] Linking CXX static library lib/libLLVMX86AsmParser.a
[2328/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kInstrInfo.cpp.o
[2329/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelLowering.cpp.o
[2330/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelDAGToDAG.cpp.o
[2331/2846] Linking CXX static library lib/libLLVMAVRInfo.a
[2332/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMCInstLower.cpp.o
[2333/2846] Linking CXX static library lib/libLLVMAVRDisassembler.a
[2334/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMachineFunction.cpp.o
[2335/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kRegisterInfo.cpp.o
[2336/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kSubtarget.cpp.o
[2338/2846] Linking CXX static library lib/libLLVMAVRAsmParser.a
[2338/2846] Linking CXX static library lib/libLLVMAVRAsmParser.a
[2339/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetMachine.cpp.o
[2340/2846] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kTargetObjectFile.cpp.o
[2341/2846] Building CXX object lib/Target/M68k/TargetInfo/CMakeFiles/LLVMM68kInfo.dir/M68kTargetInfo.cpp.o
[2342/2846] Linking CXX static library lib/libLLVMM68kInfo.a
[2343/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kAsmBackend.cpp.o
[2344/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kELFObjectWriter.cpp.o
[2345/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kInstPrinter.cpp.o
[2346/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCAsmInfo.cpp.o
[2347/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCCodeEmitter.cpp.o
[2348/2846] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCTargetDesc.cpp.o
[2349/2846] Building CXX object lib/Target/M68k/AsmParser/CMakeFiles/LLVMM68kAsmParser.dir/M68kAsmParser.cpp.o
[2351/2846] Building CXX object tools/llvm-gsymutil/CMakeFiles/llvm-gsymutil.dir/llvm-gsymutil.cpp.o
[2351/2846] Building CXX object tools/llvm-gsymutil/CMakeFiles/llvm-gsymutil.dir/llvm-gsymutil.cpp.o
[2352/2846] Building CXX object lib/Target/M68k/Disassembler/CMakeFiles/LLVMM68kDisassembler.dir/M68kDisassembler.cpp.o
[2354/2846] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfWriter.cpp.o
[2355/2846] Building CXX object lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingWriter.cpp.o
[2356/2846] Building CXX object lib/ProfileData/Coverage/CMakeFiles/LLVMCoverage.dir/CoverageMappingReader.cpp.o
[2357/2846] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/InterfaceFile.cpp.o
---
[2434/2846] Linking CXX static library lib/libLLVMMIRParser.a
[2435/2846] Linking CXX static library lib/libLLVMCoroutines.a
[2436/2846] Linking CXX static library lib/libLLVMInterpreter.a
[2437/2846] Linking CXX static library lib/libLLVMWebAssemblyUtils.a
[2438/2846] Linking CXX static library lib/libLLVMM68kDesc.a
[2440/2846] Linking CXX static library lib/libLLVMWebAssemblyAsmParser.a
[2441/2846] Linking CXX static library lib/libLLVMWebAssemblyDesc.a
[2442/2846] Linking CXX executable bin/FileCheck
[2443/2846] Linking C executable bin/count
---
[2486/2846] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/BinaryHolder.cpp.o
[2487/2846] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachODebugMapParser.cpp.o
[2488/2846] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachOUtils.cpp.o
[2489/2846] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/Reproducer.cpp.o
FAILED: sccache /usr/bin/c++  -DGTEST_HAS_RTTI=0 -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib/Target/M68k/AsmParser -I/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser -I/checkout/src/llvm-project/llvm/lib/Target/M68k -Ilib/Target/M68k -Iinclude -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG -fvisibility=hidden    -fno-exceptions -fno-rtti -UNDEBUG -std=c++14 -MD -MT lib/Target/M68k/AsmParser/CMakeFiles/LLVMM68kAsmParser.dir/M68kAsmParser.cpp.o -MF lib/Target/M68k/AsmParser/CMakeFiles/LLVMM68kAsmParser.dir/M68kAsmParser.cpp.o.d -o lib/Target/M68k/AsmParser/CMakeFiles/LLVMM68kAsmParser.dir/M68kAsmParser.cpp.o -c /checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In member function 'bool {anonymous}::M68kOperand::isMemOp() const':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:146:41: error: 'Kind' is not a class, namespace, or enumeration
   bool isMemOp() const { return Kind == Kind::MemOp; }
                                         ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In member function 'virtual bool {anonymous}::M68kOperand::isReg() const':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:251:18: error: 'Kind' is not a class, namespace, or enumeration
   return Kind == Kind::MemOp && MemOp.Op == M68kMemOp::Kind::Reg;
                  ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In static member function 'static std::unique_ptr<{anonymous}::M68kOperand> {anonymous}::M68kOperand::createMemOp({anonymous}::M68kMemOp, llvm::SMLoc, llvm::SMLoc)':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:268:43: error: 'Kind' is not a class, namespace, or enumeration
   auto Op = std::make_unique<M68kOperand>(Kind::MemOp, Start, End);
                                           ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In member function 'virtual bool {anonymous}::M68kOperand::isToken() const':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:274:52: error: 'Kind' is not a class, namespace, or enumeration
 bool M68kOperand::isToken() const { return Kind == Kind::Token; }
                                                    ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In static member function 'static std::unique_ptr<{anonymous}::M68kOperand> {anonymous}::M68kOperand::createToken(llvm::StringRef, llvm::SMLoc, llvm::SMLoc)':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:282:43: error: 'Kind' is not a class, namespace, or enumeration
   auto Op = std::make_unique<M68kOperand>(Kind::Token, Start, End);
                                           ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In member function 'virtual bool {anonymous}::M68kOperand::isImm() const':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:288:50: error: 'Kind' is not a class, namespace, or enumeration
 bool M68kOperand::isImm() const { return Kind == Kind::Imm; }
                                                  ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In static member function 'static std::unique_ptr<{anonymous}::M68kOperand> {anonymous}::M68kOperand::createImm(const llvm::MCExpr*, llvm::SMLoc, llvm::SMLoc)':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:298:43: error: 'Kind' is not a class, namespace, or enumeration
   auto Op = std::make_unique<M68kOperand>(Kind::Imm, Start, End);
                                           ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp: In member function 'virtual void {anonymous}::M68kOperand::print(llvm::raw_ostream&) const':
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:845:8: error: 'Kind' is not a class, namespace, or enumeration
   case Kind::Invalid:
        ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:849:8: error: 'Kind' is not a class, namespace, or enumeration
   case Kind::Token:
        ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:853:8: error: 'Kind' is not a class, namespace, or enumeration
   case Kind::Imm:
        ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:857:8: error: 'Kind' is not a class, namespace, or enumeration
   case Kind::MemOp:
        ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:844:10: warning: enumeration value 'Invalid' not handled in switch [-Wswitch]
   switch (Kind) {
          ^
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:844:10: warning: enumeration value 'Token' not handled in switch [-Wswitch]
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:844:10: warning: enumeration value 'Imm' not handled in switch [-Wswitch]
/checkout/src/llvm-project/llvm/lib/Target/M68k/AsmParser/M68kAsmParser.cpp:844:10: warning: enumeration value 'MemOp' not handled in switch [-Wswitch]
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 158.281 seconds
Build completed unsuccessfully in 0:03:46
cat: /tmp/toolstate/toolstates.json: No such file or directory
