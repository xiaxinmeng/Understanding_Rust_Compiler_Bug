plain
##[section]Starting: Linux dist-various-2
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 31'
Agent machine name: 'fv-az659'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a5a82fca-bb43-48cd-9024-1078a7bc2047.sh

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
 ---> 088af03142e5
Step 7/48 : RUN add-apt-repository -y 'deb http://apt.dilos.org/dilos dilos2 main'
 ---> Using cache
 ---> aae1c6ed405a
Step 8/48 : ENV     AR_x86_64_fuchsia=x86_64-fuchsia-ar     CC_x86_64_fuchsia=x86_64-fuchsia-clang     CXX_x86_64_fuchsia=x86_64-fuchsia-clang++     AR_aarch64_fuchsia=aarch64-fuchsia-ar     CC_aarch64_fuchsia=aarch64-fuchsia-clang     CXX_aarch64_fuchsia=aarch64-fuchsia-clang++     AR_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-ar     CC_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-gcc     CXX_sparcv9_sun_solaris=sparcv9-sun-solaris2.10-g++     AR_x86_64_sun_solaris=x86_64-sun-solaris2.10-ar     CC_x86_64_sun_solaris=x86_64-sun-solaris2.10-gcc     CXX_x86_64_sun_solaris=x86_64-sun-solaris2.10-g++     CC_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-gcc-7     CXX_armv7_unknown_linux_gnueabi=arm-linux-gnueabi-g++-7     AR_x86_64_fortanix_unknown_sgx=ar     CC_x86_64_fortanix_unknown_sgx=x86_64-fortanix-unknown-sgx-clang-11     CFLAGS_x86_64_fortanix_unknown_sgx="-mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     CXX_x86_64_fortanix_unknown_sgx=x86_64-fortanix-unknown-sgx-clang++-11     CXXFLAGS_x86_64_fortanix_unknown_sgx="-mlvi-hardening -mllvm -x86-experimental-lvi-inline-asm-hardening"     CC=gcc-7     CXX=g++-7
 ---> 4e08f10d073d
Step 9/48 : WORKDIR /build
 ---> Using cache
 ---> 0836e546b131
---
 ---> d8f381ff34f5
Step 21/48 : COPY dist-various-2/build-x86_64-fortanix-unknown-sgx-toolchain.sh /tmp/
 ---> Using cache
 ---> 3df6cee8a87d
Step 22/48 : COPY dist-various-2/x86_64-fortanix-unknown-sgx-clang-wrap.sh /usr/bin/x86_64-fortanix-unknown-sgx-clang-11
 ---> a203ee765f22
 ---> a203ee765f22
Step 23/48 : RUN ln -s /usr/bin/x86_64-fortanix-unknown-sgx-clang-11 /usr/bin/x86_64-fortanix-unknown-sgx-clang++-11
 ---> 4771e54913a6
Step 24/48 : RUN /tmp/build-x86_64-fortanix-unknown-sgx-toolchain.sh "800f95131fe6acd20b96b6f4723ca3c820f3d379"
 ---> Using cache
 ---> 2accafb73097
---
Scanning dependencies of target LLVMTableGenGlobalISel
Scanning dependencies of target LLVMTableGen
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TableGenBackend.cpp.o
[ 10%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[ 10%] Linking CXX static library ../../../lib/libLLVMTableGenGlobalISel.a
[ 10%] Built target LLVMTableGenGlobalISel
Scanning dependencies of target LLVMBitstreamReader
[ 10%] Building CXX object lib/Bitstream/Reader/CMakeFiles/LLVMBitstreamReader.dir/BitstreamReader.cpp.o
[ 10%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.o
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
Scanning dependencies of target ARMCommonTableGen
---
[ 16%] Building AArch64GenGlobalISel.inc...
[ 16%] Building HexagonGenDFAPacketizer.inc...
[ 16%] Building HexagonGenDisassemblerTables.inc...
[ 16%] Building HexagonGenInstrInfo.inc...
[ 16%] Building AArch64GenGICombiner.inc...
[ 16%] Building HexagonGenMCCodeEmitter.inc...
[ 16%] Building HexagonGenRegisterInfo.inc...
[ 17%] Building HexagonGenSubtargetInfo.inc...
[ 17%] Built target HexagonCommonTableGen
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
[ 26%] Building CXX object lib/AsmParser/CMakeFiles/LLVMAsmParser.dir/Parser.cpp.o
[ 26%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWin64EH.cpp.o
[ 26%] Linking CXX static library ../libLLVMAsmParser.a
[ 26%] Built target LLVMAsmParser
Scanning dependencies of target LLVMCFGuard
[ 26%] Building CXX object lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.o
[ 26%] Linking CXX static library ../../libLLVMCFGuard.a
[ 26%] Built target LLVMCFGuard
[ 26%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCWinEH.cpp.o
Scanning dependencies of target LLVMAArch64Utils
---
[ 28%] Linking CXX static library ../../../libLLVMX86Info.a
[ 28%] Linking CXX static library ../../../libLLVMX86Utils.a
[ 28%] Built target LLVMX86Info
[ 28%] Built target LLVMX86Utils
Scanning dependencies of target InstallNameToolOptsTableGen
[ 28%] Building InstallNameToolOpts.inc...
[ 28%] Built target InstallNameToolOptsTableGen
[ 28%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeAnalyzer.cpp.o
Scanning dependencies of target LLVMMCParser
[ 28%] Building CXX object lib/MC/MCParser/CMakeFiles/LLVMMCParser.dir/AsmLexer.cpp.o
[ 28%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitReader.cpp.o
---
[ 29%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.o
[ 30%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RegisterFile.cpp.o
[ 30%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
[ 30%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/ResourceManager.cpp.o
[ 31%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.o
[ 31%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/HardwareUnits/RetireControlUnit.cpp.o
[ 31%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.o
[ 31%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
[ 31%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.o
[ 31%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.o
[ 31%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
---
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolicFile.cpp.o
Scanning dependencies of target LLVMARMAsmParser
[ 36%] Building CXX object lib/Target/ARM/AsmParser/CMakeFiles/LLVMARMAsmParser.dir/ARMAsmParser.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolSize.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiFile.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiUniversal.cpp.o
[ 36%] Built target LLVMARMAsmParser
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WasmObjectFile.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WindowsMachineFlag.cpp.o
[ 36%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WindowsResource.cpp.o
---
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumGlobals.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VectorUtils.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumInjectedSources.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VFABIDemangling.cpp.o
[ 46%] Linking CXX static library ../libLLVMAnalysis.a
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumTypes.cpp.o
[ 46%] Built target LLVMAnalysis
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeExeSymbol.cpp.o
---
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GlobalStatus.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GuardUtils.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InlineFunction.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ImportedFunctionsInliningStatistics.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InjectTLIMappings.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/IntegerDivision.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LCSSA.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LibCallsShrinkWrap.cpp.o
[ 49%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Local.cpp.o
---
[ 51%] Building CXX object lib/Transforms/AggressiveInstCombine/CMakeFiles/LLVMAggressiveInstCombine.dir/TruncInstCombine.cpp.o
[ 51%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAddSub.cpp.o
[ 51%] Linking CXX static library ../../libLLVMAggressiveInstCombine.a
[ 51%] Built target LLVMAggressiveInstCombine
Scanning dependencies of target LLVMFrontendOpenMP
[ 51%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPConstants.cpp.o
[ 51%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.o
[ 51%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMPIRBuilder.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.o
[ 52%] Linking CXX static library ../../libLLVMFrontendOpenMP.a
[ 52%] Built target LLVMFrontendOpenMP
[ 52%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/AddressSanitizer.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.o
[ 52%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/BoundsChecking.cpp.o
[ 52%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCompares.cpp.o
---
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DetectDeadLanes.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalOpt.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DFAPacketizer.cpp.o
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp: In function 'llvm::GlobalVariable* SRAGlobal(llvm::GlobalVariable*, const llvm::DataLayout&)':
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp:548:32: warning: unused variable 'STy' [-Wunused-variable]
     } else if (SequentialType *STy = dyn_cast<SequentialType>(Ty)) {
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DwarfEHPrepare.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/IPConstantPropagation.cpp.o
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
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIRParser.cpp.o
[ 67%] Linking CXX static library ../../libLLVMSelectionDAG.a
[ 67%] Built target LLVMSelectionDAG
Scanning dependencies of target LLVMDWARFLinker
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerCompileUnit.cpp.o
[ 67%] Built target LLVMMIRParser
Scanning dependencies of target LLVMPasses
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassBuilder.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerDeclContext.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinker.cpp.o
[ 68%] Linking CXX static library ../libLLVMDWARFLinker.a
[ 68%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/StandardInstrumentations.cpp.o
[ 68%] Built target LLVMDWARFLinker
Scanning dependencies of target LLVMInterpreter
---
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCExpandISEL.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelDAGToDAG.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZISelLowering.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCLowerMASSVEntries.cpp.o
[ 73%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
[ 73%] Built target LLVMPowerPCCodeGen
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZLDCleanup.cpp.o
Scanning dependencies of target LLVMWebAssemblyCodeGen
---
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMTargetTransformInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SelectionDAGInfo.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MLxExpansionPass.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEGatherScatterLowering.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVETailPredication.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEVPTBlockPass.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTaggingPreRA.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ThumbRegisterInfo.cpp.o
---
[ 88%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86InstrFoldTables.cpp.o
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
[ 93%] Built target llvm-exegesis
Scanning dependencies of target llvm-extract
[ 93%] Building CXX object tools/llvm-extract/CMakeFiles/llvm-extract.dir/llvm-extract.cpp.o
[ 93%] Built target llvm-elfabi
Scanning dependencies of target llvm-ifs
[ 93%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 94%] Linking CXX executable ../../bin/llvm-ifs
[ 94%] Built target llvm-extract
Scanning dependencies of target llvm-isel-fuzzer
[ 94%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/DummyISelFuzzer.cpp.o
---
[ 97%] Linking CXX executable ../../bin/llvm-readobj
[ 97%] Linking CXX executable ../../bin/llvm-pdbutil
[ 97%] Built target llvm-readobj
[ 97%] Built target llvm-pdbutil
Scanning dependencies of target llvm-reduce
[ 97%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[ 97%] Building CXX object tools/llvm-rtdyld/CMakeFiles/llvm-rtdyld.dir/llvm-rtdyld.cpp.o
[ 97%] Building CXX object tools/llvm-rtdyld/CMakeFiles/llvm-rtdyld.dir/llvm-rtdyld.cpp.o
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 98%] Linking CXX executable ../../bin/llvm-rtdyld
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
Scanning dependencies of target llvm-size
[ 98%] Building CXX object tools/llvm-size/CMakeFiles/llvm-size.dir/llvm-size.cpp.o
[ 98%] Building CXX object tools/llvm-size/CMakeFiles/llvm-size.dir/llvm-size.cpp.o
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
[ 98%] Linking CXX executable ../../bin/llvm-size
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalVars.cpp.o
[ 98%] Built target llvm-size
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceMetadata.cpp.o
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
Scanning dependencies of target llvm-special-case-list-fuzzer
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
[ 98%] Building CXX object tools/llvm-special-case-list-fuzzer/CMakeFiles/llvm-special-case-list-fuzzer.dir/special-case-list-fuzzer.cpp.o
[ 98%] Building CXX object tools/llvm-special-case-list-fuzzer/CMakeFiles/llvm-special-case-list-fuzzer.dir/special-case-list-fuzzer.cpp.o
[ 98%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructions.cpp.o
[ 98%] Linking CXX executable ../../bin/llvm-reduce
[ 98%] Built target llvm-special-case-list-fuzzer
Scanning dependencies of target llvm-split
[ 98%] Building CXX object tools/llvm-split/CMakeFiles/llvm-split.dir/llvm-split.cpp.o
---
[ 99%] Built target sanstats
Scanning dependencies of target yaml2obj
[ 99%] Building CXX object tools/yaml2obj/CMakeFiles/yaml2obj.dir/yaml2obj.cpp.o
[ 99%] Built target verify-uselistorder
Scanning dependencies of target llvm-locstats
[ 99%] Copying llvm-locstats into /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/./bin
[100%] Built target llvm-locstats
Scanning dependencies of target llvm-lib
[100%] Generating ../../bin/llvm-lib
[100%] Built target llvm-lib
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
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
-rw-r--r--  1 vsts docker  22M Jun  9 15:14 rust-std-nightly-x86_64-unknown-linux-gnux32.tar.gz
-rw-r--r--  1 vsts docker  15M Jun  9 15:14 rust-std-nightly-x86_64-unknown-linux-gnux32.tar.xz


src/ci/scripts/upload-artifacts.sh: line 39: DEPLOY_BUCKET: unbound variable
##[error]Bash exited with code '1'.
##[section]Finishing: Upload artifacts
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
Terminate orphan process: pid (4089) (python)
##[section]Finishing: Finalize Job
