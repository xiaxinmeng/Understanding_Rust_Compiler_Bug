plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d5f9c40e6a9ecc62432e71e886cef83a4c2c9b98 and f153f8ba08e5beb30aba12b61ee31eeaddbb4008
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-- Looking for __register_frame
-- Looking for __register_frame - found
-- Looking for __deregister_frame
-- Looking for __deregister_frame - found
-- Looking for __unw_add_dynamic_fde
-- Looking for __unw_add_dynamic_fde - not found
-- Looking for _Unwind_Backtrace - found
-- Looking for getpagesize
-- Looking for getpagesize - found
-- Looking for sysconf
---
[524/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineVerifier.cpp.o
[525/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRFSDiscriminator.cpp.o
[526/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRSampleProfile.cpp.o
[527/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRYamlMapping.cpp.o
[528/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MLRegallocEvictAdvisor.cpp.o
[530/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MultiHazardRecognizer.cpp.o
[531/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/PatchableFunction.cpp.o
[532/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MBFIWrapper.cpp.o
[533/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRPrinter.cpp.o
---
[550/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RDFRegisters.cpp.o
[551/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ReachingDefAnalysis.cpp.o
[552/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocBase.cpp.o
[553/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocBasic.cpp.o
[554/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocEvictionAdvisor.cpp.o
[556/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocGreedy.cpp.o
[557/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocPBQP.cpp.o
[558/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocScore.cpp.o
[559/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterClassInfo.cpp.o
---
[604/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TypePromotion.cpp.o
[605/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/UnreachableBlockElim.cpp.o
[606/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TwoAddressInstructionPass.cpp.o
[607/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ValueTypes.cpp.o
[608/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/VLIWMachineScheduler.cpp.o
[610/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/WasmEHPrepare.cpp.o
[611/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/WinEHPrepare.cpp.o
[612/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/XRayInstrumentation.cpp.o
[613/2917] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LiveDebugValues/LiveDebugValues.cpp.o
---
[1050/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MemoryDependenceAnalysis.cpp.o
[1051/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MemorySSA.cpp.o
[1052/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MemorySSAUpdater.cpp.o
[1053/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModuleDebugInfoPrinter.cpp.o
[1054/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModelUnderTrainingRunner.cpp.o
[1056/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MustExecute.cpp.o
[1056/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MustExecute.cpp.o
[1057/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/NoInferenceModelRunner.cpp.o
[1059/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ObjCARCAnalysisUtils.cpp.o
[1060/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ObjCARCInstKind.cpp.o
[1061/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/OverflowInstAnalysis.cpp.o
[1062/2917] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/OptimizationRemarkEmitter.cpp.o
---
[1213/2917] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStreamer.cpp.o
[1214/2917] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStringTable.cpp.o
[1215/2917] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkParser.cpp.o
[1216/2917] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkSerializer.cpp.o
[1217/2917] Building CXX object lib/Debuginfod/CMakeFiles/LLVMDebuginfod.dir/Debuginfod.cpp.o
[1218/2917] Building CXX object lib/Debuginfod/CMakeFiles/LLVMDebuginfod.dir/HTTPClient.cpp.o
[1220/2917] Linking CXX static library lib/libLLVMOption.a
[1221/2917] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAddressRange.cpp.o
[1222/2917] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAcceleratorTable.cpp.o
[1223/2917] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFCompileUnit.cpp.o
---
[1438/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/DebugObjectManagerPlugin.cpp.o
[1439/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/DebugUtils.cpp.o
[1440/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/DebuggerSupportPlugin.cpp.o
[1441/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCDynamicLibrarySearchGenerator.cpp.o
[1442/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCGenericJITLinkMemoryManager.cpp.o
[1444/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCEHFrameRegistrar.cpp.o
[1444/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCEHFrameRegistrar.cpp.o
[1445/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCGenericDylibManager.cpp.o
[1446/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/EPCGenericRTDyldMemoryManager.cpp.o
[1448/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/ExecutionUtils.cpp.o
[1449/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/ObjectFileInterface.cpp.o
[1450/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IndirectionUtils.cpp.o
[1451/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IRCompileLayer.cpp.o
[1451/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IRCompileLayer.cpp.o
[1452/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/Layer.cpp.o
[1453/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IRTransformLayer.cpp.o
[1454/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/LazyReexports.cpp.o
[1455/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/JITTargetMachineBuilder.cpp.o
[1456/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/LLJIT.cpp.o
[1457/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/LookupAndRecordAddrs.cpp.o
[1459/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/Mangling.cpp.o
[1459/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/Mangling.cpp.o
[1460/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/ELFNixPlatform.cpp.o
[1462/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/OrcABISupport.cpp.o
[1463/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/ObjectTransformLayer.cpp.o
[1464/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/OrcV2CBindings.cpp.o
[1465/2917] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/Speculation.cpp.o
---
[1475/2917] Building CXX object lib/ExecutionEngine/Orc/Shared/CMakeFiles/LLVMOrcShared.dir/SimpleRemoteEPCUtils.cpp.o
[1476/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/JITLoaderGDB.cpp.o
[1477/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/OrcRTBootstrap.cpp.o
[1478/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/RegisterEHFrames.cpp.o
[1479/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/SimpleExecutorDylibManager.cpp.o
[1480/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/SimpleExecutorMemoryManager.cpp.o
/checkout/src/llvm-project/llvm/lib/ExecutionEngine/Orc/Shared/SimpleRemoteEPCUtils.cpp: In member function 'void llvm::orc::FDSimpleRemoteEPCTransport::listenLoop()':
/checkout/src/llvm-project/llvm/lib/ExecutionEngine/Orc/Shared/SimpleRemoteEPCUtils.cpp:209:78: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
         *((support::ulittle64_t *)(HeaderBuffer + FDMsgHeader::MsgSizeOffset));
[1481/2917] Linking CXX static library lib/libLLVMOrcShared.a
[1481/2917] Linking CXX static library lib/libLLVMOrcShared.a
[1482/2917] Building CXX object lib/ExecutionEngine/Orc/TargetProcess/CMakeFiles/LLVMOrcTargetProcess.dir/SimpleRemoteEPCServer.cpp.o
[1484/2917] Building CXX object lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/JITSymbol.cpp.o
[1485/2917] Building CXX object lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RTDyldMemoryManager.cpp.o
[1486/2917] Building CXX object lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyld.cpp.o
[1487/2917] Building CXX object lib/ExecutionEngine/RuntimeDyld/CMakeFiles/LLVMRuntimeDyld.dir/RuntimeDyldChecker.cpp.o
---
[1840/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFAsmPrinter.cpp.o
[1841/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFCheckAndAdjustIR.cpp.o
[1842/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFFrameLowering.cpp.o
[1843/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFInstrInfo.cpp.o
[1844/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFIRPeephole.cpp.o
[1846/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFISelLowering.cpp.o
[1847/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFMCInstLower.cpp.o
[1848/2917] Linking CXX static library lib/libLLVMARMInfo.a
[1849/2917] Building CXX object lib/Target/BPF/CMakeFiles/LLVMBPFCodeGen.dir/BPFPreserveDIType.cpp.o
---
[2005/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsSEISelLowering.cpp.o
[2006/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsSERegisterInfo.cpp.o
[2007/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsSubtarget.cpp.o
[2008/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsTargetMachine.cpp.o
[2009/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsMulMulBugPass.cpp.o
[2011/2917] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MicroMipsSizeReduction.cpp.o
[2012/2917] Building CXX object lib/Target/Mips/AsmParser/CMakeFiles/LLVMMipsAsmParser.dir/MipsAsmParser.cpp.o
[2013/2917] Building CXX object lib/Target/Mips/Disassembler/CMakeFiles/LLVMMipsDisassembler.dir/MipsDisassembler.cpp.o
[2014/2917] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsABIInfo.cpp.o
---
[2111/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVMergeBaseOffset.cpp.o
[2112/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVSubtarget.cpp.o
[2113/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVRegisterBankInfo.cpp.o
[2114/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVRegisterInfo.cpp.o
[2115/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVSExtWRemoval.cpp.o
[2117/2917] Linking CXX static library lib/libLLVMPowerPCDesc.a
[2118/2917] Linking CXX static library lib/libLLVMPowerPCAsmParser.a
[2119/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVTargetObjectFile.cpp.o
[2120/2917] Building CXX object lib/Target/RISCV/CMakeFiles/LLVMRISCVCodeGen.dir/RISCVTargetTransformInfo.cpp.o
---
[2275/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ExpandPseudo.cpp.o
[2276/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FastISel.cpp.o
[2277/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupBWInsts.cpp.o
[2278/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupLEAs.cpp.o
[2279/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86DynAllocaExpander.cpp.o
[2281/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupSetCC.cpp.o
[2282/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FlagsCopyLowering.cpp.o
[2283/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FloatingPoint.cpp.o
[2284/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FrameLowering.cpp.o
---
[2318/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86WinEHState.cpp.o
[2319/2917] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InsertWait.cpp.o
[2320/2917] Building CXX object lib/Target/X86/AsmParser/CMakeFiles/LLVMX86AsmParser.dir/X86AsmParser.cpp.o
[2321/2917] Building CXX object lib/Target/X86/Disassembler/CMakeFiles/LLVMX86Disassembler.dir/X86Disassembler.cpp.o
[2322/2917] Building CXX object lib/Target/X86/MCA/CMakeFiles/LLVMX86TargetMCA.dir/X86CustomBehaviour.cpp.o
[2324/2917] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86IntelInstPrinter.cpp.o
[2325/2917] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86InstComments.cpp.o
[2326/2917] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86InstPrinterCommon.cpp.o
[2327/2917] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86ShuffleDecode.cpp.o
---
[2351/2917] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRTargetMachine.cpp.o
[2352/2917] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRTargetObjectFile.cpp.o
[2353/2917] Linking CXX static library lib/libLLVMX86Desc.a
[2354/2917] Linking CXX static library lib/libLLVMX86AsmParser.a
[2355/2917] Linking CXX static library lib/libLLVMX86TargetMCA.a
[2357/2917] Building CXX object lib/Target/AVR/Disassembler/CMakeFiles/LLVMAVRDisassembler.dir/AVRDisassembler.cpp.o
[2358/2917] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRAsmBackend.cpp.o
[2358/2917] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRAsmBackend.cpp.o
FAILED: sccache /usr/bin/c++  -DGTEST_HAS_RTTI=0 -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -Ilib/Target/AVR -I/checkout/src/llvm-project/llvm/lib/Target/AVR -Iinclude -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG -fvisibility=hidden    -fno-exceptions -fno-rtti -UNDEBUG -std=c++14 -MD -MT lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRFrameLowering.cpp.o -MF lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRFrameLowering.cpp.o.d -o lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRFrameLowering.cpp.o -c /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp: In function 'void llvm::fixStackStores(llvm::MachineBasicBlock&, llvm::MachineBasicBlock::iterator, const llvm::TargetInstrInfo&, llvm::Register)':
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp:305:65: error: no matching function for call to 'make_range(llvm::MachineInstr&, llvm::MachineBasicBlock::iterator)'
        llvm::make_early_inc_range(llvm::make_range(MI, MBB.end()))) {
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/GraphTraits.h:22:0,
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/MachineBasicBlock.h:16,
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/TargetFrameLowering.h:16,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.h:12,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.h:12,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp:13:
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:53:38: note: candidate: template<class T> llvm::iterator_range<T> llvm::make_range(T, T)
 template <class T> iterator_range<T> make_range(T x, T y) {
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:53:38: note:   template argument deduction/substitution failed:
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:53:38: note:   template argument deduction/substitution failed:
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp:305:65: note:   deduced conflicting types for parameter 'T' ('llvm::MachineInstr' and 'llvm::MachineInstrBundleIterator<llvm::MachineInstr>')
        llvm::make_early_inc_range(llvm::make_range(MI, MBB.end()))) {
In file included from /checkout/src/llvm-project/llvm/include/llvm/ADT/GraphTraits.h:22:0,
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/MachineBasicBlock.h:16,
                 from /checkout/src/llvm-project/llvm/include/llvm/CodeGen/TargetFrameLowering.h:16,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.h:12,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.h:12,
                 from /checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp:13:
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:57:41: note: candidate: template<class T> llvm::iterator_range<T> llvm::make_range(std::pair<T, T>)
 template <typename T> iterator_range<T> make_range(std::pair<T, T> p) {
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:57:41: note:   template argument deduction/substitution failed:
/checkout/src/llvm-project/llvm/include/llvm/ADT/iterator_range.h:57:41: note:   template argument deduction/substitution failed:
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRFrameLowering.cpp:305:65: note:   'llvm::MachineInstr' is not derived from 'std::pair<T, T>'
        llvm::make_early_inc_range(llvm::make_range(MI, MBB.end()))) {
                                                                 ^
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit status: 1
 finished in 696.181 seconds

Build completed unsuccessfully in 0:12:32
Build completed unsuccessfully in 0:12:32
build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
cat: /tmp/toolstate/toolstates.json: No such file or directory
