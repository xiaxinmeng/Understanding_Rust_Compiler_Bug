plain
##[section]Starting: Linux dist-aarch64-linux
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/840fb76d-5bea-4b93-80fb-af701cd6bdba.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73058/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73058/merge:refs/remotes/pull/73058/merge
---
 ---> a3599b623c1b
Successfully built a3599b623c1b
Successfully tagged rust-ci:latest
Built container sha256:a3599b623c1bed4d36191bcaa7dfc61dc0fbc03fbd8c6b1b44a984c081cd09e5
Uploading finished image to https://ci-caches.rust-lang.org/docker/ec663993490fb74ca0eed3bed1f1a7c5b29b13b7b793b8912f58dd46a99d20a6b417bf08ed711a2f7bfb25f0a898c4a183e5a62a39c7db65d16b94b9eccaf2dd
upload failed: - to s3://rust-lang-ci-sccache2/docker/ec663993490fb74ca0eed3bed1f1a7c5b29b13b7b793b8912f58dd46a99d20a6b417bf08ed711a2f7bfb25f0a898c4a183e5a62a39c7db65d16b94b9eccaf2dd An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=dist-aarch64-linux]
== clock drift check ==
  local time: Sat Jun  6 12:52:15 UTC 2020
  network time: Sat, 06 Jun 2020 12:52:15 GMT
---
Scanning dependencies of target LLVMTableGen
[  8%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackend.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGLexer.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[  9%] Linking CXX static library ../../../lib/libLLVMTableGenGlobalISel.a
[  9%] Built target LLVMTableGenGlobalISel
Scanning dependencies of target LLVMBitstreamReader
[  9%] Building CXX object lib/Bitstream/Reader/CMakeFiles/LLVMBitstreamReader.dir/BitstreamReader.cpp.o
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
[ 15%] Built target LLVMDebugInfoCodeView
Scanning dependencies of target LibOptionsTableGen
---
[ 17%] Building AArch64GenGlobalISel.inc...
[ 17%] Building HexagonGenAsmWriter.inc...
[ 17%] Building HexagonGenCallingConv.inc...
[ 17%] Building HexagonGenDAGISel.inc...
[ 17%] Building AArch64GenGICombiner.inc...
[ 17%] Building HexagonGenDFAPacketizer.inc...
[ 17%] Building HexagonGenDisassemblerTables.inc...
[ 17%] Building HexagonGenInstrInfo.inc...
[ 17%] Building HexagonGenMCCodeEmitter.inc...
---
[ 22%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodeEmitter.cpp.o
[ 22%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCCodeView.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[ 23%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCContext.cpp.o
[ 23%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/FPEnv.cpp.o
[ 23%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCDwarf.cpp.o
[ 23%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCELFObjectTargetWriter.cpp.o
[ 23%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/GVMaterializer.cpp.o
[ 23%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCELFStreamer.cpp.o
---
Scanning dependencies of target LLVMX86Utils
[ 27%] Building CXX object lib/Target/X86/Utils/CMakeFiles/LLVMX86Utils.dir/X86ShuffleDecode.cpp.o
[ 27%] Linking CXX static library ../../../libLLVMX86Info.a
[ 27%] Built target LLVMX86Info
Scanning dependencies of target InstallNameToolOptsTableGen
[ 27%] Building InstallNameToolOpts.inc...
[ 27%] Built target InstallNameToolOptsTableGen
[ 27%] Built target LLVMX86Utils
Scanning dependencies of target LLVMBitReader
[ 27%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeAnalyzer.cpp.o
Scanning dependencies of target LLVMMCParser
---
[ 28%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfWriter.cpp.o
[ 28%] Linking CXX static library ../libLLVMProfileData.a
[ 28%] Built target LLVMProfileData
[ 28%] Building CXX object lib/AsmParser/CMakeFiles/LLVMAsmParser.dir/Parser.cpp.o
Scanning dependencies of target LLVMCFGuard
[ 28%] Building CXX object lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.o
[ 28%] Built target LLVMAsmParser
Scanning dependencies of target LLVMMCDisassembler
[ 28%] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/Disassembler.cpp.o
[ 28%] Linking CXX static library ../../libLLVMCFGuard.a
---
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/Scheduler.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[ 30%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.o
[ 30%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/DispatchStage.cpp.o
---
[ 35%] Built target LLVMMipsAsmParser
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/RelocationResolver.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolicFile.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolSize.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiFile.cpp.o
Scanning dependencies of target LLVMPowerPCAsmParser
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiUniversal.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WasmObjectFile.cpp.o
[ 35%] Linking CXX static library ../../../libLLVMPowerPCAsmParser.a
[ 35%] Built target LLVMPowerPCAsmParser
Scanning dependencies of target LLVMRISCVAsmParser
---
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VectorUtils.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStreamBuilder.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VFABIDemangling.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[ 46%] Linking CXX static library ../libLLVMAnalysis.a
[ 46%] Built target LLVMAnalysis
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
---
[ 50%] Built target LLVMCFIVerify
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GuardUtils.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InlineFunction.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ImportedFunctionsInliningStatistics.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InjectTLIMappings.cpp.o
Scanning dependencies of target LLVMMCJIT
[ 50%] Building CXX object lib/ExecutionEngine/MCJIT/CMakeFiles/LLVMMCJIT.dir/MCJIT.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/IntegerDivision.cpp.o
[ 50%] Linking CXX static library ../../libLLVMMCJIT.a
---
[ 52%] Building CXX object lib/Transforms/AggressiveInstCombine/CMakeFiles/LLVMAggressiveInstCombine.dir/TruncInstCombine.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAddSub.cpp.o
[ 52%] Linking CXX static library ../../libLLVMAggressiveInstCombine.a
[ 52%] Built target LLVMAggressiveInstCombine
Scanning dependencies of target LLVMFrontendOpenMP
[ 52%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPConstants.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.o
[ 52%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPIRBuilder.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAndOrXor.cpp.o
[ 53%] Linking CXX static library ../../libLLVMFrontendOpenMP.a
[ 53%] Built target LLVMFrontendOpenMP
Scanning dependencies of target LLVMInstrumentation
[ 53%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/AddressSanitizer.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.o
[ 53%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/BoundsChecking.cpp.o
---
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DwarfEHPrepare.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalOpt.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EarlyIfConversion.cpp.o
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp: In function 'llvm::GlobalVariable* SRAGlobal(llvm::GlobalVariable*, const llvm::DataLayout&)':
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp:548:32: warning: unused variable 'STy' [-Wunused-variable]
     } else if (SequentialType *STy = dyn_cast<SequentialType>(Ty)) {
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EdgeBundles.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExecutionDomainFix.cpp.o
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
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MILexer.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIParser.cpp.o
[ 67%] Linking CXX static library ../../libLLVMSelectionDAG.a
[ 67%] Built target LLVMSelectionDAG
Scanning dependencies of target LLVMDWARFLinker
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerCompileUnit.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIRParser.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerDeclContext.cpp.o
[ 67%] Built target LLVMMIRParser
Scanning dependencies of target LLVMPasses
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinker.cpp.o
[ 67%] Linking CXX static library ../libLLVMDWARFLinker.a
[ 67%] Built target LLVMDWARFLinker
[ 67%] Building CXX object lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Execution.cpp.o
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassPlugin.cpp.o
[ 67%] Building CXX object lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/ExternalFunctions.cpp.o
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/StandardInstrumentations.cpp.o
---
Scanning dependencies of target LLVMSystemZCodeGen
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZAsmPrinter.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZCallingConv.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCLowerMASSVEntries.cpp.o
[ 73%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
[ 73%] Built target LLVMPowerPCCodeGen
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZElimCompare.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZFrameLowering.cpp.o
---
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMLowOverheadLoops.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMCInstLower.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTaggingPreRA.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMacroFusion.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterInfo.cpp.o
---
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MLxExpansionPass.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16HardFloatInfo.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16InstrInfo.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16ISelDAGToDAG.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEGatherScatterLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVETailPredication.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16ISelLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEVPTBlockPass.cpp.o
[ 84%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsAnalyzeImmediate.cpp.o
[ 84%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1FrameLowering.cpp.o
[ 84%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsAsmPrinter.cpp.o
[ 84%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
---
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[ 90%] Built target llvm-cxxfilt
Scanning dependencies of target llvm-reduce
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
[ 90%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-rc
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
Scanning dependencies of target llvm-undname
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalVars.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-undname
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceMetadata.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
Scanning dependencies of target llvm-pdbutil
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DumpOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructions.cpp.o
[ 92%] Linking CXX executable ../../bin/llvm-reduce
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/InputFile.cpp.o
[ 92%] Built target llvm-reduce
Scanning dependencies of target llvm-cov
---
[ 95%] Linking CXX executable ../../bin/lli
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-extract.cpp.o
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-fdr-dump.cpp.o
[ 95%] Built target lli
Scanning dependencies of target llvm-ifs
[ 95%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ifs
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-graph.cpp.o
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-registry.cpp.o
[ 96%] Built target llvm-ifs
---
[100%] Linking CXX executable ../../bin/llvm-exegesis
[100%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/llvm-microsoft-demangle-fuzzer.cpp.o
[100%] Linking CXX executable ../../bin/llvm-microsoft-demangle-fuzzer
[100%] Built target llvm-microsoft-demangle-fuzzer
Scanning dependencies of target llvm-locstats
[100%] Copying llvm-locstats into /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/./bin
[100%] Built target llvm-locstats
[100%] Generating ../../bin/llvm-dlltool
[100%] Built target llvm-dlltool
Scanning dependencies of target llvm-ranlib
[100%] Generating ../../bin/llvm-ranlib
---
[100%] Built target BugpointPasses
Install the project...
-- Install configuration: "Release"
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPIRBuilder.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPConstants.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPKinds.def
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/SupportHelpers.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Annotations.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Error.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/LineEditor/LineEditor.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Wasm.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/WindowsMachineFlag.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/TapiUniversal.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELFObjectFile.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/RelocationResolver.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ArchiveWriter.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolicFile.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/XCOFFObjectFile.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Archive.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Minidump.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/MachOUniversal.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/TapiFile.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/StackMapParser.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolSize.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Error.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/IRObjectFile.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/SymbolizableModule.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LookupResult.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineEntry.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymReader.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymCreator.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineTable.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Header.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/FileEntry.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/InlineInfo.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.td
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfoFlags.def
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.def
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/FixedMetadataKinds.def
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsX86.td
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalIndirectSymbol.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassManager.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DebugLoc.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DataLayout.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/PassManagerInternal.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsMips.td
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/FPEnv.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalVariable.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/LLVMContext.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalObject.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SymbolRewriter.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Cloning.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ASanStackFrameLayout.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/NameAnonGlobals.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/InjectTLIMappings.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterBulk.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopSimplify.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/PromoteMemToReg.h
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CodeMoverUtils.h
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBinaryFormat.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitReader.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitWriter.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitstreamReader.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDWARFLinker.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMFrontendOpenMP.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstrumentation.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAggressiveInstCombine.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstCombine.a
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMScalarOpts.a
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
-- Looking for __cxa_throw in stdc++
-- Looking for __cxa_throw in stdc++ - found
-- Performing Test COMPILER_RT_HAS_Z_TEXT
-- Performing Test COMPILER_RT_HAS_Z_TEXT - Success
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG - Failed
-- Compiler-RT supported architectures: aarch64
-- Generating done
-- Build files have been written to: /checkout/obj/build/aarch64-unknown-linux-gnu/native/sanitizers/build
running: "cmake" "--build" "." "--target" "clang_rt.asan-aarch64" "--config" "Release" "--" "-j" "2"
Scanning dependencies of target RTSanitizerCommonLibc.aarch64
---
In file included from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.h:17:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.cpp:14:
/checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h: In function '__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)':
/checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h:58:65: warning: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' may be used uninitialized in this function [-Wmaybe-uninitialized]
       ReportStringFunctionSizeOverflow(__offset, __size, &stack);       \
In file included from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.cpp:172:0:
/checkout/src/llvm-project/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:3218:21: note: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' was declared here
   __sanitizer_iovec local_iovec;
                     ^
---
Scanning dependencies of target clang_rt.tsan-aarch64
[ 76%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_clock.cpp.o
[ 76%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_debugging.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_debugging.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 76%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_external.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_clock.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_clock.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 76%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_fd.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_external.cpp:12:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_external.cpp:12:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 76%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_flags.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.h:36:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.cpp:13:
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.cpp:13:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 80%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_ignoreset.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_flags.cpp:17:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_flags.cpp:17:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 80%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_interceptors_posix.cpp.o
[ 80%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_interface.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface.cpp:15:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 80%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_interface_ann.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_ann.cpp:20:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_ann.cpp:20:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 84%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_interface_atomic.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_atomic.cpp:26:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_atomic.cpp:26:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 84%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_interface_java.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_java.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_java.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 84%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_malloc_mac.cpp.o
[ 84%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_md5.cpp.o
[ 84%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_mman.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mman.cpp:19:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mman.cpp:19:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 88%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_mutex.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutex.cpp:15:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutex.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 88%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_mutexset.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutexset.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutexset.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 88%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_preinit.cpp.o
[ 88%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_report.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_report.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_report.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 92%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.cpp:23:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.cpp:23:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 92%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl_mutex.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_mutex.cpp:16:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_mutex.cpp:16:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 92%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl_proc.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_proc.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_proc.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 92%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl_report.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_report.cpp:19:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_report.cpp:19:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 92%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl_thread.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_thread.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_thread.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 96%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_stack_trace.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stack_trace.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stack_trace.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 96%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_stat.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stat.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stat.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 96%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_suppressions.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_suppressions.cpp:18:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_suppressions.cpp:18:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 96%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_symbolize.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_symbolize.cpp:20:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_symbolize.cpp:20:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_sync.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors.h:5:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:26:
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:26:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:2337:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_common_interceptors.inc: In function '__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_common_interceptors.inc:3268:58: warning: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' may be used uninitialized in this function [-Wmaybe-uninitialized]
                                      local_iovec.iov_len);
---
   __sanitizer_iovec local_iovec;
                     ^
[100%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_platform_linux.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_sync.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_platform_posix.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_linux.cpp:28:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_linux.cpp:28:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Building C object lib/tsan/CMakeFiles/clang_rt.tsan-aarch64.dir/rtl/tsan_rtl_aarch64.S.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_posix.cpp:22:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_posix.cpp:22:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Linking CXX static library ../linux/libclang_rt.tsan-aarch64.a
[100%] Built target clang_rt.tsan-aarch64
cargo:root=/checkout/obj/build/aarch64-unknown-linux-gnu/native/sanitizers
 finished in 80.953
---
-- Looking for __cxa_throw in stdc++
-- Looking for __cxa_throw in stdc++ - found
-- Performing Test COMPILER_RT_HAS_Z_TEXT
-- Performing Test COMPILER_RT_HAS_Z_TEXT - Success
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG - Failed
-- Linker detection: GNU ld
-- Configuring done
-- Generating done
-- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers/build
---
In file included from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.h:17:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.cpp:14:
/checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h: In function '__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)':
/checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h:58:65: warning: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' may be used uninitialized in this function [-Wmaybe-uninitialized]
       ReportStringFunctionSizeOverflow(__offset, __size, &stack);       \
In file included from /checkout/src/llvm-project/compiler-rt/lib/asan/asan_interceptors.cpp:172:0:
/checkout/src/llvm-project/compiler-rt/lib/asan/../sanitizer_common/sanitizer_common_interceptors.inc:3218:21: note: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' was declared here
   __sanitizer_iovec local_iovec;
                     ^
---
Scanning dependencies of target clang_rt.tsan-x86_64
[ 72%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_debugging.cpp.o
[ 72%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_clock.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_debugging.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 72%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_external.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_clock.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_clock.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 72%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_fd.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_external.cpp:12:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_external.cpp:12:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 77%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_flags.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.h:36:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.cpp:13:
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_fd.cpp:13:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 77%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_ignoreset.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_flags.cpp:17:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_flags.cpp:17:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 77%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interceptors_posix.cpp.o
[ 77%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface.cpp:15:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 77%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_ann.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_ann.cpp:20:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_ann.cpp:20:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 81%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_atomic.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_atomic.cpp:26:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_atomic.cpp:26:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 81%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_interface_java.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_java.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interface_java.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 81%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_malloc_mac.cpp.o
[ 81%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_md5.cpp.o
[ 81%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mman.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mman.cpp:19:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mman.cpp:19:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 86%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutex.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutex.cpp:15:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutex.cpp:15:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 86%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_mutexset.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutexset.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_mutexset.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 86%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_preinit.cpp.o
[ 86%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_report.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_report.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_report.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 86%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.cpp:23:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.cpp:23:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 90%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl_mutex.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_mutex.cpp:16:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_mutex.cpp:16:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 90%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl_proc.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_proc.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_proc.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 90%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl_report.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_report.cpp:19:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_report.cpp:19:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 90%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl_thread.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_thread.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl_thread.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 90%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_stack_trace.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stack_trace.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stack_trace.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 95%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_stat.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stat.cpp:13:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_stat.cpp:13:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 95%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_suppressions.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors.h:5:0,
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:26:
                 from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:26:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_interceptors_posix.cpp:2337:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_common_interceptors.inc: In function '__sanitizer::uptr __interceptor_ptrace(int, int, void*, void*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/../sanitizer_common/sanitizer_common_interceptors.inc:3268:58: warning: 'local_iovec.__sanitizer::__sanitizer_iovec::iov_len' may be used uninitialized in this function [-Wmaybe-uninitialized]
                                      local_iovec.iov_len);
---
   __sanitizer_iovec local_iovec;
                     ^
[ 95%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_symbolize.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_suppressions.cpp:18:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 95%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_sync.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_symbolize.cpp:20:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_symbolize.cpp:20:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[ 95%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_platform_linux.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_sync.cpp:14:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_sync.cpp:14:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Building CXX object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_platform_posix.cpp.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_linux.cpp:28:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_linux.cpp:28:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Building C object lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rtl/tsan_rtl_amd64.S.o
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_posix.cpp:22:0:
In file included from /checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_platform_posix.cpp:22:0:
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function '__tsan::ThreadState* __tsan::cur_thread()':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:466:65: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   return reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current;
                                                                 ^
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h: In function 'void __tsan::set_cur_thread(__tsan::ThreadState*)':
/checkout/src/llvm-project/compiler-rt/lib/tsan/rtl/tsan_rtl.h:474:58: warning: dereferencing type-punned pointer will break strict-aliasing rules [-Wstrict-aliasing]
   reinterpret_cast<ThreadState *>(cur_thread_placeholder)->current = thr;
[100%] Linking CXX static library ../linux/libclang_rt.tsan-x86_64.a
[100%] Built target clang_rt.tsan-x86_64
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers
 finished in 73.081
---
[  4%] Building HexagonGenAsmWriter.inc...
[  4%] Building AArch64GenGlobalISel.inc...
[  4%] Building HexagonGenCallingConv.inc...
[  4%] Building HexagonGenDAGISel.inc...
[  4%] Building AArch64GenGICombiner.inc...
[  4%] Building HexagonGenDFAPacketizer.inc...
[  4%] Building HexagonGenDisassemblerTables.inc...
[  4%] Building HexagonGenInstrInfo.inc...
[  4%] Building HexagonGenMCCodeEmitter.inc...
---
[ 15%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Valgrind.cpp.o
[ 15%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Watchdog.cpp.o
[ 15%] Linking CXX static library ../libLLVMSupport.a
[ 15%] Built target LLVMSupport
Scanning dependencies of target InstallNameToolOptsTableGen
[ 15%] Building InstallNameToolOpts.inc...
[ 15%] Built target InstallNameToolOptsTableGen
[ 16%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[ 16%] Building X86GenRegisterBank.inc...
[ 16%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[ 16%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
---
[ 16%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.o
[ 16%] Linking CXX static library ../libLLVMTableGen.a
[ 16%] Built target LLVMTableGen
[ 16%] Building X86GenSubtargetInfo.inc...
Scanning dependencies of target LLVMTableGenGlobalISel
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[ 16%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[ 16%] Linking CXX static library ../../../lib/libLLVMTableGenGlobalISel.a
[ 16%] Built target LLVMTableGenGlobalISel
[ 16%] Building CXX object lib/Bitstream/Reader/CMakeFiles/LLVMBitstreamReader.dir/BitstreamReader.cpp.o
[ 16%] Linking CXX static library ../../libLLVMBitstreamReader.a
[ 16%] Built target LLVMBitstreamReader
Scanning dependencies of target LLVMBinaryFormat
---
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticPrinter.cpp.o
[ 22%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordSerialization.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[ 23%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SimpleTypeSerializer.cpp.o
[ 23%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/FPEnv.cpp.o
[ 23%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Function.cpp.o
[ 23%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolDumper.cpp.o
[ 23%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolRecordHelpers.cpp.o
[ 23%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/GVMaterializer.cpp.o
---
[ 24%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IntrinsicInst.cpp.o
[ 24%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordMapping.cpp.o
[ 24%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/LLVMContext.cpp.o
[ 24%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
/checkout/src/llvm-project/llvm/lib/DebugInfo/CodeView/TypeStreamMerger.cpp: In member function 'llvm::ArrayRef<unsigned char> {anonymous}::TypeStreamMerger::remapIndices(const CVType&, llvm::MutableArrayRef<unsigned char>)':
/checkout/src/llvm-project/llvm/lib/DebugInfo/CodeView/TypeStreamMerger.cpp:392:12: warning: unused variable 'AlignedSize' [-Wunused-variable]
   unsigned AlignedSize = alignTo(OriginalType.RecordData.size(), 4);
[ 24%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/LLVMContextImpl.cpp.o
[ 24%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeTableCollection.cpp.o
[ 24%] Linking CXX static library ../../libLLVMDebugInfoCodeView.a
[ 24%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/LegacyPassManager.cpp.o
---
[ 27%] Built target LLVMAsmParser
[ 27%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCTargetOptions.cpp.o
[ 27%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCValue.cpp.o
[ 27%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWasmObjectTargetWriter.cpp.o
Scanning dependencies of target LLVMCFGuard
[ 27%] Building CXX object lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.o
[ 27%] Linking CXX static library ../../libLLVMCFGuard.a
[ 27%] Built target LLVMCFGuard
Scanning dependencies of target LLVMIRReader
[ 27%] Building CXX object lib/IRReader/CMakeFiles/LLVMIRReader.dir/IRReader.cpp.o
---
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/ResourceManager.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/Scheduler.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
[ 29%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[ 30%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.o
---
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolicFile.cpp.o
Scanning dependencies of target LLVMARMDisassembler
[ 35%] Building CXX object lib/Target/ARM/Disassembler/CMakeFiles/LLVMARMDisassembler.dir/ARMDisassembler.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolSize.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiFile.cpp.o
[ 35%] Built target LLVMARMDisassembler
[ 35%] Built target LLVMARMDisassembler
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiUniversal.cpp.o
[ 35%] Building CXX object lib/Target/Hexagon/AsmParser/CMakeFiles/LLVMHexagonAsmParser.dir/HexagonAsmParser.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WasmObjectFile.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WindowsMachineFlag.cpp.o
[ 35%] Linking CXX static library ../../../libLLVMHexagonAsmParser.a
---
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VectorUtils.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumGlobals.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumInjectedSources.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VFABIDemangling.cpp.o
[ 46%] Linking CXX static library ../libLLVMAnalysis.a
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumTypes.cpp.o
[ 46%] Built target LLVMAnalysis
Scanning dependencies of target LLVMRuntimeDyld
---
[ 50%] Built target LLVMMCJIT
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GuardUtils.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InlineFunction.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ImportedFunctionsInliningStatistics.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InjectTLIMappings.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/IntegerDivision.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LCSSA.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LibCallsShrinkWrap.cpp.o
[ 50%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Local.cpp.o
---
[ 52%] Built target LLVMAggressiveInstCombine
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAndOrXor.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.o
Scanning dependencies of target LLVMFrontendOpenMP
[ 53%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPConstants.cpp.o
[ 53%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPIRBuilder.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.o
[ 53%] Linking CXX static library ../../libLLVMFrontendOpenMP.a
[ 53%] Built target LLVMFrontendOpenMP
Scanning dependencies of target LLVMInstrumentation
[ 53%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/AddressSanitizer.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineLoadStoreAlloca.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineMulDivRem.cpp.o
---
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DwarfEHPrepare.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalOpt.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EarlyIfConversion.cpp.o
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp: In function 'llvm::GlobalVariable* SRAGlobal(llvm::GlobalVariable*, const llvm::DataLayout&)':
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp:548:32: warning: unused variable 'STy' [-Wunused-variable]
     } else if (SequentialType *STy = dyn_cast<SequentialType>(Ty)) {
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EdgeBundles.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExecutionDomainFix.cpp.o
---
[ 61%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineTraceMetrics.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineVerifier.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ModuloSchedule.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/PatchableFunction.cpp.o
/checkout/src/llvm-project/llvm/lib/CodeGen/ModuloSchedule.cpp: In member function 'void llvm::ModuloScheduleExpander::rewriteScheduledInstr(llvm::MachineBasicBlock*, llvm::ModuloScheduleExpander::InstrMapTy&, unsigned int, unsigned int, llvm::MachineInstr*, unsigned int, unsigned int, unsigned int)':
/checkout/src/llvm-project/llvm/lib/CodeGen/ModuloSchedule.cpp:1179:31: warning: assuming signed overflow does not occur when assuming that (X + c) < X is always false [-Wstrict-overflow]
     if (StagePhi > StageSched && Phi->isPHI())
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRPrinter.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRPrintingPass.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MacroFusion.cpp.o
[ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/NonRelocatableStringpool.cpp.o
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
[ 67%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/TargetLowering.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIParser.cpp.o
[ 67%] Linking CXX static library ../../libLLVMSelectionDAG.a
[ 67%] Built target LLVMSelectionDAG
Scanning dependencies of target LLVMDWARFLinker
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerCompileUnit.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIRParser.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerDeclContext.cpp.o
[ 67%] Built target LLVMMIRParser
Scanning dependencies of target LLVMPasses
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinker.cpp.o
[ 67%] Linking CXX static library ../libLLVMDWARFLinker.a
[ 67%] Built target LLVMDWARFLinker
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/StandardInstrumentations.cpp.o
Scanning dependencies of target LLVMInterpreter
---
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCExpandISEL.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZConstantPoolValue.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZElimCompare.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCLowerMASSVEntries.cpp.o
[ 73%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
[ 73%] Built target LLVMPowerPCCodeGen
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZHazardRecognizer.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelDAGToDAG.cpp.o
---
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMachineFunctionInfo.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMacroFusion.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTaggingPreRA.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMOptimizeBarriersPass.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterBankInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
---
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetTransformInfo.cpp.o
[ 83%] Linking CXX static library ../../libLLVMAArch64CodeGen.a
[ 83%] Built target LLVMAArch64CodeGen
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MLxExpansionPass.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEGatherScatterLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVETailPredication.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEVPTBlockPass.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16HardFloat.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
---
[ 90%] Linking CXX executable ../../bin/llvm-cxxfilt
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[ 90%] Built target llvm-cxxfilt
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
Scanning dependencies of target llvm-reduce
[ 90%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-rc
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
Scanning dependencies of target llvm-undname
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-undname
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalVars.cpp.o
[ 91%] Built target llvm-undname
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceMetadata.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DumpOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/ExplainOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructions.cpp.o
[ 92%] Linking CXX executable ../../bin/llvm-reduce
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/llvm-pdbutil.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/FormatUtil.cpp.o
[ 92%] Built target llvm-reduce
---
[ 95%] Linking CXX executable ../../bin/lli
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-extract.cpp.o
[ 95%] Built target lli
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-fdr-dump.cpp.o
Scanning dependencies of target llvm-ifs
[ 95%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ifs
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-graph.cpp.o
[ 96%] Built target llvm-ifs
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-registry.cpp.o
---
Scanning dependencies of target llvm-microsoft-demangle-fuzzer
[100%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[100%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/llvm-microsoft-demangle-fuzzer.cpp.o
[100%] Built target llvm-objdump
Scanning dependencies of target llvm-locstats
[100%] Copying llvm-locstats into /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/./bin
[100%] Built target llvm-locstats
[100%] Generating ../../bin/llvm-dlltool
[100%] Built target llvm-dlltool
[100%] Linking CXX executable ../../bin/llvm-microsoft-demangle-fuzzer
Scanning dependencies of target llvm-ranlib
---
[100%] Built target BugpointPasses
Install the project...
-- Install configuration: "Release"
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Frontend
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPIRBuilder.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPConstants.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Frontend/OpenMP/OMPKinds.def
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Testing/Support
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/SupportHelpers.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Annotations.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Testing/Support/Error.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/LineEditor/LineEditor.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/Wasm.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/WindowsMachineFlag.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/TapiUniversal.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/ELFObjectFile.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/RelocationResolver.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/ArchiveWriter.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolicFile.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/XCOFFObjectFile.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/Archive.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/Minidump.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/MachOUniversal.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/TapiFile.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/StackMapParser.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/SymbolSize.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/Error.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Object/IRObjectFile.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/Symbolize/SymbolizableModule.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LookupResult.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineEntry.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymReader.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/GsymCreator.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineTable.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Header.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/FileEntry.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/InlineInfo.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.td
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/DebugInfoFlags.def
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.def
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/FixedMetadataKinds.def
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsX86.td
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalIndirectSymbol.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/Metadata.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/LegacyPassManager.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/DebugLoc.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/DataLayout.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/PassManagerInternal.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsMips.td
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/FPEnv.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalVariable.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/LLVMContext.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/GlobalObject.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SymbolRewriter.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/Cloning.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/ASanStackFrameLayout.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/NameAnonGlobals.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/InjectTLIMappings.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterBulk.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LoopSimplify.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/PromoteMemToReg.h
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/CodeMoverUtils.h
---
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMBinaryFormat.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMBitReader.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMBitWriter.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMBitstreamReader.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMDWARFLinker.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMFrontendOpenMP.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMInstrumentation.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMAggressiveInstCombine.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMInstCombine.a
-- Installing: /checkout/obj/build/aarch64-unknown-linux-gnu/llvm/lib/libLLVMScalarOpts.a
---
   Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
   Compiling rustc-main v0.0.0 (/checkout/src/rustc)
    Finished release [optimized] target(s) in 36m 33s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / aarch64-unknown-linux-gnu)
Building LLD for aarch64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/lld" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=aarch64-unknown-linux-gnueabi-gcc" "-DCMAKE_CXX_COMPILER=aarch64-unknown-linux-gnueabi-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -static-libstdc++" "-DLLVM_CONFIG_PATH=/checkout/obj/build/bootstrap/debug/llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_TABLEGEN_EXE=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-tblgen" "-DCMAKE_CXX_STANDARD=14" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/aarch64-unknown-linux-gnu/lld" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 5.2.0
-- Check for working C compiler: /x-tools/aarch64-unknown-linux-gnueabi/bin/aarch64-unknown-linux-gnueabi-gcc
-- Check for working C compiler: /x-tools/aarch64-unknown-linux-gnueabi/bin/aarch64-unknown-linux-gnueabi-gcc -- works
-- Detecting C compiler ABI info
---
   Compiling rustc-ap-rustc_ast v659.0.0
   Compiling rustc-ap-rustc_errors v659.0.0
   Compiling rustc-ap-rustc_feature v659.0.0
   Compiling rustc-ap-rustc_target v659.0.0
   Compiling rls-ipc v0.1.0 (/checkout/src/tools/rls/rls-ipc)
   Compiling rustc-ap-rustc_session v659.0.0
   Compiling rls-rustc v0.6.0 (/checkout/src/tools/rls/rls-rustc)
   Compiling rustc-ap-rustc_parse v659.0.0
   Compiling rustc-ap-rustc_attr v659.0.0
---
-rw-r--r--  1 vsts docker  22M Jun  6 15:16 rust-std-nightly-aarch64-unknown-linux-gnu.tar.xz

src/ci/scripts/upload-artifacts.sh: line 39: DEPLOY_BUCKET: unbound variable

##[error]Bash exited with code '1'.
##[section]Finishing: Upload artifacts
##[section]Starting: Checkout rust-lang/rust@refs/pull/73058/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73058/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3552) (python)
##[section]Finishing: Finalize Job
