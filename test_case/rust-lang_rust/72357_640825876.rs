plain
##[section]Starting: Linux x86_64-gnu-distcheck
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 5'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0c339662-b1de-443a-9345-b31316f91ab3.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72357/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72357/merge:refs/remotes/pull/72357/merge
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackend.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGLexer.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[ 10%] Linking CXX static library ../../../lib/libLLVMTableGenGlobalISel.a
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 10%] Built target LLVMTableGenGlobalISel
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
---
[ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
[ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordHelpers.cpp.o
[ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordMapping.cpp.o
[ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
/checkout/src/llvm-project/llvm/lib/DebugInfo/CodeView/TypeStreamMerger.cpp: In member function 'llvm::ArrayRef<unsigned char> {anonymous}::TypeStreamMerger::remapIndices(const CVType&, llvm::MutableArrayRef<unsigned char>)':
/checkout/src/llvm-project/llvm/lib/DebugInfo/CodeView/TypeStreamMerger.cpp:392:12: warning: unused variable 'AlignedSize' [-Wunused-variable]
   unsigned AlignedSize = alignTo(OriginalType.RecordData.size(), 4);
[ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeTableCollection.cpp.o
[ 15%] Linking CXX static library ../../libLLVMDebugInfoCodeView.a
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 15%] Built target LLVMDebugInfoCodeView
---
[ 17%] Building HexagonGenCallingConv.inc...
[ 17%] Building HexagonGenDAGISel.inc...
[ 17%] Building AArch64GenGlobalISel.inc...
[ 17%] Building HexagonGenDFAPacketizer.inc...
[ 17%] Building AArch64GenGICombiner.inc...
[ 17%] Building AArch64GenInstrInfo.inc...
[ 17%] Building HexagonGenInstrInfo.inc...
[ 17%] Building HexagonGenMCCodeEmitter.inc...
[ 17%] Building HexagonGenRegisterInfo.inc...
---
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticHandler.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticInfo.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticPrinter.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/FPEnv.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/GVMaterializer.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRBuilder.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRPrintingPasses.cpp.o
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target LLVMCFGuard
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 26%] Building CXX object lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 26%] Built target LLVMAsmParser
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target LLVMMCDisassembler
---
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/DispatchStage.cpp.o
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/EntryStage.cpp.o
[ 28%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.o
[ 28%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/ExecuteStage.cpp.o
[ 28%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.o
[ 28%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
[ 28%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/MicroOpQueueStage.cpp.o
[ 28%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.o
[ 28%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/RetireStage.cpp.o
---
[ 30%] Linking CXX static library ../../../libLLVMX86Info.a
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 30%] Built target LLVMX86Info
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target InstallNameToolOptsTableGen
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 30%] Building InstallNameToolOpts.inc...
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 30%] Built target InstallNameToolOptsTableGen
Scanning dependencies of target LLVMBitReader
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 30%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeAnalyzer.cpp.o
---
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolSize.cpp.o
[ 36%] Building CXX object lib/Target/SystemZ/AsmParser/CMakeFiles/LLVMSystemZAsmParser.dir/SystemZAsmParser.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiFile.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 36%] Built target LLVMSystemZAsmParser
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target LLVMSystemZDisassembler
Scanning dependencies of target LLVMSystemZDisassembler
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 36%] Building CXX object lib/Target/SystemZ/Disassembler/CMakeFiles/LLVMSystemZDisassembler.dir/SystemZDisassembler.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiUniversal.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 36%] Built target LLVMSystemZDisassembler
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WasmObjectFile.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WindowsMachineFlag.cpp.o
---
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumModules.cpp.o
[ 44%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VectorUtils.cpp.o
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumTypes.cpp.o
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeExeSymbol.cpp.o
[ 44%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VFABIDemangling.cpp.o
[ 44%] Linking CXX static library ../libLLVMAnalysis.a
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeSymbolEnumerator.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 44%] Built target LLVMAnalysis
---
[ 49%] Linking CXX static library ../../../lib/libLLVMCFIVerify.a
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ImportedFunctionsInliningStatistics.cpp.o
[ 49%] Built target LLVMCFIVerify
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InjectTLIMappings.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/IntegerDivision.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LCSSA.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LibCallsShrinkWrap.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Local.cpp.o
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target LLVMFrontendOpenMP
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 51%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPConstants.cpp.o
[ 51%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.o
[ 51%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPIRBuilder.cpp.o
[ 51%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAndOrXor.cpp.o
[ 51%] Linking CXX static library ../../libLLVMFrontendOpenMP.a
[ 51%] Built target LLVMFrontendOpenMP
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.o
Scanning dependencies of target LLVMInstrumentation
---
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DFAPacketizer.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DwarfEHPrepare.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalOpt.cpp.o
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp: In function 'llvm::GlobalVariable* SRAGlobal(llvm::GlobalVariable*, const llvm::DataLayout&)':
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp:548:32: warning: unused variable 'STy' [-Wunused-variable]
     } else if (SequentialType *STy = dyn_cast<SequentialType>(Ty)) {
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EarlyIfConversion.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EdgeBundles.cpp.o
---
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterCoalescer.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterPressure.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterScavenging.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RenameIndependentSubregs.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRVRegNamerUtils.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRNamerPass.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterUsageInfo.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegUsageInfoCollector.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegUsageInfoPropagate.cpp.o
[ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ResetMachineFunctionPass.cpp.o
---
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIParser.cpp.o
Scanning dependencies of target LLVMDWARFLinker
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerCompileUnit.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerDeclContext.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIRParser.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinker.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 67%] Built target LLVMMIRParser
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target LLVMPasses
---
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCExpandISEL.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZElimCompare.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZFrameLowering.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCLowerMASSVEntries.cpp.o
[ 73%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelDAGToDAG.cpp.o
[ 73%] Built target LLVMPowerPCCodeGen
---
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterInfo.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMOptimizeBarriersPass.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterBankInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTaggingPreRA.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMSubtarget.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetMachine.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetMachine.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64TargetMachine.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetObjectFile.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64TargetObjectFile.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetTransformInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64TargetTransformInfo.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MLxExpansionPass.cpp.o
[ 83%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SIMDInstrOpt.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEGatherScatterLowering.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 83%] Built target LLVMAArch64CodeGen
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVETailPredication.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEVPTBlockPass.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16HardFloatInfo.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16InstrInfo.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
---
[ 88%] Built target LLVMExegesisMips
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InstrInfo.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86EvexToVex.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86LegalizerInfo.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86LoadValueInjectionLoadHardening.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86LoadValueInjectionRetHardening.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86MachineFunctionInfo.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86MacroFusion.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86OptimizeLEAs.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86PadShortFunction.cpp.o
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-reduce
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 90%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-rc
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
[ 91%] Built target llvm-rc
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-undname
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalVars.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-undname
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceMetadata.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
[ 91%] Built target llvm-undname
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-pdbutil
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DumpOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructions.cpp.o
[ 92%] Linking CXX executable ../../bin/llvm-reduce
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/InputFile.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/llvm-pdbutil.cpp.o
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-ifs
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 95%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ifs
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[ 96%] Built target lli
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
---
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-locstats
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
[100%] Copying llvm-locstats into /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/./bin
[100%] Built target llvm-locstats
make[3]: Entering directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Scanning dependencies of target llvm-dlltool
make[3]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
---
make[2]: Leaving directory '/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build'
Install the project...
-- Install configuration: "Release"
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPIRBuilder.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPConstants.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPKinds.def
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/SupportHelpers.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Annotations.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Error.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LineEditor/LineEditor.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Wasm.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/WindowsMachineFlag.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/TapiUniversal.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELFObjectFile.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/RelocationResolver.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ArchiveWriter.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolicFile.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/XCOFFObjectFile.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Archive.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Minidump.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/MachOUniversal.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/TapiFile.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/StackMapParser.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolSize.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Error.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/IRObjectFile.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/SymbolizableModule.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LookupResult.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineEntry.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymReader.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymCreator.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineTable.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Header.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/FileEntry.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/InlineInfo.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.td
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfoFlags.def
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.def
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/FixedMetadataKinds.def
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsX86.td
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalIndirectSymbol.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassManager.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugLoc.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DataLayout.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PassManagerInternal.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsMips.td
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/FPEnv.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalVariable.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LLVMContext.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalObject.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SymbolRewriter.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Cloning.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ASanStackFrameLayout.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/NameAnonGlobals.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/InjectTLIMappings.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterBulk.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopSimplify.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/PromoteMemToReg.h
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CodeMoverUtils.h
---
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBinaryFormat.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitReader.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitWriter.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitstreamReader.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDWARFLinker.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMFrontendOpenMP.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstrumentation.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAggressiveInstCombine.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstCombine.a
-- Installing: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMScalarOpts.a
---
   Compiling rustc_parse_format v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_parse_format)
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/obj/build/tmp/distcheck/src/librustc_ast_lowering)
---
............................i...............i....................................................... 5200/10287
.................................................................................................... 5300/10287
............................................................................i....................... 5400/10287
.................................................................................................... 5500/10287
.......................................................................................ii.ii........ 5600/10287
i...i............................................................................................... 5700/10287
......................................i............................................................. 5900/10287
............................................................................................ii...... 6000/10287
...............................i.................................................................... 6100/10287
.................................................................................................... 6200/10287
.................................................................................................... 6200/10287
.................................................................................................... 6300/10287
......................................................ii...i..ii...........i........................ 6400/10287
.................................................................................................... 6600/10287
.................................................................................................... 6700/10287
.................................................................................................... 6700/10287
.......................................................................................i..ii........ 6800/10287
.................................................................................................... 7000/10287
.................................................................................................... 7100/10287
.........................................i.......................................................... 7200/10287
.................................................................................................... 7300/10287
---
.................................................................................................... 8200/10287
.................................................................................................... 8300/10287
................................................................................i................... 8400/10287
.................................................................................................... 8500/10287
..................................iiiiii.iiiiii.i................................................... 8600/10287
.................................................................................................... 8800/10287
.................................................................................................... 8900/10287
.................................................................................................... 9000/10287
.................................................................................................... 9100/10287
---
 finished in 8.989
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 193 tests
..i.......................i.i..........i..................................i..i.....................i 100/193
.............i.i.i....ii..iiiiiiiiiiiiiiiii.........................i........................

 finished in 5.696
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

---
 finished in 1.870
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
...........iiiiiiiii.

 finished in 0.565
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

---
 finished in 20.472
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiii..i.....i..i...i..i.i.iF.i.ii.Fii....i.i....ii..........iiii.........i...Fii.FFi.......ii.i.ii. 100/116
....iiii.....ii.

---- [debuginfo-gdb] debuginfo/empty-string.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
NOTE: compiletest thinks it is using GDB version 7011001
SCRIPT
set charset UTF-8
show version
add-auto-load-safe-path /checkout/obj/build/tmp/distcheck/./src/etc
set print pretty off
directory /checkout/obj/build/tmp/distcheck/./src/etc
file /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string.gdb/a
break 'empty-string.rs':33
 print empty_string
 print empty_str
quit




error: line not found in debugger output: $1 = ""
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/empty-string.gdb/empty-string.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0xbcf: file /checkout/obj/build/tmp/distcheck/src/test/debuginfo/empty-string.rs, line 33.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, empty_string::main::hde31c8bc9c2a5d54 () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/empty-string.rs:33
33     zzz(); // #break
$1 = {vec = {buf = {ptr = {pointer = 0x1 <error: Cannot access memory at address 0x1>, _marker = {<No data fields>}}, cap = 0, alloc = {<No data fields>}}, len = 0}}
$2 = ""


 Inferior 1 [process 31903] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------

---
NOTE: compiletest thinks it is using GDB version 7011001
SCRIPT
set charset UTF-8
show version
add-auto-load-safe-path /checkout/obj/build/tmp/distcheck/./src/etc
set print pretty off
directory /checkout/obj/build/tmp/distcheck/./src/etc
file /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums.gdb/a
break 'gdb-pretty-struct-and-enums.rs':60
 print regular_struct
 print empty_struct
 print c_style_enum1
 print c_style_enum2
 print c_style_enum2
 print c_style_enum3
quit



error: line not found in debugger output: $2 = EmptyStruct
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums.gdb/gdb-pretty-struct-and-enums.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0xa1f: file /checkout/obj/build/tmp/distcheck/src/test/debuginfo/gdb-pretty-struct-and-enums.rs, line 60.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, gdb_pretty_struct_and_enums::main::h880114516c795488 () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/gdb-pretty-struct-and-enums.rs:60
60     zzz(); // #break
$1 = {the_first_field = 101, the_second_field = 102.5, the_third_field = false}
$2 = {<No data fields>}
$3 = CStyleEnumVar1
$4 = CStyleEnumVar2
$5 = CStyleEnumVar3


 Inferior 1 [process 31980] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------

---
NOTE: compiletest thinks it is using GDB version 7011001
SCRIPT
set charset UTF-8
show version
add-auto-load-safe-path /checkout/obj/build/tmp/distcheck/./src/etc
set print pretty off
directory /checkout/obj/build/tmp/distcheck/./src/etc
file /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec.gdb/a
break 'pretty-huge-vec.rs':27
 print vec
 print slice
quit




error: line not found in debugger output: $1 = Vec(size=1000000000) = {[...]...}
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-huge-vec.gdb/pretty-huge-vec.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1a5d: file /checkout/obj/build/tmp/distcheck/src/test/debuginfo/pretty-huge-vec.rs, line 27.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_huge_vec::main::h5dfd4c3cb6e3db0d () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/pretty-huge-vec.rs:27
27     zzz(); // #break
$1 = {buf = {ptr = {pointer = 0x7fffbb520010 "", _marker = {<No data fields>}}, cap = 1000000000, alloc = {<No data fields>}}, len = 1000000000}
$2 = {data_ptr = 0x7fffbb520010 "", length = 1000000000}


 Inferior 1 [process 32672] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------

---
NOTE: compiletest thinks it is using GDB version 7011001
SCRIPT
set charset UTF-8
show version
add-auto-load-safe-path /checkout/obj/build/tmp/distcheck/./src/etc
set print pretty off
directory /checkout/obj/build/tmp/distcheck/./src/etc
file /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec.gdb/a
break 'pretty-uninitialized-vec.rs':21
 print vec
quit




error: line not found in debugger output: $1 = Vec(size=[...])[...]
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-uninitialized-vec.gdb/pretty-uninitialized-vec.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x1a14: file /checkout/obj/build/tmp/distcheck/src/test/debuginfo/pretty-uninitialized-vec.rs, line 21.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, pretty_uninitialized_vec::main::h512c42bd101a2311 () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/pretty-uninitialized-vec.rs:21
21     zzz(); // #break
$1 = {buf = {ptr = {pointer = 0x7fffff7fe000, _marker = {<No data fields>}}, cap = 140737479962624, alloc = {<No data fields>}}, len = 93824994345120}


 Inferior 1 [process 32719] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [debuginfo-gdb] debuginfo/rc_arc.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support
NOTE: compiletest thinks it is using GDB version 7011001
set charset UTF-8
show version
show version
add-auto-load-safe-path /checkout/obj/build/tmp/distcheck/./src/etc
set print pretty off
directory /checkout/obj/build/tmp/distcheck/./src/etc
file /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/rc_arc.gdb/a
break 'rc_arc.rs':34
print r
print a
quit




error: line not found in debugger output: [...]$1 = Rc(strong=2, weak=1) = {value = 42, strong = 2, weak = 1}
status: exit code: 0
command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo/rc_arc.gdb/rc_arc.debugger.script"
------------------------------------------
GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
Copyright (C) 2016 Free Software Foundation, Inc.
Copyright (C) 2016 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
Breakpoint 1 at 0x327e: file /checkout/obj/build/tmp/distcheck/src/test/debuginfo/rc_arc.rs, line 34.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, rc_arc::main::h8b797dc2ea1f4c51 () at /checkout/obj/build/tmp/distcheck/src/test/debuginfo/rc_arc.rs:34
34     print!(""); // #break
$1 = {ptr = {pointer = 0x55555575a130}, phantom = {<No data fields>}}
$2 = {ptr = {pointer = 0x55555575a150}, phantom = {<No data fields>}}


 Inferior 1 [process 32755] will be killed.

Quit anyway? (y or n) [answered Y; input not from terminal]
------------------------------------------
stderr:
------------------------------------------


------------------------------------------



failures:
    [debuginfo-gdb] debuginfo/empty-string.rs
    [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs
    [debuginfo-gdb] debuginfo/pretty-huge-vec.rs
    [debuginfo-gdb] debuginfo/pretty-uninitialized-vec.rs
    [debuginfo-gdb] debuginfo/rc_arc.rs
test result: FAILED. 71 passed; 5 failed; 40 ignored; 0 measured; 0 filtered out




command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/obj/build/tmp/distcheck/src/test/debuginfo" "--build-base" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.1-rust-1.46.0-nightly" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
  local time: Mon Jun  8 19:22:32 UTC 2020
  network time: Mon, 08 Jun 2020 19:22:32 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72357/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72357/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3776) (python)
##[section]Finishing: Finalize Job
