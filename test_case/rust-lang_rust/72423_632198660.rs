plain
##[section]Starting: Linux x86_64-gnu-tools
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 4'
Agent machine name: 'fv-az619'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/88190abc-fe3a-45d6-bdb2-8195a35a2bd3.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72423/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72423/merge:refs/remotes/pull/72423/merge
---
Scanning dependencies of target LLVMTableGenGlobalISel
[  8%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
Scanning dependencies of target LLVMTableGen
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Error.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDag.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/JSONBackend.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagEdge.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Main.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagInstr.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/Record.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/SetTheory.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/StringMatcher.cpp.o
[  9%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[  9%] Linking CXX static library ../../../lib/libLLVMTableGenGlobalISel.a
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGLexer.cpp.o
[  9%] Built target LLVMTableGenGlobalISel
[  9%] Building CXX object lib/TableGen/CMakeFiles/LLVMTableGen.dir/TGParser.cpp.o
---
[ 17%] Building ARMGenDisassemblerTables.inc...
[ 17%] Building ARMGenFastISel.inc...
[ 17%] Building ARMGenGlobalISel.inc...
[ 18%] Building ARMGenInstrInfo.inc...
[ 18%] Building AArch64GenGICombiner.inc...
[ 18%] Building AArch64GenInstrInfo.inc...
[ 18%] Building ARMGenMCPseudoLowering.inc...
[ 18%] Building ARMGenRegisterBank.inc...
[ 18%] Building ARMGenRegisterInfo.inc...
---
[ 21%] Building SystemZGenSubtargetInfo.inc...
Scanning dependencies of target X86CommonTableGen
[ 21%] Building X86GenAsmMatcher.inc...
[ 21%] Built target SystemZCommonTableGen
Scanning dependencies of target InstallNameToolOptsTableGen
[ 21%] Building InstallNameToolOpts.inc...
[ 21%] Built target InstallNameToolOptsTableGen
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AbstractCallSite.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Attributes.cpp.o
[ 21%] Building X86GenAsmWriter.inc...
---
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticHandler.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticInfo.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/DiagnosticPrinter.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Dominators.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/FPEnv.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/GVMaterializer.cpp.o
[ 21%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Globals.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRBuilder.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/IRPrintingPasses.cpp.o
---
[ 26%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/SampleProfWriter.cpp.o
[ 26%] Building CXX object lib/AsmParser/CMakeFiles/LLVMAsmParser.dir/Parser.cpp.o
[ 26%] Linking CXX static library ../libLLVMProfileData.a
[ 26%] Built target LLVMProfileData
Scanning dependencies of target LLVMCFGuard
[ 26%] Building CXX object lib/Transforms/CFGuard/CMakeFiles/LLVMCFGuard.dir/CFGuard.cpp.o
[ 26%] Built target LLVMAsmParser
Scanning dependencies of target LLVMMCDisassembler
[ 26%] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/Disassembler.cpp.o
[ 26%] Linking CXX static library ../../libLLVMCFGuard.a
---
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FileWriter.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/InstrBuilder.cpp.o
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Instruction.cpp.o
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymCreator.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Pipeline.cpp.o
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/GsymReader.cpp.o
[ 27%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
[ 27%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/EntryStage.cpp.o
[ 28%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LineTable.cpp.o
[ 28%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/ExecuteStage.cpp.o
---
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolicFile.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/SymbolSize.cpp.o
Scanning dependencies of target LLVMHexagonDisassembler
[ 35%] Building CXX object lib/Target/Hexagon/Disassembler/CMakeFiles/LLVMHexagonDisassembler.dir/HexagonDisassembler.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiFile.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/TapiUniversal.cpp.o
[ 35%] Built target LLVMHexagonDisassembler
Scanning dependencies of target LLVMMSP430AsmParser
[ 35%] Building CXX object lib/Target/MSP430/AsmParser/CMakeFiles/LLVMMSP430AsmParser.dir/MSP430AsmParser.cpp.o
[ 35%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/WasmObjectFile.cpp.o
---
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VectorUtils.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStreamBuilder.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InjectedSourceStream.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/VFABIDemangling.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[ 46%] Built target LLVMAnalysis
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
[ 46%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumGlobals.cpp.o
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
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCompares.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineLoadStoreAlloca.cpp.o
[ 53%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineMulDivRem.cpp.o
---
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalDCE.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DetectDeadLanes.cpp.o
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalOpt.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DFAPacketizer.cpp.o
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp: In function 'llvm::GlobalVariable* SRAGlobal(llvm::GlobalVariable*, const llvm::DataLayout&)':
/checkout/src/llvm-project/llvm/lib/Transforms/IPO/GlobalOpt.cpp:548:32: warning: unused variable 'STy' [-Wunused-variable]
     } else if (SequentialType *STy = dyn_cast<SequentialType>(Ty)) {
[ 57%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/DwarfEHPrepare.cpp.o
[ 58%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EarlyIfConversion.cpp.o
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
Scanning dependencies of target LLVMMIRParser
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MILexer.cpp.o
[ 67%] Linking CXX static library ../../libLLVMSelectionDAG.a
[ 67%] Built target LLVMSelectionDAG
Scanning dependencies of target LLVMDWARFLinker
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerCompileUnit.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIParser.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinkerDeclContext.cpp.o
[ 67%] Building CXX object lib/CodeGen/MIRParser/CMakeFiles/LLVMMIRParser.dir/MIRParser.cpp.o
[ 67%] Building CXX object lib/DWARFLinker/CMakeFiles/LLVMDWARFLinker.dir/DWARFLinker.cpp.o
[ 67%] Built target LLVMMIRParser
[ 67%] Linking CXX static library ../libLLVMDWARFLinker.a
Scanning dependencies of target LLVMPasses
[ 67%] Built target LLVMDWARFLinker
---
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCExpandISEL.cpp.o
[ 73%] Building CXX object lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcMCInstLower.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
[ 73%] Building CXX object lib/Target/Sparc/CMakeFiles/LLVMSparcCodeGen.dir/SparcTargetObjectFile.cpp.o
[ 73%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCLowerMASSVEntries.cpp.o
[ 73%] Built target LLVMSparcCodeGen
Scanning dependencies of target LLVMSystemZCodeGen
[ 73%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZAsmPrinter.cpp.o
[ 73%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
---
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMacroFusion.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 81%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterInfo.cpp.o
[ 81%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTaggingPreRA.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterBankInfo.cpp.o
[ 82%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
[ 82%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMSelectionDAGInfo.cpp.o
---
[ 83%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SIMDInstrOpt.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MLxExpansionPass.cpp.o
[ 83%] Linking CXX static library ../../libLLVMAArch64CodeGen.a
[ 83%] Built target LLVMAArch64CodeGen
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEGatherScatterLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVETailPredication.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/MVEVPTBlockPass.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
Scanning dependencies of target LLVMMipsCodeGen
[ 83%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16FrameLowering.cpp.o
[ 83%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ThumbRegisterInfo.cpp.o
---
[ 90%] Linking CXX executable ../../bin/llvm-cxxfilt
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
[ 90%] Built target llvm-cxxfilt
Scanning dependencies of target llvm-reduce
[ 90%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[ 90%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-rc
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
[ 91%] Built target llvm-rc
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalVars.cpp.o
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceMetadata.cpp.o
[ 91%] Linking CXX executable ../../bin/llvm-undname
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
[ 91%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
Scanning dependencies of target llvm-pdbutil
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/BytesOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructions.cpp.o
[ 92%] Linking CXX executable ../../bin/llvm-reduce
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/ExplainOutputStyle.cpp.o
[ 92%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/InputFile.cpp.o
[ 92%] Built target llvm-reduce
---
[ 95%] Linking CXX executable ../../bin/lli
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-fdr-dump.cpp.o
[ 95%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-graph-diff.cpp.o
[ 95%] Built target lli
Scanning dependencies of target llvm-ifs
[ 95%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ifs
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-registry.cpp.o
[ 96%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-stacks.cpp.o
[ 96%] Built target llvm-ifs
---
[100%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[100%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/llvm-microsoft-demangle-fuzzer.cpp.o
[100%] Linking CXX executable ../../bin/llvm-microsoft-demangle-fuzzer
[100%] Built target llvm-exegesis
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
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
normalized stderr:
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:8:62
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                                              ^^^^ future returned by `private_future` is not `Send`
   = note: `-D clippy::future-not-send` implied by `-D warnings`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:9:5
   |
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                         -- has type `std::rc::Rc<[u8]>` which is not `Send`
LL |     async { true }.await
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
   | - `rc` is later dropped here
   | - `rc` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:9:5
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                       ---- has type `&std::cell::Cell<usize>` which is not `Send`
LL |     async { true }.await
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `cell` maybe used later
   | - `cell` is later dropped here
   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`

error: future cannot be sent between threads safely
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:12:42
   |
LL | pub async fn public_future(rc: Rc<[u8]>) {
   |                                          ^ future returned by `public_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:13:5
   |
   |
LL | pub async fn public_future(rc: Rc<[u8]>) {
   |                            -- has type `std::rc::Rc<[u8]>` which is not `Send`
LL |     async { true }.await;
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
   | - `rc` is later dropped here
   | - `rc` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:20:63
   |
   |
LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                                               ^^^^ future returned by `private_future2` is not `Send`
note: captured value is not `Send`
  --> $DIR/future_not_send.rs:20:26
   |
   |
LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                          ^^ has type `std::rc::Rc<[u8]>` which is not `Send`
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
note: captured value is not `Send`
  --> $DIR/future_not_send.rs:20:40
   |
LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                        ^^^^ has type `&std::cell::Cell<usize>` which is not `Send`
   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:24:43
   |
   |
LL | pub async fn public_future2(rc: Rc<[u8]>) {}
   |                                           ^ future returned by `public_future2` is not `Send`
note: captured value is not `Send`
  --> $DIR/future_not_send.rs:24:29
   |
   |
LL | pub async fn public_future2(rc: Rc<[u8]>) {}
   |                             ^^ has type `std::rc::Rc<[u8]>` which is not `Send`
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:35:39
   |
LL |     async fn private_future(&self) -> usize {
LL |     async fn private_future(&self) -> usize {
   |                                       ^^^^^ future returned by `private_future` is not `Send`
   |
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:36:9
   |
LL |     async fn private_future(&self) -> usize {
   |                             ----- has type `&Dummy` which is not `Send`
LL |         async { true }.await;
   |         ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
LL |         self.rc.len()
   |     - `&self` is later dropped here
   |     - `&self` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:40:39
   |
LL |     pub async fn public_future(&self) {
LL |     pub async fn public_future(&self) {
   |                                       ^ future returned by `public_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:41:9
   |
LL |     pub async fn public_future(&self) {
LL |     pub async fn public_future(&self) {
   |                                ----- has type `&Dummy` which is not `Send`
LL |         self.private_future().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
   |     - `&self` is later dropped here
   |     - `&self` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:50:37
   |
LL | async fn generic_future<T>(t: T) -> T
LL | async fn generic_future<T>(t: T) -> T
   |                                     ^ future returned by `generic_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:55:5
   |
LL |     let rt = &t;
---

error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:66:34
   |
LL | async fn unclear_future<T>(t: T) {}
   |                                  ^ future returned by `unclear_future` is not `Send`
note: captured value is not `Send`
  --> $DIR/future_not_send.rs:66:28
   |
   |
LL | async fn unclear_future<T>(t: T) {}
   |                            ^ has type `T` which is not `Send`
   = note: `T` doesn't implement `std::marker::Send`
error: aborting due to 8 previous errors




expected stderr:
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:8:62
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                                              ^^^^ future returned by `private_future` is not `Send`
   = note: `-D clippy::future-not-send` implied by `-D warnings`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:9:5
   |
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                         -- has type `std::rc::Rc<[u8]>` which is not `Send`
LL |     async { true }.await
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
   | - `rc` is later dropped here
   | - `rc` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:9:5
   |
LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |                                       ---- has type `&std::cell::Cell<usize>` which is not `Send`
LL |     async { true }.await
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `cell` maybe used later
   | - `cell` is later dropped here
   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`

error: future cannot be sent between threads safely
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:12:42
   |
LL | pub async fn public_future(rc: Rc<[u8]>) {
   |                                          ^ future returned by `public_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:13:5
   |
   |
LL | pub async fn public_future(rc: Rc<[u8]>) {
   |                            -- has type `std::rc::Rc<[u8]>` which is not `Send`
LL |     async { true }.await;
   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
   | - `rc` is later dropped here
   | - `rc` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:20:63
   |
   |
LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
   |
   |
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:24:43
   |
   |
LL | pub async fn public_future2(rc: Rc<[u8]>) {}
   |
   |
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:35:39
   |
LL |     async fn private_future(&self) -> usize {
LL |     async fn private_future(&self) -> usize {
   |                                       ^^^^^ future returned by `private_future` is not `Send`
   |
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:36:9
   |
LL |     async fn private_future(&self) -> usize {
   |                             ----- has type `&Dummy` which is not `Send`
LL |         async { true }.await;
   |         ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
LL |         self.rc.len()
   |     - `&self` is later dropped here
   |     - `&self` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:40:39
   |
LL |     pub async fn public_future(&self) {
LL |     pub async fn public_future(&self) {
   |                                       ^ future returned by `public_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:41:9
   |
LL |     pub async fn public_future(&self) {
LL |     pub async fn public_future(&self) {
   |                                ----- has type `&Dummy` which is not `Send`
LL |         self.private_future().await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
   |     - `&self` is later dropped here
   |     - `&self` is later dropped here
   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:50:37
   |
LL | async fn generic_future<T>(t: T) -> T
LL | async fn generic_future<T>(t: T) -> T
   |                                     ^ future returned by `generic_future` is not `Send`
note: future is not `Send` as this value is used across an await
  --> $DIR/future_not_send.rs:55:5
   |
LL |     let rt = &t;
---

error: future cannot be sent between threads safely
  --> $DIR/future_not_send.rs:66:34
   |
LL | async fn unclear_future<T>(t: T) {}
   |
   = note: `T` doesn't implement `std::marker::Send`

error: aborting due to 8 previous errors
---

 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:8:62
    |
 LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
    |                                                              ^^^^ future returned by `private_future` is not `Send`
    = note: `-D clippy::future-not-send` implied by `-D warnings`
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:9:5
    |
    |
 LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
    |                         -- has type `std::rc::Rc<[u8]>` which is not `Send`
 LL |     async { true }.await
    |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
    | - `rc` is later dropped here
    | - `rc` is later dropped here
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:9:5
    |
 LL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
    |                                       ---- has type `&std::cell::Cell<usize>` which is not `Send`
 LL |     async { true }.await
    |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `cell` maybe used later
    | - `cell` is later dropped here
    = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`
 
 error: future cannot be sent between threads safely
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:12:42
    |
 LL | pub async fn public_future(rc: Rc<[u8]>) {
    |                                          ^ future returned by `public_future` is not `Send`
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:13:5
    |
    |
 LL | pub async fn public_future(rc: Rc<[u8]>) {
    |                            -- has type `std::rc::Rc<[u8]>` which is not `Send`
 LL |     async { true }.await;
    |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
    | - `rc` is later dropped here
    | - `rc` is later dropped here
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:20:63
    |
    |
 LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
+   |                                                               ^^^^ future returned by `private_future2` is not `Send`
    |
+note: captured value is not `Send`
+  --> $DIR/future_not_send.rs:20:26
+  --> $DIR/future_not_send.rs:20:26
+   |
+LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
+   |                          ^^ has type `std::rc::Rc<[u8]>` which is not `Send`
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
+note: captured value is not `Send`
+  --> $DIR/future_not_send.rs:20:40
+   |
+LL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {
+   |                                        ^^^^ has type `&std::cell::Cell<usize>` which is not `Send`
    = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:24:43
    |
    |
 LL | pub async fn public_future2(rc: Rc<[u8]>) {}
-   |                                           ^
+   |                                           ^ future returned by `public_future2` is not `Send`
+note: captured value is not `Send`
+  --> $DIR/future_not_send.rs:24:29
+   |
+   |
+LL | pub async fn public_future2(rc: Rc<[u8]>) {}
+   |                             ^^ has type `std::rc::Rc<[u8]>` which is not `Send`
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:35:39
    |
 LL |     async fn private_future(&self) -> usize {
 LL |     async fn private_future(&self) -> usize {
    |                                       ^^^^^ future returned by `private_future` is not `Send`
    |
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:36:9
    |
 LL |     async fn private_future(&self) -> usize {
    |                             ----- has type `&Dummy` which is not `Send`
 LL |         async { true }.await;
    |         ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
 LL |         self.rc.len()
    |     - `&self` is later dropped here
    |     - `&self` is later dropped here
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:40:39
    |
 LL |     pub async fn public_future(&self) {
 LL |     pub async fn public_future(&self) {
    |                                       ^ future returned by `public_future` is not `Send`
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:41:9
    |
 LL |     pub async fn public_future(&self) {
 LL |     pub async fn public_future(&self) {
    |                                ----- has type `&Dummy` which is not `Send`
 LL |         self.private_future().await;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later
    |     - `&self` is later dropped here
    |     - `&self` is later dropped here
    = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:50:37
    |
 LL | async fn generic_future<T>(t: T) -> T
 LL | async fn generic_future<T>(t: T) -> T
    |                                     ^ future returned by `generic_future` is not `Send`
 note: future is not `Send` as this value is used across an await
   --> $DIR/future_not_send.rs:55:5
    |
 LL |     let rt = &t;
---
 
 error: future cannot be sent between threads safely
   --> $DIR/future_not_send.rs:66:34
    |
 LL | async fn unclear_future<T>(t: T) {}
-   |                                  ^
+   |                                  ^ future returned by `unclear_future` is not `Send`
+note: captured value is not `Send`
+  --> $DIR/future_not_send.rs:66:28
+   |
+   |
+LL | async fn unclear_future<T>(t: T) {}
+   |                            ^ has type `T` which is not `Send`
    = note: `T` doesn't implement `std::marker::Send`
 error: aborting due to 8 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-fafe959634cadcc6/out/test_build_base/future_not_send.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-fafe959634cadcc6/out/test_build_base' 'future_not_send.rs'

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/future_not_send.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-fafe959634cadcc6/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-fafe959634cadcc6/out/test_build_base/future_not_send.stage-id" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-725a73e9f9707e2d.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4201b540226d08f1.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-b3bf0c1d531ed936.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-fafe959634cadcc6/out/test_build_base/future_not_send.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":171,"byte_end":175,"line_start":8,"line_end":8,"column_start":62,"column_end":66,"is_primary":true,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":62,"highlight_end":66}],"label":"future returned by `private_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":171,"byte_end":175,"line_start":8,"line_end":8,"column_start":62,"column_end":66,"is_primary":false,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":62,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::future-not-send` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":182,"byte_end":202,"line_start":9,"line_end":9,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    async { true }.await","highlight_start":5,"highlight_end":25}],"label":"await occurs here, with `rc` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":182,"byte_end":202,"line_start":9,"line_end":9,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    async { true }.await","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":203,"byte_end":204,"line_start":10,"line_end":10,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`rc` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":176,"byte_end":204,"line_start":8,"line_end":10,"column_start":67,"column_end":2,"is_primary":false,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":67,"highlight_end":68},{"text":"    async { true }.await","highlight_start":1,"highlight_end":25},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":134,"byte_end":136,"line_start":8,"line_end":8,"column_start":25,"column_end":27,"is_primary":false,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":25,"highlight_end":27}],"label":"has type `std::rc::Rc<[u8]>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":182,"byte_end":202,"line_start":9,"line_end":9,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    async { true }.await","highlight_start":5,"highlight_end":25}],"label":"await occurs here, with `cell` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":182,"byte_end":202,"line_start":9,"line_end":9,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    async { true }.await","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":203,"byte_end":204,"line_start":10,"line_end":10,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`cell` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":176,"byte_end":204,"line_start":8,"line_end":10,"column_start":67,"column_end":2,"is_primary":false,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":67,"highlight_end":68},{"text":"    async { true }.await","highlight_start":1,"highlight_end":25},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":148,"byte_end":152,"line_start":8,"line_end":8,"column_start":39,"column_end":43,"is_primary":false,"text":[{"text":"async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":39,"highlight_end":43}],"label":"has type `&std::cell::Cell<usize>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::cell::Cell<usize>` doesn't implement `std::marker::Sync`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:8:62\n   |\nLL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                                                              ^^^^ future returned by `private_future` is not `Send`\n   |\n   = note: `-D clippy::future-not-send` implied by `-D warnings`\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:9:5\n   |\nLL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                         -- has type `std::rc::Rc<[u8]>` which is not `Send`\nLL |     async { true }.await\n   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later\nLL | }\n   | - `rc` is later dropped here\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:9:5\n   |\nLL | async fn private_future(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                                       ---- has type `&std::cell::Cell<usize>` which is not `Send`\nLL |     async { true }.await\n   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `cell` maybe used later\nLL | }\n   | - `cell` is later dropped here\n   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":247,"byte_end":247,"line_start":12,"line_end":12,"column_start":42,"column_end":42,"is_primary":true,"text":[{"text":"pub async fn public_future(rc: Rc<[u8]>) {","highlight_start":42,"highlight_end":42}],"label":"future returned by `public_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":247,"byte_end":247,"line_start":12,"line_end":12,"column_start":42,"column_end":42,"is_primary":false,"text":[{"text":"pub async fn public_future(rc: Rc<[u8]>) {","highlight_start":42,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":253,"byte_end":273,"line_start":13,"line_end":13,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    async { true }.await;","highlight_start":5,"highlight_end":25}],"label":"await occurs here, with `rc` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":253,"byte_end":273,"line_start":13,"line_end":13,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    async { true }.await;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":275,"byte_end":276,"line_start":14,"line_end":14,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`rc` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":247,"byte_end":276,"line_start":12,"line_end":14,"column_start":42,"column_end":2,"is_primary":false,"text":[{"text":"pub async fn public_future(rc: Rc<[u8]>) {","highlight_start":42,"highlight_end":43},{"text":"    async { true }.await;","highlight_start":1,"highlight_end":26},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":233,"byte_end":235,"line_start":12,"line_end":12,"column_start":28,"column_end":30,"is_primary":false,"text":[{"text":"pub async fn public_future(rc: Rc<[u8]>) {","highlight_start":28,"highlight_end":30}],"label":"has type `std::rc::Rc<[u8]>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:12:42\n   |\nLL | pub async fn public_future(rc: Rc<[u8]>) {\n   |                                          ^ future returned by `public_future` is not `Send`\n   |\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:13:5\n   |\nLL | pub async fn public_future(rc: Rc<[u8]>) {\n   |                            -- has type `std::rc::Rc<[u8]>` which is not `Send`\nLL |     async { true }.await;\n   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later\nLL | }\n   | - `rc` is later dropped here\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":420,"byte_end":424,"line_start":20,"line_end":20,"column_start":63,"column_end":67,"is_primary":true,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":63,"highlight_end":67}],"label":"future returned by `private_future2` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":420,"byte_end":424,"line_start":20,"line_end":20,"column_start":63,"column_end":67,"is_primary":false,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":63,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"captured value is not `Send`","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":383,"byte_end":385,"line_start":20,"line_end":20,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":26,"highlight_end":28}],"label":"has type `std::rc::Rc<[u8]>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":383,"byte_end":385,"line_start":20,"line_end":20,"column_start":26,"column_end":28,"is_primary":false,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"captured value is not `Send`","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":397,"byte_end":401,"line_start":20,"line_end":20,"column_start":40,"column_end":44,"is_primary":true,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":40,"highlight_end":44}],"label":"has type `&std::cell::Cell<usize>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":397,"byte_end":401,"line_start":20,"line_end":20,"column_start":40,"column_end":44,"is_primary":false,"text":[{"text":"async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {","highlight_start":40,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"`std::cell::Cell<usize>` doesn't implement `std::marker::Sync`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:20:63\n   |\nLL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                                                               ^^^^ future returned by `private_future2` is not `Send`\n   |\nnote: captured value is not `Send`\n  --> tests/ui/future_not_send.rs:20:26\n   |\nLL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                          ^^ has type `std::rc::Rc<[u8]>` which is not `Send`\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`\nnote: captured value is not `Send`\n  --> tests/ui/future_not_send.rs:20:40\n   |\nLL | async fn private_future2(rc: Rc<[u8]>, cell: &Cell<usize>) -> bool {\n   |                                        ^^^^ has type `&std::cell::Cell<usize>` which is not `Send`\n   = note: `std::cell::Cell<usize>` doesn't implement `std::marker::Sync`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":481,"byte_end":481,"line_start":24,"line_end":24,"column_start":43,"column_end":43,"is_primary":true,"text":[{"text":"pub async fn public_future2(rc: Rc<[u8]>) {}","highlight_start":43,"highlight_end":43}],"label":"future returned by `public_future2` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":481,"byte_end":481,"line_start":24,"line_end":24,"column_start":43,"column_end":43,"is_primary":false,"text":[{"text":"pub async fn public_future2(rc: Rc<[u8]>) {}","highlight_start":43,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"captured value is not `Send`","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":467,"byte_end":469,"line_start":24,"line_end":24,"column_start":29,"column_end":31,"is_primary":true,"text":[{"text":"pub async fn public_future2(rc: Rc<[u8]>) {}","highlight_start":29,"highlight_end":31}],"label":"has type `std::rc::Rc<[u8]>` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":467,"byte_end":469,"line_start":24,"line_end":24,"column_start":29,"column_end":31,"is_primary":false,"text":[{"text":"pub async fn public_future2(rc: Rc<[u8]>) {}","highlight_start":29,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:24:43\n   |\nLL | pub async fn public_future2(rc: Rc<[u8]>) {}\n   |                                           ^ future returned by `public_future2` is not `Send`\n   |\nnote: captured value is not `Send`\n  --> tests/ui/future_not_send.rs:24:29\n   |\nLL | pub async fn public_future2(rc: Rc<[u8]>) {}\n   |                             ^^ has type `std::rc::Rc<[u8]>` which is not `Send`\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Send`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":637,"byte_end":642,"line_start":35,"line_end":35,"column_start":39,"column_end":44,"is_primary":true,"text":[{"text":"    async fn private_future(&self) -> usize {","highlight_start":39,"highlight_end":44}],"label":"future returned by `private_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":637,"byte_end":642,"line_start":35,"line_end":35,"column_start":39,"column_end":44,"is_primary":false,"text":[{"text":"    async fn private_future(&self) -> usize {","highlight_start":39,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":653,"byte_end":673,"line_start":36,"line_end":36,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"        async { true }.await;","highlight_start":9,"highlight_end":29}],"label":"await occurs here, with `&self` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":653,"byte_end":673,"line_start":36,"line_end":36,"column_start":9,"column_end":29,"is_primary":false,"text":[{"text":"        async { true }.await;","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":701,"byte_end":702,"line_start":38,"line_end":38,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`&self` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":643,"byte_end":702,"line_start":35,"line_end":38,"column_start":45,"column_end":6,"is_primary":false,"text":[{"text":"    async fn private_future(&self) -> usize {","highlight_start":45,"highlight_end":46},{"text":"        async { true }.await;","highlight_start":1,"highlight_end":30},{"text":"        self.rc.len()","highlight_start":1,"highlight_end":22},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":627,"byte_end":632,"line_start":35,"line_end":35,"column_start":29,"column_end":34,"is_primary":false,"text":[{"text":"    async fn private_future(&self) -> usize {","highlight_start":29,"highlight_end":34}],"label":"has type `&Dummy` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:35:39\n   |\nLL |     async fn private_future(&self) -> usize {\n   |                                       ^^^^^ future returned by `private_future` is not `Send`\n   |\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:36:9\n   |\nLL |     async fn private_future(&self) -> usize {\n   |                             ----- has type `&Dummy` which is not `Send`\nLL |         async { true }.await;\n   |         ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later\nLL |         self.rc.len()\nLL |     }\n   |     - `&self` is later dropped here\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":742,"byte_end":742,"line_start":40,"line_end":40,"column_start":39,"column_end":39,"is_primary":true,"text":[{"text":"    pub async fn public_future(&self) {","highlight_start":39,"highlight_end":39}],"label":"future returned by `public_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":742,"byte_end":742,"line_start":40,"line_end":40,"column_start":39,"column_end":39,"is_primary":false,"text":[{"text":"    pub async fn public_future(&self) {","highlight_start":39,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":752,"byte_end":779,"line_start":41,"line_end":41,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"        self.private_future().await;","highlight_start":9,"highlight_end":36}],"label":"await occurs here, with `&self` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":752,"byte_end":779,"line_start":41,"line_end":41,"column_start":9,"column_end":36,"is_primary":false,"text":[{"text":"        self.private_future().await;","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":785,"byte_end":786,"line_start":42,"line_end":42,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    }","highlight_start":5,"highlight_end":6}],"label":"`&self` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":742,"byte_end":786,"line_start":40,"line_end":42,"column_start":39,"column_end":6,"is_primary":false,"text":[{"text":"    pub async fn public_future(&self) {","highlight_start":39,"highlight_end":40},{"text":"        self.private_future().await;","highlight_start":1,"highlight_end":37},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":735,"byte_end":740,"line_start":40,"line_end":40,"column_start":32,"column_end":37,"is_primary":false,"text":[{"text":"    pub async fn public_future(&self) {","highlight_start":32,"highlight_end":37}],"label":"has type `&Dummy` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:40:39\n   |\nLL |     pub async fn public_future(&self) {\n   |                                       ^ future returned by `public_future` is not `Send`\n   |\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:41:9\n   |\nLL |     pub async fn public_future(&self) {\n   |                                ----- has type `&Dummy` which is not `Send`\nLL |         self.private_future().await;\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `&self` maybe used later\nLL |     }\n   |     - `&self` is later dropped here\n   = note: `std::rc::Rc<[u8]>` doesn't implement `std::marker::Sync`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":970,"byte_end":971,"line_start":50,"line_end":50,"column_start":37,"column_end":38,"is_primary":true,"text":[{"text":"async fn generic_future<T>(t: T) -> T","highlight_start":37,"highlight_end":38}],"label":"future returned by `generic_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":970,"byte_end":971,"line_start":50,"line_end":50,"column_start":37,"column_end":38,"is_primary":false,"text":[{"text":"async fn generic_future<T>(t: T) -> T","highlight_start":37,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"future is not `Send` as this value is used across an await","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":1014,"byte_end":1034,"line_start":55,"line_end":55,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    async { true }.await;","highlight_start":5,"highlight_end":25}],"label":"await occurs here, with `rt` maybe used later","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":1014,"byte_end":1034,"line_start":55,"line_end":55,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    async { true }.await;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `await` expression","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/future_not_send.rs","byte_start":1042,"byte_end":1043,"line_start":57,"line_end":57,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`rt` is later dropped here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/future_not_send.rs","byte_start":1001,"byte_end":1003,"line_start":54,"line_end":54,"column_start":9,"column_end":11,"is_primary":false,"text":[{"text":"    let rt = &t;","highlight_start":9,"highlight_end":11}],"label":"has type `&T` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`T` doesn't implement `std::marker::Sync`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:50:37\n   |\nLL | async fn generic_future<T>(t: T) -> T\n   |                                     ^ future returned by `generic_future` is not `Send`\n   |\nnote: future is not `Send` as this value is used across an await\n  --> tests/ui/future_not_send.rs:55:5\n   |\nLL |     let rt = &t;\n   |         -- has type `&T` which is not `Send`\nLL |     async { true }.await;\n   |     ^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rt` maybe used later\nLL |     t\nLL | }\n   | - `rt` is later dropped here\n   = note: `T` doesn't implement `std::marker::Sync`\n\n"}
{"message":"future cannot be sent between threads safely","code":null,"level":"error","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":1166,"byte_end":1166,"line_start":66,"line_end":66,"column_start":34,"column_end":34,"is_primary":true,"text":[{"text":"async fn unclear_future<T>(t: T) {}","highlight_start":34,"highlight_end":34}],"label":"future returned by `unclear_future` is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":1166,"byte_end":1166,"line_start":66,"line_end":66,"column_start":34,"column_end":34,"is_primary":false,"text":[{"text":"async fn unclear_future<T>(t: T) {}","highlight_start":34,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"captured value is not `Send`","code":null,"level":"note","spans":[{"file_name":"tests/ui/future_not_send.rs","byte_start":1160,"byte_end":1161,"line_start":66,"line_end":66,"column_start":28,"column_end":29,"is_primary":true,"text":[{"text":"async fn unclear_future<T>(t: T) {}","highlight_start":28,"highlight_end":29}],"label":"has type `T` which is not `Send`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/future_not_send.rs","byte_start":1160,"byte_end":1161,"line_start":66,"line_end":66,"column_start":28,"column_end":29,"is_primary":false,"text":[{"text":"async fn unclear_future<T>(t: T) {}","highlight_start":28,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `async` block or function","def_site_span":{"file_name":"tests/ui/future_not_send.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null},{"message":"`T` doesn't implement `std::marker::Send`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: future cannot be sent between threads safely\n  --> tests/ui/future_not_send.rs:66:34\n   |\nLL | async fn unclear_future<T>(t: T) {}\n   |                                  ^ future returned by `unclear_future` is not `Send`\n   |\nnote: captured value is not `Send`\n  --> tests/ui/future_not_send.rs:66:28\n   |\nLL | async fn unclear_future<T>(t: T) {}\n   |                            ^ has type `T` which is not `Send`\n   = note: `T` doesn't implement `std::marker::Send`\n\n"}

------------------------------------------

test [ui] ui/future_not_send.rs ... FAILED
---

------------------------------------------
stderr:
------------------------------------------
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::add` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":207,"byte_end":229,"line_start":12,"line_end":12,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        ptr.add(offset_usize);","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-must-use` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::add` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:12:9\n   |\nLL |         ptr.add(offset_usize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unused-must-use` implied by `-D warnings`\n   = note: returns a new pointer rather than modifying its argument\n\n"}
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::offset` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":238,"byte_end":272,"line_start":13,"line_end":13,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"        ptr.offset(offset_isize as isize);","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::offset` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:13:9\n   |\nLL |         ptr.offset(offset_isize as isize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: returns a new pointer rather than modifying its argument\n\n"}
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::offset` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":281,"byte_end":312,"line_start":14,"line_end":14,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"        ptr.offset(offset_u8 as isize);","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::offset` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:14:9\n   |\nLL |         ptr.offset(offset_u8 as isize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: returns a new pointer rather than modifying its argument\n\n"}
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_add` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":322,"byte_end":353,"line_start":16,"line_end":16,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"        ptr.wrapping_add(offset_usize);","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_add` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:16:9\n   |\nLL |         ptr.wrapping_add(offset_usize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: returns a new pointer rather than modifying its argument\n\n"}
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_offset` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":362,"byte_end":405,"line_start":17,"line_end":17,"column_start":9,"column_end":52,"is_primary":true,"text":[{"text":"        ptr.wrapping_offset(offset_isize as isize);","highlight_start":9,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_offset` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:17:9\n   |\nLL |         ptr.wrapping_offset(offset_isize as isize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: returns a new pointer rather than modifying its argument\n\n"}
{"message":"unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_offset` that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ptr_offset_with_cast.fixed","byte_start":414,"byte_end":454,"line_start":18,"line_end":18,"column_start":9,"column_end":49,"is_primary":true,"text":[{"text":"        ptr.wrapping_offset(offset_u8 as isize);","highlight_start":9,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"returns a new pointer rather than modifying its argument","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused return value of `std::ptr::const_ptr::<impl *const T>::wrapping_offset` that must be used\n  --> tests/ui/ptr_offset_with_cast.fixed:18:9\n   |\nLL |         ptr.wrapping_offset(offset_u8 as isize);\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: returns a new pointer rather than modifying its argument\n\n"}

------------------------------------------

test [ui] ui/ptr_offset_with_cast.rs ... FAILED
---
test [ui] ui/rest_pat_in_fully_bound_structs.rs ... ok
test [ui] ui/result_map_or_into_option.rs ... ok
test [ui] ui/result_map_unit_fn_fixable.rs ... ok
test [ui] ui/result_map_unit_fn_unfixable.rs ... ok
test [ui] ui/reversed_empty_ranges_loops_fixable.rs ... ok
test [ui] ui/reversed_empty_ranges_fixable.rs ... ok
test [ui] ui/reversed_empty_ranges_loops_unfixable.rs ... ok
test [ui] ui/reversed_empty_ranges_unfixable.rs ... ok
test [ui] ui/serde.rs ... ok
test [ui] ui/shadow.rs ... ok
test [ui] ui/similar_names.rs ... ok
test [ui] ui/short_circuit_statement.rs ... ok
---
   Compiling rustc-ap-rustc_parse v659.0.0
error: environment variable `CFG_RELEASE` not defined
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_attr-659.0.0/builtin.rs:657:48
    |
657 |             let rustc_version = Version::parse(env!("CFG_RELEASE")).unwrap();

error: aborting due to previous error

error: could not compile `rustc-ap-rustc_attr`.
---
   Compiling rustc-ap-rustc_attr v659.0.0
error: environment variable `CFG_RELEASE` not defined
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_attr-659.0.0/builtin.rs:657:48
    |
657 |             let rustc_version = Version::parse(env!("CFG_RELEASE")).unwrap();

error: aborting due to previous error

error: could not compile `rustc-ap-rustc_attr`.
---
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-d3177702824c042a
## Running run-pass tests in tests/run-pass against miri for target x86_64-unknown-linux-gnu
   Compiler flags: --edition 2018 -Astable-features --sysroot /home/user/.cache/miri/HOST
running 190 tests
test [ui] run-pass/args.rs ... ok
normalized stdout:

---
-"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
-

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestPp3ADk/align_offset.stdout
normalized stderr:
error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---


diff of stderr:

+error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
+thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
+note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -C prefer-dynamic
+
+
+error: aborting due to previous error
+
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/align_offset.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'align_offset.rs'
error: 2 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/align_offset.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/align_offset.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/align_offset.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits\n\n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---
test [ui] run-pass/closure-field-ty.rs ... ok
test [ui] run-pass/coerce_non_capture_closure_to_fn_ptr.rs ... ok
test [ui] run-pass/closures.rs ... ok
test [ui] run-pass/coercions.rs ... ok
test [ui] run-pass/concurrency/locks.rs ... ok
test [ui] run-pass/concurrency/thread_locals.rs ... ok
test [ui] run-pass/concurrency/simple.rs ... ok
test [ui] run-pass/concurrency/tls_lib_drop_single_thread.rs ... ok
test [ui] run-pass/concurrency/tls_lib_drop.rs ... ok
test [ui] run-pass/constants.rs ... ok
test [ui] run-pass/current_dir.rs ... ok
test [ui] run-pass/deriving-associated-types.rs ... ok
test [ui] run-pass/disable-alignment-check.rs ... ok
---
-[]
 

The actual stdout differed from the expected stdout.
Actual stdout saved to /tmp/compiletestPp3ADk/env.stdout
normalized stderr:
error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---


diff of stderr:

+error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
+thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
+note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -C prefer-dynamic
+
+
+error: aborting due to previous error
+
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/env.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'env.rs'
error: 2 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/env.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/env.stage-id.aux"
------------------------------------------
[]
[
    (
    (

------------------------------------------
stderr:
------------------------------------------
{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits\n\n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---
+ right: `6`', $DIR/heap.rs:17:5
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/heap.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'heap.rs'
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/heap.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/heap.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
test [ui] run-pass/ints.rs ... ok
test [ui] run-pass/issue-15063.rs ... ok
test [ui] run-pass/issue-15080.rs ... ok
normalized stderr:
error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---


diff of stderr:

+error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
+thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
+note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -C prefer-dynamic
+
+
+error: aborting due to previous error
+
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/issue-15523-big.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'issue-15523-big.rs'
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/issue-15523-big.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/issue-15523-big.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits\n\n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---
test [ui] run-pass/option_box_transmute_ptr.rs ... ok
test [ui] run-pass/option_eq.rs ... ok
test [ui] run-pass/overloaded-calls-simple.rs ... ok
normalized stderr:
error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---


diff of stderr:

+error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits
+thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+
+note: the compiler unexpectedly panicked. this is a bug.
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
+note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -C prefer-dynamic
+
+
+error: aborting due to previous error
+
+

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/packed_struct.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'packed_struct.rs'
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/packed_struct.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/packed_struct.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits\n\n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---
test [ui] run-pass/stacked-borrows/2phase.rs ... ok
test [ui] run-pass/stacked-borrows/interior_mutability.rs ... ok
test [ui] run-pass/stacked-borrows/refcell.rs ... ok
normalized stderr:
"{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits/n/n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---


diff of stderr:

-"hello world" [0, 1, 2, 4]
+"{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits/n/n"}
+note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+
+note: the compiler unexpectedly panicked. this is a bug.
+
+
+note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
+note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -C prefer-dynamic
+
+
+error: aborting due to previous error
+
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestPp3ADk/stacked-borrows/stacked-borrows.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestPp3ADk' 'stacked-borrows/stacked-borrows.rs'
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestPp3ADk" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPp3ADk/stacked-borrows/stacked-borrows.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPp3ADk/stacked-borrows/stacked-borrows.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
"{"message":"src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_middle/mir/interpret/value.rs:327: Signed value 0xffffffffffffffff does not fit in 64 bits\n\n"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.45.0-nightly (55eaaee88 2020-05-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C prefer-dynamic

---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
Build completed unsuccessfully in 0:00:01
== clock drift check ==
  local time: Thu May 21 16:35:03 UTC 2020
  network time: Thu, 21 May 2020 16:35:04 GMT
  network time: Thu, 21 May 2020 16:35:04 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72423/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72423/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (11218) (python)
##[section]Finishing: Finalize Job
