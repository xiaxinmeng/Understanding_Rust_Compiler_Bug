plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:16c3eff9
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:19:25] -- Looking for dlopen in dl
[00:19:25] -- Looking for dlopen in dl - found
[00:19:25] -- Looking for clock_gettime in rt
[00:19:26] -- Looking for clock_gettime in rt - found
[00:19:26] -- Looking for pfm_initialize in pfm
[00:19:26] -- Looking for pfm_initialize in pfm - not found
[00:19:26] -- Looking for pthread.h - found
[00:19:26] -- Looking for pthread_create
[00:19:26] -- Looking for pthread_create - not found
[00:19:26] -- Looking for pthread_create in pthreads
---
[00:19:53] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstPrinter.cpp.o
[00:19:53] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/GlobalISelEmitter.cpp.o
[00:19:53] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.o
[00:19:54] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstrAnalysis.cpp.o
[00:19:54] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DJB.cpp.o
[00:19:54] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCInstrDesc.cpp.o
[00:19:54] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.o
[00:19:54] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InstrInfoEmitter.cpp.o
[00:19:54] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCLabel.cpp.o
---
[00:19:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.o
[00:19:56] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCObjectFileInfo.cpp.o
[00:19:56] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FoldingSet.cpp.o
[00:19:56] [  5%] Building CXX object lib/MC/MCDisassembler/CMakeFiles/LLVMMCDisassembler.dir/MCExternalSymbolizer.cpp.o
[00:19:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RISCVCompressInstEmitter.cpp.o
[00:19:56] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormattedStream.cpp.o
[00:19:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterBankEmitter.cpp.o
[00:19:56] [  5%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCObjectWriter.cpp.o
[00:19:57] [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FormatVariadic.cpp.o
---
[00:20:08] [ 10%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Triple.cpp.o
[00:20:08] [ 10%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/CodeViewYAMLTypeHashing.cpp.o
[00:20:08] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Twine.cpp.o
[00:20:09] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Unicode.cpp.o
[00:20:09] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeCaseFold.cpp.o
[00:20:09] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VersionTuple.cpp.o
[00:20:09] [ 11%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/OptTable.cpp.o
[00:20:10] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/WithColor.cpp.o
[00:20:10] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLTraits.cpp.o
[00:20:11] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_os_ostream.cpp.o
[00:20:11] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/raw_ostream.cpp.o
[00:20:11] [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/CodeViewYAMLTypes.cpp.o
---
[00:20:24] [ 12%] Linking CXX static library ../libLLVMSupport.a
[00:20:24] [ 12%] Built target LLVMSupport
[00:20:24] Scanning dependencies of target LLVMDebugInfoMSF
[00:20:24] [ 12%] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MappedBlockStream.cpp.o
[00:20:26] [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAddressRange.cpp.o
[00:20:27] [ 12%] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFCommon.cpp.o
[00:20:28] [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAcceleratorTable.cpp.o
[00:20:29] [ 12%] Building CXX object lib/DebugInfo/MSF/CMakeFiles/LLVMDebugInfoMSF.dir/MSFError.cpp.o
[00:20:30] Scanning dependencies of target LLVMDebugInfoCodeView
---
[00:21:06] [ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/Line.cpp.o
[00:21:07] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolLabel.cpp.o
[00:21:07] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolPublicSymbol.cpp.o
[00:21:07] [ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/MergingTypeTableBuilder.cpp.o
[00:21:09] [ 15%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDebugRnglists.cpp.o
[00:21:09] [ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordName.cpp.o
[00:21:10] [ 15%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordSerialization.cpp.o
[00:21:11] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeArray.cpp.o
[00:21:12] [ 15%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDie.cpp.o
---
[00:21:50] [ 19%] Built target AttributeCompatFuncTableGen
[00:21:50] Scanning dependencies of target intrinsics_gen
[00:21:50] [ 20%] Building Attributes.inc...
[00:21:50] [ 21%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[00:21:50] [ 21%] Building IntrinsicEnums.inc...
[00:21:50] [ 21%] Building IntrinsicImpl.inc...
[00:21:51] [ 21%] Updating Attributes.inc...
[00:21:51] [ 21%] Updating IntrinsicEnums.inc...
[00:21:51] [ 21%] Updating IntrinsicImpl.inc...
[00:21:51] [ 21%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStream.cpp.o
[00:21:52] [ 21%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStreamBuilder.cpp.o
[00:21:52] [ 21%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[00:21:52] [ 21%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeBuiltinSymbol.cpp.o
---
[00:23:27] [ 26%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/StripDeadPrototypes.cpp.o
[00:23:30] [ 26%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/StripSymbols.cpp.o
[00:23:30] [ 26%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopSink.cpp.o
[00:23:32] [ 26%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopDeletion.cpp.o
[00:23:35] [ 26%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/SyntheticCountsPropagation.cpp.o
[00:23:37] [ 26%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/ThinLTOBitcodeWriter.cpp.o
[00:23:37] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopDistribute.cpp.o
[00:23:39] [ 27%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/WholeProgramDevirt.cpp.o
[00:23:41] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopIdiomRecognize.cpp.o
---
[00:24:01] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopRotation.cpp.o
[00:24:03] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopSimplifyCFG.cpp.o
[00:24:06] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopStrengthReduce.cpp.o
[00:24:08] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopUnrollPass.cpp.o
[00:24:09] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopUnrollAndJamPass.cpp.o
[00:24:18] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopVersioningLICM.cpp.o
[00:24:23] [ 27%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/SLPVectorizer.cpp.o
[00:24:24] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerAtomic.cpp.o
[00:24:25] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerExpectIntrinsic.cpp.o
[00:24:25] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerExpectIntrinsic.cpp.o
[00:24:27] [ 27%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/Vectorize.cpp.o
[00:24:28] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerGuardIntrinsic.cpp.o
[00:24:28] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/MemCpyOptimizer.cpp.o
[00:24:28] [ 27%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlan.cpp.o
[00:24:32] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/MergeICmps.cpp.o
[00:24:35] [ 27%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanHCFGBuilder.cpp.o
[00:24:38] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/NaryReassociate.cpp.o
[00:24:38] [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/NaryReassociate.cpp.o
[00:24:39] [ 27%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanHCFGTransforms.cpp.o
[00:24:44] [ 28%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/PartiallyInlineLibCalls.cpp.o
[00:24:45] [ 29%] Building CXX object lib/Transforms/Vectorize/CMakeFiles/LLVMVectorize.dir/VPlanVerifier.cpp.o
[00:24:47] Scanning dependencies of target LLVMHello
[00:24:47] [ 29%] Building CXX object lib/Transforms/Hello/CMakeFiles/LLVMHello.dir/Hello.cpp.o
---
[00:28:20] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MemorySSA.cpp.o
[00:28:21] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MemorySSAUpdater.cpp.o
[00:28:24] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModuleDebugInfoPrinter.cpp.o
[00:28:26] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ModuleSummaryAnalysis.cpp.o
[00:28:27] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/MustExecute.cpp.o
[00:28:31] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ObjCARCAnalysisUtils.cpp.o
[00:28:32] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ObjCARCInstKind.cpp.o
[00:28:34] [ 35%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/OptimizationRemarkEmitter.cpp.o
[00:28:35] [ 35%] Building X86GenDisassemblerTables.inc...
---
[00:29:02] [ 37%] Updating X86GenSubtargetInfo.inc...
[00:29:02] [ 37%] Built target X86CommonTableGen
[00:29:02] Scanning dependencies of target ARMCommonTableGen
[00:29:02] [ 38%] Building ARMGenAsmMatcher.inc...
[00:29:02] [ 38%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/SyntheticCountsUtils.cpp.o
[00:29:03] [ 38%] Building ARMGenCallingConv.inc...
[00:29:03] [ 38%] Building ARMGenDAGISel.inc...
[00:29:04] [ 38%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/TargetLibraryInfo.cpp.o
[00:29:05] [ 38%] Building ARMGenDisassemblerTables.inc...
---
[00:30:05] [ 44%] Updating Opts.inc...
[00:30:05] [ 44%] Built target LLVMXRay
[00:30:05] [ 44%] Built target CvtResTableGen
[00:30:05] Scanning dependencies of target MtTableGen
[00:30:05] Scanning dependencies of target ObjcopyOptsTableGen
[00:30:05] [ 45%] Building ObjcopyOpts.inc...
[00:30:05] [ 45%] Updating Opts.inc...
[00:30:05] [ 45%] Updating ObjcopyOpts.inc...
[00:30:05] [ 45%] Built target MtTableGen
[00:30:05] [ 45%] Built target MtTableGen
[00:30:05] [ 45%] Built target ObjcopyOptsTableGen
[00:30:05] [ 45%] Building CXX object lib/Passes/CMakeFiles/LLVMPasses.dir/PassPlugin.cpp.o
[00:30:05] [ 46%] Building Opts.inc...
[00:30:05] [ 46%] Updating Opts.inc...
[00:30:05] [ 46%] Built target RcTableGen
[00:30:06] Scanning dependencies of target LLVMCore
---
[00:31:38] [ 49%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/BranchFolding.cpp.o
[00:31:41] [ 49%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/FunctionLoweringInfo.cpp.o
[00:31:41] [ 49%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/BranchRelaxation.cpp.o
[00:31:46] [ 50%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/InstrEmitter.cpp.o
[00:31:47] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/BreakFalseDeps.cpp.o
[00:31:50] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/CalcSpillWeights.cpp.o
[00:31:52] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/CallingConvLower.cpp.o
[00:31:53] [ 51%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/LegalizeDAG.cpp.o
[00:31:56] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/CFIInstrInserter.cpp.o
---
[00:32:52] [ 51%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/SelectionDAGTargetInfo.cpp.o
[00:32:52] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/EdgeBundles.cpp.o
[00:32:54] [ 51%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/StatepointLowering.cpp.o
[00:32:55] [ 51%] Building CXX object lib/CodeGen/SelectionDAG/CMakeFiles/LLVMSelectionDAG.dir/TargetLowering.cpp.o
[00:32:56] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExecutionDomainFix.cpp.o
[00:33:02] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExpandMemCmp.cpp.o
[00:33:02] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExpandPostRAPseudos.cpp.o
[00:33:02] [ 51%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ExpandReductions.cpp.o
[00:33:06] [ 51%] Linking CXX static library ../../libLLVMSelectionDAG.a
---
[00:34:05] [ 53%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/EHStreamer.cpp.o
[00:34:05] [ 53%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/ErlangGCPrinter.cpp.o
[00:34:10] [ 53%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LiveIntervalUnion.cpp.o
[00:34:10] [ 53%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/OcamlGCPrinter.cpp.o
[00:34:10] [ 53%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/WinCFGuard.cpp.o
[00:34:15] [ 53%] Building CXX object lib/CodeGen/AsmPrinter/CMakeFiles/LLVMAsmPrinter.dir/CodeViewDebug.cpp.o
[00:34:15] [ 53%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LivePhysRegs.cpp.o
[00:34:15] [ 53%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LiveRangeCalc.cpp.o
[00:34:17] [ 53%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LiveRangeEdit.cpp.o
---
[00:36:51] [ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/PseudoSourceValue.cpp.o
[00:36:51] [ 57%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CtorUtils.cpp.o
[00:36:54] [ 57%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/DemoteRegToStack.cpp.o
[00:36:55] [ 57%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EntryExitInstrumenter.cpp.o
[00:36:56] [ 57%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ReachingDefAnalysis.cpp.o
[00:36:58] [ 57%] Linking CXX static library ../../libLLVMBitWriter.a
[00:36:58] [ 57%] Built target LLVMBitWriter
[00:36:58] [ 57%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Evaluator.cpp.o
[00:36:59] [ 57%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FlattenCFG.cpp.o
---
[00:37:16] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopRotationUtils.cpp.o
[00:37:17] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopSimplify.cpp.o
[00:37:18] [ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocGreedy.cpp.o
[00:37:23] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopUnroll.cpp.o
[00:37:24] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopUnrollAndJam.cpp.o
[00:37:29] [ 58%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegAllocPBQP.cpp.o
[00:37:32] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopUnrollRuntime.cpp.o
[00:37:32] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopUtils.cpp.o
[00:37:32] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopVersioning.cpp.o
---
[00:37:48] [ 58%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/PromoteMemoryToRegister.cpp.o
[00:37:49] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/StripGCRelocates.cpp.o
[00:37:52] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/SSAUpdater.cpp.o
[00:37:54] [ 59%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterPressure.cpp.o
[00:37:55] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/SSAUpdaterBulk.cpp.o
[00:37:57] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/SimplifyCFG.cpp.o
[00:37:59] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/SimplifyIndVar.cpp.o
[00:38:00] [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/SimplifyLibCalls.cpp.o
[00:38:00] [ 59%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/RegisterScavenging.cpp.o
---
[00:38:43] [ 61%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/ThreadSanitizer.cpp.o
[00:38:46] [ 61%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/EfficiencySanitizer.cpp.o
[00:38:47] [ 61%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/SafeStackColoring.cpp.o
[00:38:48] [ 61%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/HWAddressSanitizer.cpp.o
[00:38:50] Scanning dependencies of target LLVMAggressiveInstCombine
[00:38:50] [ 61%] Building CXX object lib/Transforms/AggressiveInstCombine/CMakeFiles/LLVMAggressiveInstCombine.dir/AggressiveInstCombine.cpp.o
[00:38:51] [ 61%] Building CXX object lib/Transforms/AggressiveInstCombine/CMakeFiles/LLVMAggressiveInstCombine.dir/TruncInstCombine.cpp.o
[00:38:53] [ 61%] Linking CXX static library ../../libLLVMInstrumentation.a
[00:38:53] [ 61%] Built target LLVMInstrumentation
[00:38:53] Scanning dependencies of target LLVMInstCombine
[00:38:53] [ 61%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstructionCombining.cpp.o
[00:38:53] [ 61%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstructionCombining.cpp.o
[00:38:55] [ 61%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ScalarizeMaskedMemIntrin.cpp.o
[00:38:55] [ 61%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ScheduleDAG.cpp.o
[00:38:55] [ 61%] Linking CXX static library ../../libLLVMAggressiveInstCombine.a
[00:38:55] [ 61%] Built target LLVMAggressiveInstCombine
[00:38:56] [ 62%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/ShadowCallStack.cpp.o
[00:38:59] [ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ScheduleDAGInstrs.cpp.o
[00:39:01] [ 62%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/ScheduleDAGPrinter.cpp.o
[00:39:03] [ 62%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86AsmPrinter.cpp.o
---
[00:39:32] [ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/SpillPlacement.cpp.o
[00:39:36] [ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/SplitKit.cpp.o
[00:39:38] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupLEAs.cpp.o
[00:39:38] [ 63%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCompares.cpp.o
[00:39:44] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86AvoidStoreForwardingBlocks.cpp.o
[00:39:50] [ 63%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineLoadStoreAlloca.cpp.o
[00:39:51] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupSetCC.cpp.o
[00:39:51] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FixupSetCC.cpp.o
[00:39:52] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FlagsCopyLowering.cpp.o
[00:39:57] [ 63%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineMulDivRem.cpp.o
[00:39:57] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FloatingPoint.cpp.o
[00:39:59] [ 63%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86FrameLowering.cpp.o
[00:39:59] [ 63%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/StackMaps.cpp.o
---
[00:40:20] [ 64%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TailDuplication.cpp.o
[00:40:21] [ 64%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineSimplifyDemanded.cpp.o
[00:40:24] [ 64%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TailDuplicator.cpp.o
[00:40:27] [ 64%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineVectorOps.cpp.o
[00:40:27] [ 64%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86IndirectBranchTracking.cpp.o
[00:40:32] [ 64%] Linking CXX static library ../../libLLVMInstCombine.a
[00:40:33] [ 64%] Built target LLVMInstCombine
[00:40:33] Scanning dependencies of target LLVMX86AsmParser
[00:40:33] [ 64%] Building CXX object lib/Target/X86/AsmParser/CMakeFiles/LLVMX86AsmParser.dir/X86AsmInstrumentation.cpp.o
---
[00:42:33] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMISelDAGToDAG.cpp.o
[00:42:34] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMISelLowering.cpp.o
[00:42:34] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMInstrInfo.cpp.o
[00:42:38] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMLegalizerInfo.cpp.o
[00:42:40] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMParallelDSP.cpp.o
[00:42:45] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMCInstLower.cpp.o
[00:42:49] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMachineFunctionInfo.cpp.o
[00:42:52] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMacroFusion.cpp.o
[00:42:55] [ 67%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMRegisterInfo.cpp.o
---
[00:45:38] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFrameLowering.cpp.o
[00:45:40] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsLegalizerInfo.cpp.o
[00:45:42] [ 72%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsABIFlagsSection.cpp.o
[00:45:44] [ 72%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsAsmBackend.cpp.o
[00:45:44] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsBranchExpansion.cpp.o
[00:45:46] [ 72%] Building CXX object lib/Target/Mips/MCTargetDesc/CMakeFiles/LLVMMipsDesc.dir/MipsELFStreamer.cpp.o
[00:45:47] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsMCInstLower.cpp.o
[00:45:47] Scanning dependencies of target LLVMMipsInfo
[00:45:47] [ 72%] Building CXX object lib/Target/Mips/TargetInfo/CMakeFiles/LLVMMipsInfo.dir/MipsTargetInfo.cpp.o
---
[00:49:52] [ 83%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyCFGStackify.cpp.o
[00:49:53] [ 83%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCCompound.cpp.o
[00:49:55] [ 83%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyCFGSort.cpp.o
[00:49:56] [ 83%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonCFGOptimizer.cpp.o
[00:49:58] [ 83%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyLateEHPrepare.cpp.o
[00:50:01] [ 83%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyExceptionInfo.cpp.o
[00:50:01] [ 83%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCELFStreamer.cpp.o
[00:50:01] [ 83%] Building CXX object lib/Target/Hexagon/CMakeFiles/LLVMHexagonCodeGen.dir/HexagonCommonGEP.cpp.o
[00:50:04] [ 83%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyExplicitLocals.cpp.o
---
[00:52:59] [ 89%] Built target llvm-dis
[00:52:59] [ 89%] Building CXX object tools/llvm-dwp/CMakeFiles/llvm-dwp.dir/DWPError.cpp.o
[00:53:00] [ 89%] Linking CXX executable ../../bin/llvm-diff
[00:53:00] [ 89%] Built target llvm-diff
[00:53:00] Scanning dependencies of target LLVMExegesis
[00:53:00] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Analysis.cpp.o
[00:53:01] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Assembler.cpp.o
[00:53:02] [ 89%] Built target llvm-dwarfdump
[00:53:02] Scanning dependencies of target llvm-extract
[00:53:02] [ 89%] Building CXX object tools/llvm-extract/CMakeFiles/llvm-extract.dir/llvm-extract.cpp.o
[00:53:05] [ 89%] Linking CXX executable ../../bin/llvm-dwp
[00:53:05] [ 89%] Linking CXX executable ../../bin/llvm-dwp
[00:53:06] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/BenchmarkResult.cpp.o
[00:53:07] [ 89%] Linking CXX executable ../../bin/llvm-extract
[00:53:07] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/BenchmarkRunner.cpp.o
[00:53:08] Scanning dependencies of target llvm-isel-fuzzer
[00:53:08] [ 89%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/DummyISelFuzzer.cpp.o
[00:53:08] [ 89%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/DummyISelFuzzer.cpp.o
[00:53:09] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Clustering.cpp.o
[00:53:09] Scanning dependencies of target llvm-link
[00:53:09] [ 89%] Building CXX object tools/llvm-link/CMakeFiles/llvm-link.dir/llvm-link.cpp.o
[00:53:09] [ 89%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/llvm-isel-fuzzer.cpp.o
[00:53:09] [ 89%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/llvm-isel-fuzzer.cpp.o
[00:53:12] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Latency.cpp.o
[00:53:14] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/LlvmState.cpp.o
[00:53:16] [ 89%] Built target llvm-link
[00:53:16] Scanning dependencies of target llvm-lto2
[00:53:16] [ 89%] Building CXX object tools/llvm-lto2/CMakeFiles/llvm-lto2.dir/llvm-lto2.cpp.o
[00:53:17] [ 89%] Linking CXX executable ../../bin/llvm-isel-fuzzer
[00:53:17] [ 89%] Linking CXX executable ../../bin/llvm-isel-fuzzer
[00:53:18] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/MCInstrDescView.cpp.o
[00:53:19] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/PerfHelper.cpp.o
[00:53:20] [ 89%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterAliasing.cpp.o
[00:53:20] [ 90%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Target.cpp.o
[00:53:21] Scanning dependencies of target llvm-mc
[00:53:21] [ 90%] Building CXX object tools/llvm-mc/CMakeFiles/llvm-mc.dir/llvm-mc.cpp.o
[00:53:21] [ 90%] Building CXX object tools/llvm-mc/CMakeFiles/llvm-mc.dir/llvm-mc.cpp.o
[00:53:22] [ 90%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Uops.cpp.o
[00:53:25] Scanning dependencies of target llvm-mca
[00:53:25] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/CodeRegion.cpp.o
[00:53:27] [ 91%] Building CXX object tools/llvm-mc/CMakeFiles/llvm-mc.dir/Disassembler.cpp.o
[00:53:27] [ 91%] Building CXX object tools/llvm-mc/CMakeFiles/llvm-mc.dir/Disassembler.cpp.o
[00:53:27] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/DispatchStage.cpp.o
[00:53:28] [ 91%] Linking CXX static library ../../../lib/libLLVMExegesis.a
[00:53:28] [ 91%] Built target LLVMExegesis
[00:53:28] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/DispatchStatistics.cpp.o
[00:53:29] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/ExecuteStage.cpp.o
[00:53:29] [ 91%] Built target llvm-lto2
[00:53:29] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/FetchStage.cpp.o
[00:53:30] [ 91%] Building CXX object tools/llvm-modextract/CMakeFiles/llvm-modextract.dir/llvm-modextract.cpp.o
[00:53:30] [ 91%] Linking CXX executable ../../bin/llvm-mc
[00:53:30] [ 91%] Built target llvm-mc
[00:53:30] Scanning dependencies of target llvm-mt
[00:53:30] Scanning dependencies of target llvm-mt
[00:53:30] [ 91%] Building CXX object tools/llvm-mt/CMakeFiles/llvm-mt.dir/llvm-mt.cpp.o
[00:53:31] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/HWEventListener.cpp.o
[00:53:31] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/InstrBuilder.cpp.o
[00:53:33] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Instruction.cpp.o
[00:53:33] [ 91%] Built target llvm-mt
[00:53:33] [ 91%] Linking CXX executable ../../bin/llvm-modextract
[00:53:33] Scanning dependencies of target llvm-nm
[00:53:33] [ 91%] Building CXX object tools/llvm-nm/CMakeFiles/llvm-nm.dir/llvm-nm.cpp.o
[00:53:33] [ 91%] Building CXX object tools/llvm-nm/CMakeFiles/llvm-nm.dir/llvm-nm.cpp.o
[00:53:34] [ 91%] Built target llvm-modextract
[00:53:34] Scanning dependencies of target StripOptsTableGen
[00:53:34] [ 91%] Building StripOpts.inc...
[00:53:34] [ 91%] Updating StripOpts.inc...
[00:53:34] [ 91%] Built target StripOptsTableGen
[00:53:34] [ 91%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/llvm-objdump.cpp.o
[00:53:34] [ 91%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/llvm-objdump.cpp.o
[00:53:34] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/InstructionInfoView.cpp.o
[00:53:35] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/InstructionTables.cpp.o
[00:53:36] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/LSUnit.cpp.o
[00:53:37] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/llvm-mca.cpp.o
[00:53:38] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Pipeline.cpp.o
[00:53:40] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/PipelinePrinter.cpp.o
[00:53:42] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/RegisterFile.cpp.o
[00:53:42] [ 91%] Linking CXX executable ../../bin/llvm-nm
[00:53:43] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/RegisterFileStatistics.cpp.o
[00:53:43] Scanning dependencies of target llvm-opt-fuzzer
[00:53:43] [ 91%] Building CXX object tools/llvm-opt-fuzzer/CMakeFiles/llvm-opt-fuzzer.dir/DummyOptFuzzer.cpp.o
[00:53:44] [ 91%] Building CXX object tools/llvm-opt-fuzzer/CMakeFiles/llvm-opt-fuzzer.dir/llvm-opt-fuzzer.cpp.o
[00:53:44] [ 91%] Building CXX object tools/llvm-opt-fuzzer/CMakeFiles/llvm-opt-fuzzer.dir/llvm-opt-fuzzer.cpp.o
[00:53:44] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/ResourcePressureView.cpp.o
[00:53:45] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/RetireControlUnit.cpp.o
[00:53:46] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/RetireControlUnitStatistics.cpp.o
[00:53:47] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/RetireStage.cpp.o
[00:53:47] [ 91%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/COFFDump.cpp.o
[00:53:48] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Scheduler.cpp.o
[00:53:48] [ 91%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/SchedulerStatistics.cpp.o
[00:53:50] [ 92%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Stage.cpp.o
[00:53:51] [ 92%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/ELFDump.cpp.o
[00:53:51] [ 92%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Support.cpp.o
[00:53:52] [ 92%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/SummaryView.cpp.o
[00:53:53] [ 92%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/TimelineView.cpp.o
[00:53:54] [ 92%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/View.cpp.o
[00:53:55] [ 92%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/WasmDump.cpp.o
[00:53:55] [ 93%] Linking CXX executable ../../bin/llvm-opt-fuzzer
[00:53:55] [ 93%] Linking CXX executable ../../bin/llvm-mca
[00:53:56] [ 93%] Built target llvm-mca
---
[00:54:00] Scanning dependencies of target llvm-rc
[00:54:00] [ 93%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/llvm-rc.cpp.o
[00:54:01] [ 93%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/DumpOutputStyle.cpp.o
[00:54:03] [ 93%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceFileWriter.cpp.o
[00:54:04] [ 93%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/ExplainOutputStyle.cpp.o
[00:54:08] [ 93%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptCppFilter.cpp.o
[00:54:09] [ 93%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[00:54:14] [ 93%] Linking CXX executable ../../bin/llvm-objdump
[00:54:14] [ 93%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/llvm-pdbutil.cpp.o
[00:54:15] [ 93%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
---
[00:56:19] [ 99%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/debuginfo.c.o
[00:56:19] [ 99%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/diagnostic.c.o
[00:56:19] [ 99%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/disassemble.c.o
[00:56:19] [ 99%] Building CXX object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/echo.cpp.o
[00:56:21] [ 99%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/NonRelocatableStringpool.cpp.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/include-all.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/main.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/module.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/metadata.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/metadata.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/object.c.o
[00:56:22] [100%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/targets.c.o
[00:56:23] Scanning dependencies of target llvm-cfi-verify
[00:56:23] [100%] Building CXX object tools/llvm-cfi-verify/CMakeFiles/llvm-cfi-verify.dir/llvm-cfi-verify.cpp.o
[00:56:23] [100%] Linking CXX executable ../../bin/llvm-c-test
[00:56:23] Scanning dependencies of target LLVMExegesisX86
[00:56:23] [100%] Building CXX object tools/llvm-exegesis/lib/X86/CMakeFiles/LLVMExegesisX86.dir/Target.cpp.o
[00:56:24] [100%] Linking CXX executable ../../bin/dsymutil
[00:56:26] Scanning dependencies of target llvm-objcopy
[00:56:26] [100%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/llvm-objcopy.cpp.o
[00:56:27] [100%] Linking CXX executable ../../bin/llvm-cfi-verify
[00:56:28] [100%] Built target dsymutil
---
[00:56:28] [100%] Generating ../../bin/llvm-ranlib
[00:56:28] [100%] Built target llvm-ranlib
[00:56:28] Scanning dependencies of target BugpointPasses
[00:56:28] [100%] Building CXX object tools/bugpoint-passes/CMakeFiles/BugpointPasses.dir/TestPasses.cpp.o
[00:56:28] [100%] Linking CXX static library ../../../../lib/libLLVMExegesisX86.a
[00:56:28] [100%] Built target LLVMExegesisX86
[00:56:28] [100%] Built target llvm-cfi-verify
[00:56:28] [100%] Built target llvm-cfi-verify
[00:56:28] Scanning dependencies of target llvm-exegesis
[00:56:28] [100%] Building CXX object tools/llvm-exegesis/CMakeFiles/llvm-exegesis.dir/llvm-exegesis.cpp.o
[00:56:31] [100%] Built target BugpointPasses
[00:56:31] [100%] Built target BugpointPasses
[00:56:33] [100%] Linking CXX executable ../../bin/llvm-exegesis
[00:56:35] [100%] Built target llvm-exegesis
[00:56:37] [100%] Built target llvm-objcopy
[00:56:37] Scanning dependencies of target llvm-strip
[00:56:37] [100%] Generating ../../bin/llvm-strip
[00:56:37] [100%] Built target llvm-strip
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/X86DisassemblerDecoderCommon.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SourceMgr.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/RWMutex.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Debug.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/WithColor.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Recycler.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ToolOutputFile.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TarWriter.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ErrorOr.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Errc.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScaledNumber.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/ScopedPrinter.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryStreamError.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/InitLLVM.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/PrettyStackTrace.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/TargetSelect.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BranchProbability.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/CheckedArithmetic.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MemAlloc.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Endian.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileSystem.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MathExtras.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AMDHSAKernelDescriptor.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/SwapByteOrder.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/AtomicOrdering.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LEB128.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/GlobPattern.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/LegalizerInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/GlobalISel/MachineIRBuilder.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachinePostDominators.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineInstr.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ExecutionDomainFix.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineConstantPool.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/RegAllocRegistry.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineOptimizationRemarkEmitter.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachORelocation.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CommandFlags.inc
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CalcSpillWeights.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineModuleInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetLoweringObjectFileImpl.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ReachingDefAnalysis.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineBlockFrequencyInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAG.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/AccelTable.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetInstrInfo.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetRegisterInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PseudoSourceValue.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CostTable.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveRegUnits.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/WasmEHFuncInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ISDOpcodes.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SchedulerRegistry.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineJumpTableInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/ScheduleDAG.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DemandedBits.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IVUsers.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/TargetLibraryInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CFLAndersAliasAnalysis.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MustExecute.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/CallGraphSCCPass.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyCallGraph.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyBlockFrequencyInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopInfo.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ObjCARCAnalysisUtils.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IntervalIterator.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LazyValueInfo.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AliasAnalysisEvaluator.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/SyntheticCountsUtils.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AssumptionCache.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/AliasAnalysis.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ValueLatticeUtils.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IndirectCallSiteVisitor.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeTypedef.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolFunc.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolPublicSymbol.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBDataStream.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/IPDBInjectedSource.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeDimension.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolFuncDebugEnd.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolExe.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBExtras.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumDebugStreams.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIATable.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSymbols.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSourceFiles.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumSectionContribs.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumLineNumbers.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASourceFile.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIARawSymbol.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIALineNumber.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIALineNumber.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAError.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIADataStream.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASectionContrib.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAInjectedSource.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIAEnumInjectedSources.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/DIA/DIASupport.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolTypeArray.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCompiland.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/PDBSymbolCompilandDetails.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFExpression.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDataExtractor.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugPubTable.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFUnit.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugRnglists.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAttribute.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugInfoEntry.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugAbbrev.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFGdbIndex.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDie.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFDebugArangeSet.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFVerifier.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFContext.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/DWARF/DWARFAddressRange.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MappedBlockStream.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFCommon.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFBuilder.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopSink.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopDataPrefetch.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopRotation.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/IndVarSimplify.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopUnrollAndJamPass.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/CorrelatedValuePropagation.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopDeletion.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/InstSimplifyPass.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/SimpleLoopUnswitch.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Coroutines.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/LoopVectorize.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/SLPVectorizer.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize/LoopVectorizationLegality.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/AggressiveInstCombine
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/AggressiveInstCombine/AggressiveInstCombine.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine/InstCombine.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/InstCombine/InstCombineWorklist.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/ObjCARC.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/FunctionAttrs.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/WholeProgramDevirt.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SCCP.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/DeadArgumentElimination.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SyntheticCountsPropagation.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SampleProfile.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/AlwaysInliner.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ForceFunctionAttrs.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/Inliner.h
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/UnifyFunctionExitNodes.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/AddDiscriminators.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterImpl.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/VNCoercion.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SSAUpdaterBulk.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BuildLibCalls.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/SimplifyLibCalls.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/BreakCriticalEdges.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Utils/LowerMemIntrinsics.h
---
[00:56:37] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm
[00:56:37] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/VCSRevision.h
[00:56:37] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicImpl.inc
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.inc
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicEnums.inc
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/abi-breaking.h
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/AsmParsers.def
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/Targets.def
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Config/AsmPrinters.def
---
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitReader.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMBitWriter.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMTransformUtils.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInstrumentation.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMAggressiveInstCombine.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMScalarOpts.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMipo.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMVectorize.a
[00:56:37] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/LLVMHello.so
---
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-diff
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dis
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dwarfdump
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-dwp
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-exegesis
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-link
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-lto2
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mc
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mca
---
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMConfig.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVMConfigVersion.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/LLVM-Config.cmake
[00:56:38] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/.
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindLibpfm.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./HandleLLVMOptions.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./TableGen.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./GetSVN.cmake
[00:56:38] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./GenerateVersionFromCVS.cmake
---
[01:03:32] warning: 3 warnings generated.
[01:03:47] error: failed to run custom build command for `rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)`
[01:03:47] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_lsan-e76b3d36992e85e6/build-script-build` (exit code: 101)
[01:03:47] --- stdout
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface_internal.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_always_instrument.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_never_instrument.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_function_call_trie.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_AArch64.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_segmented_array.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/weak_symbols.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_linux.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_malloc_mac.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/.clang-format
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_flags.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_mac.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_preinit.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/CMakeLists.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_interceptors.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_linux.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_mac.cc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/weak_symbols.txt
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negvdi2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_thread_fence.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/subdf3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfti.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divdi3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixtfdi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qadd.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/restFP.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qsub.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatditf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/saveFP.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixunstfdi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/divtc3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatunditf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qmul.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/DD.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qdiv.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/multc3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negdf2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/extenddftf2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulosi4.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ashldi3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/absvsi2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatsidf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdidf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fmin_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_abi2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/sfdiv_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath_dlib_asm.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfaddsub.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivmodsi4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/memcpy_forward_vp4cp4n2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fabs_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivmoddi4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_legacy.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivdi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_abi1.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/memcpy_likely_aligned.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/divsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/umoddi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfsqrt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath2_ldlib_asm.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/sfsqrt_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/moddi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/modsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath2_dlib_asm.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfmul.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfdiv.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fmax_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fma_opt.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/divdi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfminmax.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/umodsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dffma.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/adddf3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfti.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvti3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/extendhfsf2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/moddi3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divmoddi4.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdisf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negvti2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/muldi3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_lib.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfsi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/comparesf2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negdi2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdixf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunstfsi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfdi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ctzti2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/muldf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/extendsfdf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uidivmod.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssidfvfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negdf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunssfsivfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcpy.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divdf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapdi2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_fcmp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/truncdfsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtdf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_ldivmod.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uldivmod.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nesf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_dcmp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memset.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ledf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapsi2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gesf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/lesf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nedf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unordsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsisfvfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memmove.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/comparesf2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subsf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/softfloat-alias.list
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divmodsi4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcmp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negsf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/modsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/mulsf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzsi2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subdf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_idivmod.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzdi2.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch16.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gedf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/adddf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixsfsivfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltdf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqdf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsidfvfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch32.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switchu8.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_4.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync-ops.h
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixdfsivfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/umodsi3.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unorddf2vfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssisfvfp.S
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatundidf.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/powixf2.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvdi3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divtf3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfsi.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/multf3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_flag_clear_explicit.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_trunc_impl.inc
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/lshrti3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/addtf3.c
[01:03:47] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ashlti3.c
---
[01:03:49] -- Looking for fopen in c
[01:03:49] -- Looking for fopen in c - found
[01:03:49] -- Looking for __gcc_personality_v0 in gcc_s
[01:03:49] -- Looking for __gcc_personality_v0 in gcc_s - found
[01:03:49] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[01:03:49] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_G_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[01:03:49] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[01:03:49] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[01:03:49] -- Looking for __func__
[01:03:49] -- Looking for __func__ - found
[01:03:49] -- Looking for dlopen in dl - found
[01:03:49] -- Looking for shm_open in rt
[01:03:49] -- Looking for shm_open in rt - found
[01:03:49] -- Looking for pow in m
[01:03:49] -- Looking for pow in m
[01:03:49] -- Looking for pow in m - found
[01:03:49] -- Looking for pthread_create in pthread - found
[01:03:49] -- Looking for pthread_create in pthread - found
[01:03:49] -- Looking for __cxa_throw in c++
[01:03:49] -- Looking for __cxa_throw in c++ - not found
[01:03:49] -- Looking for __cxa_throw in stdc++
[01:03:49] -- Looking for __cxa_throw in stdc++ - found
[01:03:49] -- Looking for __i386__
[01:03:49] -- Looking for __i386__ - found
[01:03:49] -- Compiler-RT supported architectures: x86_64;i386
[01:03:49] -- Looking for rpc/xdr.h
[01:03:49] -- Looking for rpc/xdr.h - not found
[01:03:49] -- Looking for tirpc/rpc/xdr.h
[01:03:49] -- Looking for tirpc/rpc/xdr.h - not found
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[01:03:49] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[01:03:49] -- Performing Test HAS_THREAD_LOCAL - Success
[01:03:49] -- Configuring done
[01:03:49] -- Generating done
[01:03:49] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[01:03:49] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[01:03:49] running: "cmake" "--build" "." "--target" "lsan" "--config" "Release" "--"
[01:03:49] Scanning dependencies of target RTLSanCommon.x86_64
[01:03:49] [  5%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common.cc.o
[01:03:49] [  5%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_linux.cc.o
[01:03:49] [  5%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_mac.cc.o
[01:03:49] [  5%] Built target RTLSanCommon.x86_64
[01:03:49] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:49] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:49] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:49] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:49] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:49] [ 10%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:49] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:49] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:49] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:49] [ 25%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:49] [ 25%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:49] [ 25%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:49] --- stderr
[01:03:49] --- stderr
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1042:17: error: use of undeclared identifier 'mmsghdr'
[01:03:49] CHECK_TYPE_SIZE(mmsghdr);
[01:03:49]                 ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: use of undeclared identifier 'mmsghdr'
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:49]                       ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:1: error: expected expression
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:49] ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:49]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:49]                                  ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: unknown type name 'mmsghdr'
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:49]                       ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: use of undeclared identifier 'mmsghdr'
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:49]                       ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:1: error: expected expression
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:49] ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:49]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:49]                                  ^
[01:03:49] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: unknown type name 'mmsghdr'
[01:03:49] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:49] 7 errors generated.
[01:03:49] 7 errors generated.
[01:03:49] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o] Error 1
[01:03:49] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:49] gmake[1]: *** [lib/lsan/CMakeFiles/lsan.dir/rule] Error 2
[01:03:49] gmake: *** [lsan] Error 2
[01:03:49] command did not execute successfully, got: exit code: 2
[01:03:49] 
[01:03:49] 
[01:03:49] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[01:03:49] 
[01:03:49] warning: build failed, waiting for other jobs to finish...
[01:03:50] error: failed to run custom build command for `rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)`
[01:03:50] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_tsan-680d16a9f08c7428/build-script-build` (exit code: 101)
[01:03:50] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_tsan-680d16a9f08c7428/build-script-build` (exit code: 101)
[01:03:50] --- stdout
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_always_instrument.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_never_instrument.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.cc
---
[01:03:52] -- Looking for fopen in c
[01:03:52] -- Looking for fopen in c - found
[01:03:52] -- Looking for __gcc_personality_v0 in gcc_s
[01:03:52] -- Looking for __gcc_personality_v0 in gcc_s - found
[01:03:52] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[01:03:52] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_G_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[01:03:52] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[01:03:52] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[01:03:52] -- Looking for __func__
[01:03:52] -- Looking for __func__ - found
[01:03:52] -- Looking for dlopen in dl - found
[01:03:52] -- Looking for shm_open in rt
[01:03:52] -- Looking for shm_open in rt - found
[01:03:52] -- Looking for pow in m
[01:03:52] -- Looking for pow in m
[01:03:52] -- Looking for pow in m - found
[01:03:52] -- Looking for pthread_create in pthread - found
[01:03:52] -- Looking for pthread_create in pthread - found
[01:03:52] -- Looking for __cxa_throw in c++
[01:03:52] -- Looking for __cxa_throw in c++ - not found
[01:03:52] -- Looking for __cxa_throw in stdc++
[01:03:52] -- Looking for __cxa_throw in stdc++ - found
[01:03:52] -- Looking for __i386__
[01:03:52] -- Looking for __i386__ - found
[01:03:52] -- Compiler-RT supported architectures: x86_64;i386
[01:03:52] -- Looking for rpc/xdr.h
[01:03:52] -- Looking for rpc/xdr.h - not found
[01:03:52] -- Looking for tirpc/rpc/xdr.h
[01:03:52] -- Looking for tirpc/rpc/xdr.h - not found
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[01:03:52] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[01:03:52] -- Performing Test HAS_THREAD_LOCAL - Success
[01:03:52] -- Configuring done
[01:03:52] -- Generating done
[01:03:52] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/tsan/build
[01:03:52] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/tsan/build
[01:03:52] running: "cmake" "--build" "." "--target" "tsan" "--config" "Release" "--"
[01:03:52] Scanning dependencies of target RTUbsan.x86_64
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_monitor.cc.o
[01:03:52] [  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[01:03:52] [  0%] Built target RTUbsan.x86_64
[01:03:52] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:52] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:52] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:52] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:52] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:52] [  7%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:52] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:52] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:52] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:52] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:52] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_mac_libcdep.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_posix_libcdep.cc.o
[01:03:52] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stoptheworld_linux_libcdep.cc.o
[01:03:52] [ 30%] Built target RTSanitizerCommonLibc.x86_64
[01:03:52] --- stderr
[01:03:52] --- stderr
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1042:17: error: use of undeclared identifier 'mmsghdr'
[01:03:52] CHECK_TYPE_SIZE(mmsghdr);
[01:03:52]                 ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: use of undeclared identifier 'mmsghdr'
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:52]                       ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:1: error: expected expression
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:52] ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:52]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:52]                                  ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: unknown type name 'mmsghdr'
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:52]                       ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: use of undeclared identifier 'mmsghdr'
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:52]                       ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:1: error: expected expression
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:52] ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:52]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:52]                                  ^
[01:03:52] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: unknown type name 'mmsghdr'
[01:03:52] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:52] 7 errors generated.
[01:03:52] 7 errors generated.
[01:03:52] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o] Error 1
[01:03:52] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:52] gmake[2]: *** Waiting for unfinished jobs....
[01:03:52] gmake[1]: *** [lib/tsan/CMakeFiles/tsan.dir/rule] Error 2
[01:03:52] gmake: *** [tsan] Error 2
[01:03:52] command did not execute successfully, got: exit code: 2
[01:03:52] 
[01:03:52] 
[01:03:52] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[01:03:52] 
[01:03:52] warning: build failed, waiting for other jobs to finish...
[01:03:53] error: failed to run custom build command for `rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)`
[01:03:53] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-0016ee115cf0e56c/build-script-build` (exit code: 101)
[01:03:53] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-0016ee115cf0e56c/build-script-build` (exit code: 101)
[01:03:53] --- stdout
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:53] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
---
[01:03:56] -- Looking for fopen in c
[01:03:56] -- Looking for fopen in c - found
[01:03:56] -- Looking for __gcc_personality_v0 in gcc_s
[01:03:56] -- Looking for __gcc_personality_v0 in gcc_s - found
[01:03:56] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[01:03:56] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_G_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[01:03:56] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[01:03:56] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[01:03:56] -- Looking for __func__
[01:03:56] -- Looking for __func__ - found
[01:03:56] -- Looking for dlopen in dl - found
[01:03:56] -- Looking for shm_open in rt
[01:03:56] -- Looking for shm_open in rt - found
[01:03:56] -- Looking for pow in m
[01:03:56] -- Looking for pow in m
[01:03:56] -- Looking for pow in m - found
[01:03:56] -- Looking for pthread_create in pthread - found
[01:03:56] -- Looking for pthread_create in pthread - found
[01:03:56] -- Looking for __cxa_throw in c++
[01:03:56] -- Looking for __cxa_throw in c++ - not found
[01:03:56] -- Looking for __cxa_throw in stdc++
[01:03:56] -- Looking for __cxa_throw in stdc++ - found
[01:03:56] -- Looking for __i386__
[01:03:56] -- Looking for __i386__ - found
[01:03:56] -- Compiler-RT supported architectures: x86_64;i386
[01:03:56] -- Looking for rpc/xdr.h
[01:03:56] -- Looking for rpc/xdr.h - not found
[01:03:56] -- Looking for tirpc/rpc/xdr.h
[01:03:56] -- Looking for tirpc/rpc/xdr.h - not found
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[01:03:56] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[01:03:56] -- Performing Test HAS_THREAD_LOCAL - Success
[01:03:56] -- Configuring done
[01:03:56] -- Generating done
[01:03:56] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[01:03:56] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[01:03:56] running: "cmake" "--build" "." "--target" "msan" "--config" "Release" "--"
[01:03:56] Scanning dependencies of target msan_blacklist
[01:03:56] [ 10%] Copying msan_blacklist.txt...
[01:03:56] [ 10%] Built target msan_blacklist
[01:03:56] Scanning dependencies of target RTUbsan.x86_64
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_monitor.cc.o
[01:03:56] [ 10%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[01:03:56] [ 10%] Built target RTUbsan.x86_64
[01:03:56] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:56] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:56] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:56] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:56] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:56] [ 20%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:56] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:56] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:56] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:56] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:56] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[01:03:56] Scanning dependencies of target RTSanitizerCommonSymbolizer.x86_64
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_allocator_report.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stackdepot.cc.o
[01:03:56] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_libcdep.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_mac_libcdep.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_printer.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_sparc.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_posix_libcdep.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libbacktrace.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libcdep.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stoptheworld_linux_libcdep.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_mac.cc.o
[01:03:56] [ 60%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_markup.cc.o
[01:03:56] [ 70%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_posix_libcdep.cc.o
[01:03:56] [ 70%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_report.cc.o
[01:03:56] [ 70%] Built target RTSanitizerCommonLibc.x86_64
[01:03:56] [ 70%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_win.cc.o
[01:03:56] [ 70%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_linux_libcdep.cc.o
[01:03:56] [ 70%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_win.cc.o
[01:03:56] [ 70%] Built target RTSanitizerCommonSymbolizer.x86_64
[01:03:56] --- stderr
[01:03:56] --- stderr
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1042:17: error: use of undeclared identifier 'mmsghdr'
[01:03:56] CHECK_TYPE_SIZE(mmsghdr);
[01:03:56]                 ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: use of undeclared identifier 'mmsghdr'
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:56]                       ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:1: error: expected expression
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:56] ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:56]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:56]                                  ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: unknown type name 'mmsghdr'
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:56]                       ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: use of undeclared identifier 'mmsghdr'
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:56]                       ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:1: error: expected expression
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:56] ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:56]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:56]                                  ^
[01:03:56] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: unknown type name 'mmsghdr'
[01:03:56] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:56] 7 errors generated.
[01:03:56] 7 errors generated.
[01:03:56] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o] Error 1
[01:03:56] gmake[3]: *** Waiting for unfinished jobs....
[01:03:56] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:56] gmake[2]: *** Waiting for unfinished jobs....
[01:03:56] gmake[1]: *** [lib/msan/CMakeFiles/msan.dir/rule] Error 2
[01:03:56] gmake: *** [msan] Error 2
[01:03:56] command did not execute successfully, got: exit code: 2
[01:03:56] 
[01:03:56] 
[01:03:56] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[01:03:56] 
[01:03:56] warning: build failed, waiting for other jobs to finish...
[01:04:24] error: build failed
[01:04:24] error: build failed
[01:04:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:04:24] expected success, got: exit code: 101
[01:04:24] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[01:04:24] travis_fold:end:stage1-std

[01:04:24] travis_time:end:stage1-std:start=1530574501043058016,finish=1530574566951103066,duration=65908045050

---
travis_time:end:1b6d52d0:start=1530574567664861042,finish=1530574567683458386,duration=18597344
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05bd330a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00639f3c
$ dmesg | grep -i kill
