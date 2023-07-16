plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0cb78dc4
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
    100% |████████████████████████████████| 552kB 2.0MB/s 
Collecting botocore==1.12.42 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/6f/ad/706263fda4a8c673fd58c1cf03160dfdcf093d6614130193d3ce12a81fad/botocore-1.12.42-py2.py3-none-any.whl (4.8MB)
    0% |                                | 10kB 46.1MB/s eta 0:00:01
    0% |▏                               | 20kB 44.2MB/s eta 0:00:01
    0% |▏                               | 30kB 48.7MB/s eta 0:00:01
    0% |▎                               | 40kB 27.1MB/s eta 0:00:01
---
[00:24:49] [ 10%] Linking CXX static library ../../libLLVMMCParser.a
[00:24:49] [ 10%] Built target LLVMMCParser
[00:24:49] Scanning dependencies of target LLVMOption
[00:24:49] [ 10%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/Arg.cpp.o
[00:24:49] Scanning dependencies of target LLVMOptRemarks
[00:24:49] [ 11%] Building CXX object lib/OptRemarks/CMakeFiles/LLVMOptRemarks.dir/OptRemarksParser.cpp.o
[00:24:51] [ 11%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/ArgList.cpp.o
[00:24:51] [ 11%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/ArgList.cpp.o
[00:24:53] [ 11%] Linking CXX static library ../libLLVMOptRemarks.a
[00:24:53] [ 11%] Built target LLVMOptRemarks
[00:24:53] [ 11%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAbbreviationDeclaration.cpp.o
[00:24:53] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Errno.cpp.o
[00:24:54] [ 11%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/Option.cpp.o
[00:24:55] [ 11%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Host.cpp.o
---
[00:27:06] [ 17%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeTypeTypedef.cpp.o
[00:27:08] [ 17%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeTypeUDT.cpp.o
[00:27:09] Scanning dependencies of target llvm-PerfectShuffle
[00:27:09] [ 17%] Building CXX object utils/PerfectShuffle/CMakeFiles/llvm-PerfectShuffle.dir/PerfectShuffle.cpp.o
[00:27:09] [ 17%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeTypeVTShape.cpp.o
[00:27:09] [ 17%] Linking CXX executable ../../bin/llvm-PerfectShuffle
[00:27:10] [ 17%] Built target llvm-PerfectShuffle
[00:27:10] Scanning dependencies of target count
[00:27:10] [ 17%] Building C object utils/count/CMakeFiles/count.dir/count.c.o
---
[00:27:20] Scanning dependencies of target LLVMMCA
[00:27:20] [ 18%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/Context.cpp.o
[00:27:21] [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/SymbolCache.cpp.o
[00:27:24] [ 18%] Built target yaml-bench
[00:27:24] Scanning dependencies of target OptRemarks_exports
[00:27:24] [ 18%] Creating export file for OptRemarks
[00:27:24] [ 18%] Built target OptRemarks_exports
[00:27:24] [ 18%] Linking CXX executable ../../bin/llvm-tblgen
[00:27:24] [ 18%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HWEventListener.cpp.o
[00:27:25] [ 18%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/HardwareUnit.cpp.o
[00:27:25] [ 18%] Building CXX object tools/llvm-mca/lib/CMakeFiles/LLVMMCA.dir/HardwareUnits/LSUnit.cpp.o
---
[00:38:16] [ 35%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopVersioning.cpp.o
[00:38:17] [ 35%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LowerInvoke.cpp.o
[00:38:18] [ 35%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/CGProfile.cpp.o
[00:38:21] [ 35%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LowerMemIntrinsics.cpp.o
[00:38:23] [ 35%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/ControlHeightReduction.cpp.o
[00:38:25] [ 36%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/DataFlowSanitizer.cpp.o
[00:38:26] [ 36%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Mem2Reg.cpp.o
[00:38:30] [ 36%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/MetaRenamer.cpp.o
[00:38:31] [ 36%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ModuleUtils.cpp.o
---
[00:41:12] [ 38%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopInstSimplify.cpp.o
[00:41:19] [ 38%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopInterchange.cpp.o
[00:41:19] [ 38%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopLoadElimination.cpp.o
[00:41:19] [ 38%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/GlobalSplit.cpp.o
[00:41:20] [ 38%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/HotColdSplitting.cpp.o
[00:41:27] [ 38%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/IPO.cpp.o
[00:41:28] [ 38%] Building CXX object lib/Transforms/IPO/CMakeFiles/LLVMipo.dir/InferFunctionAttrs.cpp.o
[00:41:28] [ 38%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopPassManager.cpp.o
[00:41:29] [ 38%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopPredication.cpp.o
---
[00:48:08] [ 52%] Building AArch64GenRegisterInfo.inc...
[00:48:09] [ 52%] Building AArch64GenSubtargetInfo.inc...
[00:48:09] [ 52%] Building X86GenInstrInfo.inc...
[00:48:10] [ 52%] Building AArch64GenSystemOperands.inc...
[00:48:11] [ 52%] Building AArch64GenExegesis.inc...
[00:48:11] [ 52%] Updating AArch64GenAsmWriter.inc...
[00:48:11] [ 52%] Updating AArch64GenAsmWriter1.inc...
[00:48:11] [ 52%] Updating AArch64GenCallingConv.inc...
[00:48:11] [ 52%] Updating AArch64GenDAGISel.inc...
---
[00:48:11] [ 52%] Updating AArch64GenMCPseudoLowering.inc...
[00:48:11] [ 52%] Updating AArch64GenRegisterBank.inc...
[00:48:11] [ 52%] Updating AArch64GenRegisterInfo.inc...
[00:48:11] [ 52%] Updating AArch64GenSystemOperands.inc...
[00:48:11] [ 52%] Updating AArch64GenExegesis.inc...
[00:48:11] [ 52%] Building MipsGenAsmMatcher.inc...
[00:48:12] [ 52%] Building X86GenRegisterBank.inc...
[00:48:12] [ 53%] Building MipsGenAsmWriter.inc...
[00:48:13] [ 53%] Building MipsGenCallingConv.inc...
---
[00:48:18] [ 53%] Building MipsGenMCPseudoLowering.inc...
[00:48:18] [ 54%] Building PPCGenRegisterInfo.inc...
[00:48:19] [ 54%] Building PPCGenSubtargetInfo.inc...
[00:48:19] [ 54%] Building MipsGenRegisterBank.inc...
[00:48:19] [ 54%] Building PPCGenExegesis.inc...
[00:48:19] [ 54%] Updating PPCGenAsmMatcher.inc...
[00:48:20] [ 54%] Updating PPCGenAsmWriter.inc...
[00:48:20] [ 54%] Updating PPCGenCallingConv.inc...
[00:48:20] [ 54%] Updating PPCGenDAGISel.inc...
[00:48:20] [ 54%] Updating PPCGenDAGISel.inc...
[00:48:20] [ 54%] Updating PPCGenDisassemblerTables.inc...
[00:48:20] [ 54%] Updating PPCGenFastISel.inc...
[00:48:20] [ 54%] Updating PPCGenInstrInfo.inc...
[00:48:20] [ 54%] Updating PPCGenMCCodeEmitter.inc...
[00:48:20] [ 54%] Building X86GenExegesis.inc...
[00:48:20] [ 54%] Updating PPCGenRegisterInfo.inc...
[00:48:20] [ 54%] Updating PPCGenSubtargetInfo.inc...
[00:48:20] [ 54%] Updating PPCGenExegesis.inc...
[00:48:20] Scanning dependencies of target SystemZCommonTableGen
[00:48:20] [ 54%] Building SystemZGenAsmMatcher.inc...
[00:48:20] [ 54%] Building MipsGenSubtargetInfo.inc...
[00:48:20] [ 54%] Building SystemZGenAsmWriter.inc...
---
[00:48:43] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/BlockPrinter.cpp.o
[00:48:45] [ 59%] Linking CXX static library ../../libLLVMCoverage.a
[00:48:45] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/BlockVerifier.cpp.o
[00:48:45] [ 59%] Built target LLVMCoverage
[00:48:45] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecordProducer.cpp.o
[00:48:47] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.o
[00:48:47] [ 59%] Built target LLVMAsmParser
[00:48:47] Scanning dependencies of target LLVMCFIVerify
[00:48:47] [ 59%] Building CXX object tools/llvm-cfi-verify/lib/CMakeFiles/LLVMCFIVerify.dir/FileAnalysis.cpp.o
[00:48:47] [ 59%] Building CXX object tools/llvm-cfi-verify/lib/CMakeFiles/LLVMCFIVerify.dir/FileAnalysis.cpp.o
[00:48:48] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.o
[00:48:49] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.o
[00:48:51] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.o
[00:48:54] [ 59%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.o
[00:48:55] [ 59%] Linking CXX static library ../../../lib/libLLVMCFIVerify.a
[00:48:55] [ 59%] Built target LLVMCFIVerify
---
[00:49:19] [ 61%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/PerfHelper.cpp.o
[00:49:20] [ 61%] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/ExecutionUtils.cpp.o
[00:49:20] [ 61%] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IndirectionUtils.cpp.o
[00:49:21] [ 61%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterAliasing.cpp.o
[00:49:23] [ 61%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/SnippetGenerator.cpp.o
[00:49:27] [ 61%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Target.cpp.o
[00:49:31] [ 61%] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IRCompileLayer.cpp.o
[00:49:31] [ 61%] Building CXX object lib/ExecutionEngine/Orc/CMakeFiles/LLVMOrcJIT.dir/IRTransformLayer.cpp.o
[00:49:31] [ 61%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Uops.cpp.o
---
[00:55:08] [ 70%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsDelaySlotFiller.cpp.o
[00:55:11] [ 70%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsExpandPseudo.cpp.o
[00:55:14] [ 70%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64FrameLowering.cpp.o
[00:55:17] [ 70%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFastISel.cpp.o
[00:55:18] [ 70%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64CompressJumpTables.cpp.o
[00:55:23] [ 70%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ConditionOptimizer.cpp.o
[00:55:26] [ 70%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64RedundantCopyElimination.cpp.o
[00:55:26] [ 71%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsInstructionSelector.cpp.o
[00:55:28] [ 71%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
---
[00:56:03] [ 72%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64MacroFusion.cpp.o
[00:56:07] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsOs16.cpp.o
[00:56:10] [ 72%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64MCInstLower.cpp.o
[00:56:10] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsRegisterBankInfo.cpp.o
[00:56:11] [ 72%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64PreLegalizerCombiner.cpp.o
[00:56:16] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsSEFrameLowering.cpp.o
[00:56:16] [ 72%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64PromoteConstant.cpp.o
[00:56:18] [ 72%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64PBQPRegAlloc.cpp.o
[00:56:18] [ 72%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsSEInstrInfo.cpp.o
---
[01:04:58] [ 87%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyTargetTransformInfo.cpp.o
[01:04:59] [ 87%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyUtilities.cpp.o
[01:05:00] [ 87%] Linking CXX static library ../../libLLVMHexagonCodeGen.a
[01:05:00] [ 87%] Built target LLVMHexagonCodeGen
[01:05:00] Scanning dependencies of target LLVMRISCVUtils
[01:05:00] [ 88%] Building CXX object lib/Target/RISCV/Utils/CMakeFiles/LLVMRISCVUtils.dir/RISCVBaseInfo.cpp.o
[01:05:00] [ 88%] Built target LLVMRISCVInfo
[01:05:00] Scanning dependencies of target lli-child-target
[01:05:00] [ 88%] Building CXX object tools/lli/ChildTarget/CMakeFiles/lli-child-target.dir/ChildTarget.cpp.o
[01:05:02] [ 88%] Linking CXX static library ../../../libLLVMRISCVUtils.a
---
[01:18:48] [ 92%] Building CXX object tools/llvm-cxxdump/CMakeFiles/llvm-cxxdump.dir/Error.cpp.o
[01:18:49] [ 92%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageExporterJson.cpp.o
[01:18:50] [ 92%] Linking CXX executable ../../bin/llvm-cxxfilt
[01:18:51] [ 92%] Built target llvm-cxxfilt
[01:18:51] Scanning dependencies of target llvm-cxxmap
[01:18:51] [ 92%] Building CXX object tools/llvm-cxxmap/CMakeFiles/llvm-cxxmap.dir/llvm-cxxmap.cpp.o
[01:18:52] [ 92%] Linking CXX executable ../../bin/llvm-cxxdump
[01:18:53] [ 92%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageExporterLcov.cpp.o
[01:18:54] [ 92%] Linking CXX executable ../../bin/llvm-cxxmap
[01:18:54] Scanning dependencies of target llvm-demangle-fuzzer
[01:18:54] [ 92%] Building CXX object tools/llvm-demangle-fuzzer/CMakeFiles/llvm-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[01:18:55] [ 92%] Built target llvm-cxxmap
[01:18:55] Scanning dependencies of target llvm-diff
---
[01:21:54] [ 98%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-extract.cpp.o
[01:21:54] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/Error.cpp.o
[01:21:56] [ 98%] Building CXX object tools/opt/CMakeFiles/opt.dir/NewPMDriver.cpp.o
[01:21:56] [ 98%] Building CXX object tools/opt/CMakeFiles/opt.dir/PassPrinters.cpp.o
[01:21:58] [ 98%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-fdr-dump.cpp.o
[01:22:01] [ 99%] Building CXX object tools/opt/CMakeFiles/opt.dir/PrintSCC.cpp.o
[01:22:03] [ 99%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-graph-diff.cpp.o
[01:22:06] [ 99%] Built target obj2yaml
[01:22:06] Scanning dependencies of target OptRemarks
[01:22:06] Scanning dependencies of target OptRemarks
[01:22:06] [ 99%] Building CXX object tools/opt-remarks/CMakeFiles/OptRemarks.dir/liboptremarks.cpp.o
[01:22:06] [ 99%] Linking CXX shared library ../../lib/libOptRemarks.so
[01:22:06] [ 99%] Built target OptRemarks
[01:22:06] [ 99%] Building CXX object tools/sancov/CMakeFiles/sancov.dir/sancov.cpp.o
[01:22:06] [ 99%] Building CXX object tools/opt/CMakeFiles/opt.dir/opt.cpp.o
[01:22:07] [ 99%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-graph.cpp.o
[01:22:09] [ 99%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-registry.cpp.o
---
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/BinaryItemStream.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/LICENSE.TXT
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FormatCommon.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/Watchdog.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/MSVCErrorWorkarounds.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/xxhash.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/RecyclingAllocator.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/DataExtractor.h
[01:22:34] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Support/FileOutputBuffer.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/SafepointIRVerifier.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/SymbolTableListTraits.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/DiagnosticInfo.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsBPF.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicsRISCV.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/Attributes.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/GVMaterializer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/IntrinsicInst.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IR/TypeBuilder.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopInfo.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/Lint.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DomPrinter.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/PHITransAddr.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LegacyDivergenceAnalysis.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/InlineCost.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ValueTracking.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/SyncDependenceAnalysis.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/IndirectCallPromotionAnalysis.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeSymbolEnumerator.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/RawTypes.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeCompilandSymbol.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/DbiModuleDescriptor.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeTypeFunctionSig.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeTypeUDT.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeTypeVTShape.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeEnumModules.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/GlobalsStream.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/TpiStreamBuilder.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/PDB/Native/NativeTypeTypedef.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/IRReader/IRReader.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassSupport.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRTraceExpander.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/BlockIndexer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRTraceWriter.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRRecordProducer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/YAMLXRayRecord.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRLogBuilder.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/BlockPrinter.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Trace.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Profile.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Graph.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/Graph.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRRecordConsumer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/XRayRecord.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/BlockVerifier.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/InstrumentationMap.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/RecordPrinter.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/XRay/FDRRecords.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest/WindowsManifestMerger.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/IRMutator.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/FuzzMutate/OpDescriptor.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/NullResolver.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LazyReexports.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/ObjectTransformLayer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/GlobalMappingLayer.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/JITTargetMachineBuilder.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcError.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcABISupport.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LLJIT.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetServer.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel/RegisterBank.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/GlobalISel/SelectionDAGCompat.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetSelectionDAG.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetSchedule.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetPfmCounters.td
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/TargetMachine.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/CodeGenCWrappers.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Vectorize.h
---
[01:22:35] Creating libLLVM.so
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/DeadArgumentElimination.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/SyntheticCountsPropagation.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/PartialInlining.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/HotColdSplitting.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/AlwaysInliner.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/ForceFunctionAttrs.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/Inliner.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/IPO/GlobalOpt.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/PGOInstrumentation.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/InstrProfiling.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/GCOVProfiler.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/CGProfile.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Instrumentation/ControlHeightReduction.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceProcessor.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceScriptToken.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsResource/ResourceScriptTokenList.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ToolDrivers
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Comdat.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Target.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Object.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Linker.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/OptRemarks.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Vectorize.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/InstCombine.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Coroutines.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/Transforms/Scalar.h
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMMCDisassembler.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObject.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObjectYAML.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOption.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOptRemarks.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoMSF.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoCodeView.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoPDB.a
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSymbolize.a
---
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-undname
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-xray
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/obj2yaml
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/opt
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libOptRemarks.so.8svn
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libOptRemarks.so
[01:22:35] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm-c/OptRemarks.h
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/opt-stats.py
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/opt-viewer.py
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/optpmap.py
[01:22:35] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/share/opt-viewer/optrecord.py
---
[02:55:32]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[02:55:32]    Compiling cargo_metadata v0.6.0
[02:56:30]    Compiling url v1.7.1

Broadcast message from root@travis-job-ae3ff621-4476-46fa-bda8-e5cc5dd405bf
 (unknown) at 6:04 ...
The system is going down for power off NOW!
