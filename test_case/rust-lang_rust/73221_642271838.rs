plain
##[section]Starting: Windows x86_64-msvc-2
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az433'
Current agent version: '2.170.1'
##[group]Operating System
10.0.14393
Datacenter
Datacenter
##[endgroup]
##[group]Virtual Environment
Environment: windows-2016
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/win16/20200604.1/images/win/Windows2016-Readme.md
##[endgroup]
Agent running as: 'VssAdministrator'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = C:/hostedtoolcache/windows Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = c:\vsts\perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/b489cb94-b881-4102-8bb4-15dff9c9e742.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73221/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73221/merge:refs/remotes/pull/73221/merge
---
warning: C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\include\vcruntime.h(245,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
warning: #define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
warning:                                               ^
warning: ../llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(398,21): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
warning:   int initialized = getenv(LPROF_INIT_ONCE_ENV) != NULL;
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\ucrt\stdlib.h(1190,20): note: 'getenv' has been explicitly marked deprecated here
warning:     _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
warning:                    ^
warning: C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\include\vcruntime.h(255,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
---
[147/2550] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj
[148/2550] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj
[149/2550] Building RC object utils\TableGen\CMakeFiles\llvm-tblgen.dir\__\__\resources\windows_version_resource.rc.res
[150/2550] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\CTagsEmitter.cpp.obj
[151/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagEdge.cpp.obj
[152/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagPredicate.cpp.obj
[153/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagOperands.cpp.obj
[154/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchTree.cpp.obj
[155/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDag.cpp.obj
[156/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagPredicateDependencyEdge.cpp.obj
[157/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\CodeExpander.cpp.obj
[158/2550] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagInstr.cpp.obj
[160/2550] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\SpecialCaseList.cpp.obj
[161/2550] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\SymbolRemappingReader.cpp.obj
[162/2550] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Timer.cpp.obj
[163/2550] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ToolOutputFile.cpp.obj
---
[421/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\RegAllocBase.cpp.obj
[422/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\RenameIndependentSubregs.cpp.obj
[423/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\MIRNamerPass.cpp.obj
[424/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\RegisterScavenging.cpp.obj
[425/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\MIRVRegNamerUtils.cpp.obj
[427/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\ScoreboardHazardRecognizer.cpp.obj
[428/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\ResetMachineFunctionPass.cpp.obj
[429/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\RegisterUsageInfo.cpp.obj
[430/2550] Building CXX object lib\CodeGen\CMakeFiles\LLVMCodeGen.dir\SafeStack.cpp.obj
---
[523/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\InlineAsm.cpp.obj
[524/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\AsmWriter.cpp.obj
[525/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\DiagnosticPrinter.cpp.obj
[526/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\DebugInfoMetadata.cpp.obj
[527/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\FPEnv.cpp.obj
[529/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Function.cpp.obj
[530/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\DataLayout.cpp.obj
[531/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\Dominators.cpp.obj
[532/2550] Building CXX object lib\IR\CMakeFiles\LLVMCore.dir\DiagnosticInfo.cpp.obj
---
[594/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\LibCallsShrinkWrap.cpp.obj
[595/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\InstructionNamer.cpp.obj
[596/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\GuardUtils.cpp.obj
[597/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\Local.cpp.obj
[598/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\InjectTLIMappings.cpp.obj
[600/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\LCSSA.cpp.obj
[601/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\IntegerDivision.cpp.obj
[602/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\LoopRotationUtils.cpp.obj
[603/2550] Building CXX object lib\Transforms\Utils\CMakeFiles\LLVMTransformUtils.dir\LoopSimplify.cpp.obj
---
[740/2550] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\WarnMissedTransforms.cpp.obj
[741/2550] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\SimplifyCFGPass.cpp.obj
[742/2550] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\Sink.cpp.obj
[743/2550] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\StraightLineStrengthReduce.cpp.obj
[744/2550] Building CXX object lib\DWARFLinker\CMakeFiles\LLVMDWARFLinker.dir\DWARFLinker.cpp.obj
[745/2550] Building CXX object lib\DWARFLinker\CMakeFiles\LLVMDWARFLinker.dir\DWARFLinkerCompileUnit.cpp.obj
[746/2550] Building CXX object lib\DWARFLinker\CMakeFiles\LLVMDWARFLinker.dir\DWARFLinkerDeclContext.cpp.obj
[747/2550] Building CXX object lib\Frontend\OpenMP\CMakeFiles\LLVMFrontendOpenMP.dir\OMPConstants.cpp.obj
[748/2550] Building CXX object lib\Frontend\OpenMP\CMakeFiles\LLVMFrontendOpenMP.dir\OMPIRBuilder.cpp.obj
[750/2550] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\ElimAvailExtern.cpp.obj
[751/2550] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\GlobalOpt.cpp.obj
[752/2550] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\IPO.cpp.obj
[753/2550] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\ExtractGV.cpp.obj
---
[803/2550] Building CXX object lib\Transforms\Coroutines\CMakeFiles\LLVMCoroutines.dir\CoroFrame.cpp.obj
[804/2550] Building CXX object lib\Transforms\Coroutines\CMakeFiles\LLVMCoroutines.dir\CoroSplit.cpp.obj
[805/2550] Building CXX object lib\Transforms\Coroutines\CMakeFiles\LLVMCoroutines.dir\CoroCleanup.cpp.obj
[806/2550] Building CXX object lib\Transforms\Coroutines\CMakeFiles\LLVMCoroutines.dir\CoroElide.cpp.obj
[807/2550] Building CXX object lib\Transforms\CFGuard\CMakeFiles\LLVMCFGuard.dir\CFGuard.cpp.obj
[809/2550] Building CXX object lib\Linker\CMakeFiles\LLVMLinker.dir\LinkModules.cpp.obj
[810/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\CFG.cpp.obj
[811/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\BasicAliasAnalysis.cpp.obj
[812/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\AliasAnalysisEvaluator.cpp.obj
---
[831/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\CodeMetrics.cpp.obj
[832/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\ConstantFolding.cpp.obj
[833/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\CallGraphSCCPass.cpp.obj
[834/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\CmpInstAnalysis.cpp.obj
[835/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\DDG.cpp.obj
[837/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\DomTreeUpdater.cpp.obj
[838/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\IVDescriptors.cpp.obj
[839/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\GlobalsModRef.cpp.obj
[840/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\IVUsers.cpp.obj
---
[904/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\TypeBasedAliasAnalysis.cpp.obj
[905/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\TypeMetadataUtils.cpp.obj
[906/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\ScopedNoAliasAA.cpp.obj
[907/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\ValueTracking.cpp.obj
[908/2550] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\VFABIDemangling.cpp.obj
[910/2550] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\LTOBackend.cpp.obj
[911/2550] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\LTOCodeGenerator.cpp.obj
[912/2550] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\LTOModule.cpp.obj
[913/2550] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\SummaryBasedOptimizations.cpp.obj
---
[977/2550] Building CXX object lib\DebugInfo\DWARF\CMakeFiles\LLVMDebugInfoDWARF.dir\DWARFUnit.cpp.obj
[978/2550] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\ArgumentPromotion.cpp.obj
[979/2550] Building CXX object lib\DebugInfo\DWARF\CMakeFiles\LLVMDebugInfoDWARF.dir\DWARFUnitIndex.cpp.obj
[980/2550] Building CXX object lib\DebugInfo\DWARF\CMakeFiles\LLVMDebugInfoDWARF.dir\DWARFVerifier.cpp.obj
[981/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\GsymReader.cpp.obj
[982/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\FileWriter.cpp.obj
[983/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\GsymCreator.cpp.obj
[984/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\LookupResult.cpp.obj
[986/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\InlineInfo.cpp.obj
[987/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\Header.cpp.obj
[988/2550] Building CXX object lib\DebugInfo\PDB\CMakeFiles\LLVMDebugInfoPDB.dir\Native\DbiModuleDescriptor.cpp.obj
[989/2550] Building CXX object lib\DebugInfo\GSYM\CMakeFiles\LLVMDebugInfoGSYM.dir\FunctionInfo.cpp.obj
---
[1031/2550] Building CXX object lib\DebugInfo\CodeView\CMakeFiles\LLVMDebugInfoCodeView.dir\TypeDumpVisitor.cpp.obj
[1032/2550] Building CXX object lib\DebugInfo\CodeView\CMakeFiles\LLVMDebugInfoCodeView.dir\TypeRecordMapping.cpp.obj
[1033/2550] Building CXX object lib\DebugInfo\CodeView\CMakeFiles\LLVMDebugInfoCodeView.dir\SymbolDumper.cpp.obj
[1034/2550] Building CXX object lib\DebugInfo\CodeView\CMakeFiles\LLVMDebugInfoCodeView.dir\TypeStreamMerger.cpp.obj
D:\a\1\s\src\llvm-project\llvm\lib\DebugInfo\CodeView\TypeStreamMerger.cpp(392,12): warning: unused variable 'AlignedSize' [-Wunused-variable]
  unsigned AlignedSize = alignTo(OriginalType.RecordData.size(), 4);
1 warning generated.
[1035/2550] Building CXX object lib\DebugInfo\CodeView\CMakeFiles\LLVMDebugInfoCodeView.dir\SymbolRecordMapping.cpp.obj
[1036/2550] Linking CXX static library lib\LLVMDebugInfoCodeView.lib
[1037/2550] Linking CXX static library lib\LLVMMC.lib
---
[1115/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\COFFObjectFile.cpp.obj
[1116/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\IRObjectFile.cpp.obj
[1117/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\Archive.cpp.obj
[1118/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\XCOFFObjectFile.cpp.obj
[1119/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\TapiFile.cpp.obj
[1121/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\WasmObjectFile.cpp.obj
[1122/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\WindowsResource.cpp.obj
[1123/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\SymbolSize.cpp.obj
[1124/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\ObjectFile.cpp.obj
[1124/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\ObjectFile.cpp.obj
[1125/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\ModuleSymbolTable.cpp.obj
[1126/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\MachOObjectFile.cpp.obj
[1127/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\MachOUniversal.cpp.obj
[1128/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\RelocationResolver.cpp.obj
[1129/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\TapiUniversal.cpp.obj
[1131/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\Object.cpp.obj
[1132/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\WindowsMachineFlag.cpp.obj
[1133/2550] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\RecordStreamer.cpp.obj
[1134/2550] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\CodeViewYAMLTypeHashing.cpp.obj
---
[1257/2550] Building CXX object lib\ExecutionEngine\JITLink\CMakeFiles\LLVMJITLink.dir\MachO.cpp.obj
[1258/2550] Building CXX object lib\ExecutionEngine\JITLink\CMakeFiles\LLVMJITLink.dir\MachO_x86_64.cpp.obj
[1259/2550] Building CXX object lib\ExecutionEngine\JITLink\CMakeFiles\LLVMJITLink.dir\MachO_arm64.cpp.obj
[1260/2550] Building CXX object lib\ExecutionEngine\MCJIT\CMakeFiles\LLVMMCJIT.dir\MCJIT.cpp.obj
[1261/2550] Building CXX object lib\ExecutionEngine\JITLink\CMakeFiles\LLVMJITLink.dir\MachOLinkGraphBuilder.cpp.obj
[1262/2550] Building CXX object lib\ExecutionEngine\OrcError\CMakeFiles\LLVMOrcError.dir\RPCError.cpp.obj
[1263/2550] Building CXX object lib\ExecutionEngine\OrcError\CMakeFiles\LLVMOrcError.dir\OrcError.cpp.obj
[1264/2550] Linking CXX static library lib\LLVMOrcError.lib
[1266/2550] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\DebugUtils.cpp.obj
[1267/2550] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\CompileOnDemandLayer.cpp.obj
[1268/2550] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\IndirectionUtils.cpp.obj
[1269/2550] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\IRCompileLayer.cpp.obj
---
[1304/2550] Building AArch64GenGlobalISel.inc...
[1305/2550] Building AArch64GenInstrInfo.inc...
[1306/2550] Building AArch64GenSystemOperands.inc...
[1307/2550] Building AArch64GenSubtargetInfo.inc...
[1308/2550] Building AArch64GenGICombiner.inc...
[1310/2550] Building AArch64GenDisassemblerTables.inc...
[1311/2550] Building AArch64GenMCPseudoLowering.inc...
[1312/2550] Building AArch64GenExegesis.inc...
[1313/2550] Building AArch64GenFastISel.inc...
---
[1462/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64PBQPRegAlloc.cpp.obj
[1463/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64MacroFusion.cpp.obj
[1464/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64StackTagging.cpp.obj
[1465/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64StorePairSuppress.cpp.obj
[1466/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64StackTaggingPreRA.cpp.obj
[1468/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64TargetMachine.cpp.obj
[1469/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64Subtarget.cpp.obj
[1470/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64TargetObjectFile.cpp.obj
[1471/2550] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64SIMDInstrOpt.cpp.obj
---
[1590/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMSubtarget.cpp.obj
[1591/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMRegisterInfo.cpp.obj
[1592/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetMachine.cpp.obj
[1593/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ThumbRegisterInfo.cpp.obj
[1594/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVETailPredication.cpp.obj
[1596/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb1FrameLowering.cpp.obj
[1597/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2ITBlockPass.cpp.obj
[1598/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2SizeReduction.cpp.obj
[1599/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetTransformInfo.cpp.obj
[1599/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetTransformInfo.cpp.obj
[1600/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\Thumb2InstrInfo.cpp.obj
[1601/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVEGatherScatterLowering.cpp.obj
[1602/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMTargetObjectFile.cpp.obj
[1603/2550] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\MVEVPTBlockPass.cpp.obj
[1605/2550] Building CXX object lib\Target\ARM\AsmParser\CMakeFiles\LLVMARMAsmParser.dir\ARMAsmParser.cpp.obj
[1606/2550] Building CXX object lib\Target\ARM\Disassembler\CMakeFiles\LLVMARMDisassembler.dir\ARMDisassembler.cpp.obj
[1607/2550] Building CXX object lib\Target\ARM\MCTargetDesc\CMakeFiles\LLVMARMDesc.dir\ARMELFObjectWriter.cpp.obj
[1608/2550] Building CXX object lib\Target\ARM\MCTargetDesc\CMakeFiles\LLVMARMDesc.dir\ARMELFStreamer.cpp.obj
---
[1624/2550] Linking CXX static library lib\LLVMARMAsmParser.lib
[1625/2550] Linking CXX static library lib\LLVMARMDisassembler.lib
[1626/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCReduceCRLogicals.cpp.obj
[1627/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCExpandISEL.cpp.obj
[1628/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCLowerMASSVEntries.cpp.obj
[1630/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCVSXFMAMutate.cpp.obj
[1631/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCVSXSwapRemoval.cpp.obj
[1632/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCVSXCopy.cpp.obj
[1633/2550] Building CXX object lib\Target\PowerPC\CMakeFiles\LLVMPowerPCCodeGen.dir\PPCPreEmitPeephole.cpp.obj
---
[1978/2550] Linking CXX static library lib\LLVMTransformUtils.lib
[1979/2550] Linking CXX static library lib\LLVMInstrumentation.lib
[1980/2550] Linking CXX static library lib\LLVMAggressiveInstCombine.lib
[1981/2550] Linking CXX static library lib\LLVMInstCombine.lib
[1982/2550] Linking CXX static library lib\LLVMFrontendOpenMP.lib
[1984/2550] Linking CXX static library lib\LLVMObjCARCOpts.lib
[1985/2550] Linking CXX static library lib\LLVMLinker.lib
[1986/2550] Linking CXX static library lib\LLVMScalarOpts.lib
[1987/2550] Linking CXX static library lib\LLVMipo.lib
---
[2092/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86ShuffleDecodeConstantPool.cpp.obj
[2093/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86SpeculativeLoadHardening.cpp.obj
[2094/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86LoadValueInjectionLoadHardening.cpp.obj
[2095/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86MacroFusion.cpp.obj
[2096/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86LoadValueInjectionRetHardening.cpp.obj
[2098/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86PadShortFunction.cpp.obj
[2099/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86Subtarget.cpp.obj
[2100/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86RegisterBankInfo.cpp.obj
[2101/2550] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86TargetObjectFile.cpp.obj
---
[2301/2550] Linking CXX static library lib\LLVMExegesisX86.lib
[2302/2550] Building CXX object tools\llvm-extract\CMakeFiles\llvm-extract.dir\llvm-extract.cpp.obj
[2303/2550] Linking CXX executable bin\llvm-extract.exe
[2304/2550] Linking CXX executable bin\llvm-exegesis.exe
[2305/2550] Building CXX object tools\llvm-exegesis\lib\Mips\CMakeFiles\LLVMExegesisMips.dir\Target.cpp.obj
[2306/2550] Linking CXX static library lib\LLVMExegesisMips.lib
[2307/2550] Building CXX object tools\llvm-ifs\CMakeFiles\llvm-ifs.dir\llvm-ifs.cpp.obj
[2309/2550] Building CXX object tools\llvm-isel-fuzzer\CMakeFiles\llvm-isel-fuzzer.dir\DummyISelFuzzer.cpp.obj
[2309/2550] Building CXX object tools\llvm-isel-fuzzer\CMakeFiles\llvm-isel-fuzzer.dir\DummyISelFuzzer.cpp.obj
[2310/2550] Building RC object tools\llvm-ifs\CMakeFiles\llvm-ifs.dir\__\__\resources\windows_version_resource.rc.res
[2312/2550] Linking CXX executable bin\llvm-ifs.exe
[2313/2550] Building CXX object tools\llvm-itanium-demangle-fuzzer\CMakeFiles\llvm-itanium-demangle-fuzzer.dir\llvm-itanium-demangle-fuzzer.cpp.obj
[2314/2550] Building RC object tools\llvm-itanium-demangle-fuzzer\CMakeFiles\llvm-itanium-demangle-fuzzer.dir\__\__\resources\windows_version_resource.rc.res
[2315/2550] Building CXX object tools\llvm-itanium-demangle-fuzzer\CMakeFiles\llvm-itanium-demangle-fuzzer.dir\DummyDemanglerFuzzer.cpp.obj
---
[2327/2550] Linking CXX executable bin\llvm-modextract.exe
[2328/2550] Building RC object tools\llvm-nm\CMakeFiles\llvm-nm.dir\__\__\resources\windows_version_resource.rc.res
[2329/2550] Building Opts.inc...
[2330/2550] Building ObjcopyOpts.inc...
[2331/2550] Building InstallNameToolOpts.inc...
[2333/2550] Building CXX object tools\llvm-nm\CMakeFiles\llvm-nm.dir\llvm-nm.cpp.obj
[2334/2550] Building CXX object tools\llvm-objdump\CMakeFiles\llvm-objdump.dir\ELFDump.cpp.obj
[2335/2550] Building RC object tools\llvm-mt\CMakeFiles\llvm-mt.dir\__\__\resources\windows_version_resource.rc.res
[2336/2550] Building CXX object tools\llvm-mt\CMakeFiles\llvm-mt.dir\llvm-mt.cpp.obj
---
[2414/2550] Building CXX object tools\llvm-readobj\CMakeFiles\llvm-readobj.dir\WindowsResourceDumper.cpp.obj
[2415/2550] Building CXX object tools\llvm-readobj\CMakeFiles\llvm-readobj.dir\Error.cpp.obj
[2416/2550] Building CXX object tools\llvm-readobj\CMakeFiles\llvm-readobj.dir\WasmDumper.cpp.obj
[2417/2550] Building CXX object tools\llvm-readobj\CMakeFiles\llvm-readobj.dir\ELFDumper.cpp.obj
[2418/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\Delta.cpp.obj
[2419/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceBasicBlocks.cpp.obj
[2421/2550] Generating ../../bin/llvm-readelf.exe
[2421/2550] Generating ../../bin/llvm-readelf.exe
[2422/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\TestRunner.cpp.obj
[2423/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceFunctions.cpp.obj
[2424/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\llvm-reduce.cpp.obj
[2425/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceGlobalVars.cpp.obj
[2426/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceInstructions.cpp.obj
[2427/2550] Building RC object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\__\__\resources\windows_version_resource.rc.res
[2428/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceMetadata.cpp.obj
[2429/2550] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceArguments.cpp.obj
[2431/2550] Building CXX object tools\llvm-jitlink\CMakeFiles\llvm-jitlink.dir\llvm-jitlink.cpp.obj
[2432/2550] Building RC object tools\llvm-link\CMakeFiles\llvm-link.dir\__\__\resources\windows_version_resource.rc.res
[2433/2550] Building CXX object tools\llvm-jitlink\CMakeFiles\llvm-jitlink.dir\llvm-jitlink-macho.cpp.obj
[2434/2550] Building CXX object tools\llvm-link\CMakeFiles\llvm-link.dir\llvm-link.cpp.obj
---
[2463/2550] Building CXX object tools\yaml2obj\CMakeFiles\yaml2obj.dir\yaml2obj.cpp.obj
[2464/2550] Building RC object tools\yaml2obj\CMakeFiles\yaml2obj.dir\__\__\resources\windows_version_resource.rc.res
[2465/2550] Linking CXX executable bin\yaml2obj.exe
[2466/2550] Linking CXX executable bin\llvm-mca.exe
[2467/2550] Copying llvm-locstats into D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/build/./bin
[2469/2550] Building CXX object tools\llvm-rtdyld\CMakeFiles\llvm-rtdyld.dir\llvm-rtdyld.cpp.obj
[2470/2550] Linking CXX executable bin\llvm-reduce.exe
[2471/2550] Linking CXX executable bin\llvm-rtdyld.exe
[2472/2550] Building CXX object tools\llvm-size\CMakeFiles\llvm-size.dir\llvm-size.cpp.obj
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/FileEntry.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/FileWriter.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/FunctionInfo.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/GsymCreator.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/GsymReader.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/Header.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/LineEntry.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/LineTable.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/LookupResult.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/DebugInfo/GSYM/Range.h
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/RTDyldMemoryManager.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/RuntimeDyld.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/RuntimeDyldChecker.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/SectionMemoryManager.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Frontend
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Frontend/OpenMP
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Frontend/OpenMP/OMPConstants.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Frontend/OpenMP/OMPIRBuilder.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Frontend/OpenMP/OMPKinds.def
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/FuzzMutate/FuzzerCLI.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/FuzzMutate/IRMutator.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/FuzzMutate/OpDescriptor.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/FuzzMutate/Operations.h
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/DiagnosticInfo.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/DiagnosticPrinter.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/DIBuilder.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/Dominators.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/FixedMetadataKinds.def
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/FPEnv.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/GetElementPtrTypeIterator.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/GlobalAlias.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/GlobalIFunc.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/GlobalIndirectSymbol.h
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/RelocationResolver.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/StackMapParser.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/SymbolicFile.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/SymbolSize.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/TapiFile.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/TapiUniversal.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/WasmTraits.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/WindowsMachineFlag.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/WindowsResource.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/XCOFFObjectFile.h
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/FunctionImportUtils.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/GlobalStatus.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/GuardUtils.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/ImportedFunctionsInliningStatistics.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/InjectTLIMappings.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/LCSSA.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/LibCallsShrinkWrap.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/Local.h
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/LoopRotationUtils.h
---
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMBinaryFormat.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMBitReader.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMBitWriter.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMBitstreamReader.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMDWARFLinker.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMFrontendOpenMP.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMInstrumentation.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMAggressiveInstCombine.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMInstCombine.lib
-- Installing: D:/a/1/s/build/x86_64-pc-windows-msvc/llvm/lib/LLVMScalarOpts.lib
---
   Compiling rustc_parse_format v0.0.0 (D:\a\1\s\src\librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (D:\a\1\s\src\librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (D:\a\1\s\src\librustc_hir)
   Compiling rustc_query_system v0.0.0 (D:\a\1\s\src\librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (D:\a\1\s\src\librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (D:\a\1\s\src\librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (D:\a\1\s\src\librustc_ast_lowering)
---
    Finished release [optimized] target(s) in 34m 38s
Copying stage0 rustc from stage0 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Assembling stage1 compiler (x86_64-pc-windows-msvc)
Building sanitizers for x86_64-pc-windows-msvc
running: "cmake" "D:\\a\\1\\s\\src/llvm-project/compiler-rt" "-G" "Ninja" "-DCMAKE_C_COMPILER_TARGET=x86_64-pc-windows-msvc" "-DCOMPILER_RT_BUILD_BUILTINS=OFF" "-DCOMPILER_RT_BUILD_CRT=OFF" "-DCOMPILER_RT_BUILD_LIBFUZZER=OFF" "-DCOMPILER_RT_BUILD_PROFILE=OFF" "-DCOMPILER_RT_BUILD_SANITIZERS=ON" "-DCOMPILER_RT_BUILD_XRAY=OFF" "-DCOMPILER_RT_DEFAULT_TARGET_ONLY=ON" "-DCOMPILER_RT_USE_LIBCXX=OFF" "-DLLVM_CONFIG_PATH=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\llvm-config.exe" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=D:/a/1/s/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/1/s/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro" "-DCMAKE_INSTALL_PREFIX=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\sanitizers" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 9.0.0 with MSVC-like command-line
-- The ASM compiler identification is Clang
-- Found assembler: D:/a/1/s/build/bootstrap/debug/sccache-plus-cl.exe
-- Check for working C compiler: D:/a/1/s/build/bootstrap/debug/sccache-plus-cl.exe
---
-- Looking for __cxa_throw in stdc++
-- Looking for __cxa_throw in stdc++ - not found
-- Performing Test COMPILER_RT_HAS_Z_TEXT
-- Performing Test COMPILER_RT_HAS_Z_TEXT - Failed
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG
-- Performing Test COMPILER_RT_HAS_FUSE_LD_LLD_FLAG - Success
-- Configuring done
-- Generating done
-- Build files have been written to: D:/a/1/s/build/x86_64-pc-windows-msvc/native/sanitizers/build
running: "cmake" "--build" "." "--target" "clang_rt.asan-x86_64" "--config" "Release" "--" "-j" "2"
running: "cmake" "--build" "." "--target" "clang_rt.asan-x86_64" "--config" "Release" "--" "-j" "2"
[1/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_errno.cpp.obj
[2/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_flags.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flags.cpp:15:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flags.cpp:18:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flags.cpp:18:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_flag_parser.h(24,34): warning: unused parameter 'value' [-Wunused-parameter]
  virtual bool Parse(const char *value) { return false; }
                                 ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flags.cpp(102,60): warning: unused parameter 'cf' [-Wunused-parameter]
void RegisterIncludeFlags(FlagParser *parser, CommonFlags *cf) {
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flags.cpp:15:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
7 warnings generated.
[3/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_allocator.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_allocator.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_allocator.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_allocator.cpp:14:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,19): warning: unused parameter 'p' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,27): warning: unused parameter 'size' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,21): warning: unused parameter 'p' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
                    ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,29): warning: unused parameter 'size' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_allocator.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
9 warnings generated.
[4/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_file.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_file.cpp:20:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_file.cpp:20:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_file.cpp:20:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
5 warnings generated.
[5/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_common.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:13:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:15:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,19): warning: unused parameter 'p' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,27): warning: unused parameter 'size' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,21): warning: unused parameter 'p' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
                    ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,29): warning: unused parameter 'size' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:19:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:19:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_placement_new.h(20,63): warning: unused parameter 'sz' [-Wunused-parameter]
inline void *operator new(__sanitizer::operator_new_size_type sz, void *p) {
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_common.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
10 warnings generated.
[6/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_deadlock_detector2.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:13:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(75,56): warning: unused parameter 'pt' [-Wunused-parameter]
  virtual void DestroyPhysicalThread(DDPhysicalThread *pt) {}
                                                       ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(77,52): warning: unused parameter 'ctx' [-Wunused-parameter]
  virtual DDLogicalThread* CreateLogicalThread(u64 ctx) { return nullptr; }
                                                   ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(78,54): warning: unused parameter 'lt' [-Wunused-parameter]
  virtual void DestroyLogicalThread(DDLogicalThread *lt) {}
                                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(80,38): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexInit(DDCallback *cb, DDMutex *m) {}
                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(80,51): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexInit(DDCallback *cb, DDMutex *m) {}
                                                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,44): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,57): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                        ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,65): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                                ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,43): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,56): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                                       ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,64): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                                               ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(83,12): warning: unused parameter 'trylock' [-Wunused-parameter]
      bool trylock) {}
           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,46): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,59): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,67): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(85,41): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexDestroy(DDCallback *cb, DDMutex *m) {}
                                        ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(85,54): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexDestroy(DDCallback *cb, DDMutex *m) {}
                                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(87,43): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual DDReport *GetReport(DDCallback *cb) { return nullptr; }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:14:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:15:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,19): warning: unused parameter 'p' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,27): warning: unused parameter 'size' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,21): warning: unused parameter 'p' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
                    ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,29): warning: unused parameter 'size' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_placement_new.h(20,63): warning: unused parameter 'sz' [-Wunused-parameter]
inline void *operator new(__sanitizer::operator_new_size_type sz, void *p) {
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector2.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
28 warnings generated.
[7/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_stoptheworld_mac.cpp.obj
[8/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_deadlock_detector1.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:13:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(75,56): warning: unused parameter 'pt' [-Wunused-parameter]
  virtual void DestroyPhysicalThread(DDPhysicalThread *pt) {}
                                                       ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(77,52): warning: unused parameter 'ctx' [-Wunused-parameter]
  virtual DDLogicalThread* CreateLogicalThread(u64 ctx) { return nullptr; }
                                                   ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(78,54): warning: unused parameter 'lt' [-Wunused-parameter]
  virtual void DestroyLogicalThread(DDLogicalThread *lt) {}
                                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(80,38): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexInit(DDCallback *cb, DDMutex *m) {}
                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(80,51): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexInit(DDCallback *cb, DDMutex *m) {}
                                                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,44): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,57): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                        ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(81,65): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexBeforeLock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                                ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,43): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,56): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                                       ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(82,64): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexAfterLock(DDCallback *cb, DDMutex *m, bool wlock,
                                                               ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(83,12): warning: unused parameter 'trylock' [-Wunused-parameter]
      bool trylock) {}
           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,46): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,59): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(84,67): warning: unused parameter 'wlock' [-Wunused-parameter]
  virtual void MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {}
                                                                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(85,41): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual void MutexDestroy(DDCallback *cb, DDMutex *m) {}
                                        ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(85,54): warning: unused parameter 'm' [-Wunused-parameter]
  virtual void MutexDestroy(DDCallback *cb, DDMutex *m) {}
                                                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h(87,43): warning: unused parameter 'cb' [-Wunused-parameter]
  virtual DDReport *GetReport(DDCallback *cb) { return nullptr; }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector.h:28:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector.h:28:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_bvgraph.h:17:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:15:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,19): warning: unused parameter 'p' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,27): warning: unused parameter 'size' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,21): warning: unused parameter 'p' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
                    ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,29): warning: unused parameter 'size' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_placement_new.h(20,63): warning: unused parameter 'sz' [-Wunused-parameter]
inline void *operator new(__sanitizer::operator_new_size_type sz, void *p) {
                                                              ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp(76,50): warning: unused parameter 'pt' [-Wunused-parameter]
void DD::DestroyPhysicalThread(DDPhysicalThread *pt) {
                                                 ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp(104,22): warning: unused parameter 'wlock' [-Wunused-parameter]
    DDMutex *m, bool wlock) {
                     ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp(172,61): warning: unused parameter 'wlock' [-Wunused-parameter]
void DD::MutexBeforeUnlock(DDCallback *cb, DDMutex *m, bool wlock) {
                                                            ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp(177,35): warning: unused parameter 'cb' [-Wunused-parameter]
void DD::MutexDestroy(DDCallback *cb,
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_deadlock_detector1.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_deadlock_detector_interface.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
32 warnings generated.
[9/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_netbsd.cpp.obj
[10/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_linux.cpp.obj
[11/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_mac.cpp.obj
[11/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_mac.cpp.obj
[12/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_netbsd.cpp.obj
[13/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_printf.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_printf.cpp:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_printf.cpp:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
5 warnings generated.
[14/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_bsd.cpp.obj
[15/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_freebsd.cpp.obj
[16/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_linux.cpp.obj
[16/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_linux.cpp.obj
[17/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_rtems.cpp.obj
[18/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_solaris.cpp.obj
[19/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_solaris.cpp.obj
[20/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_mac.cpp.obj
[21/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_openbsd.cpp.obj
[22/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_procmaps_common.cpp.obj
[23/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_openbsd.cpp.obj
[24/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_posix.cpp.obj
[25/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_persistent_allocator.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_persistent_allocator.cpp:12:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_persistent_allocator.h:19:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_persistent_allocator.cpp:12:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_persistent_allocator.h:17:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
5 warnings generated.
[26/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_posix.cpp.obj
[27/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_platform_limits_solaris.cpp.obj
[28/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_fuchsia.cpp.obj
[28/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_fuchsia.cpp.obj
[29/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_libignore.cpp.obj
[30/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_linux_s390.cpp.obj
[31/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_libc.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_libc.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_libc.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,19): warning: unused parameter 'p' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                  ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(48,27): warning: unused parameter 'size' [-Wunused-parameter]
  void OnMap(uptr p, uptr size) const { }
                          ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,21): warning: unused parameter 'p' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
                    ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h(49,29): warning: unused parameter 'size' [-Wunused-parameter]
  void OnUnmap(uptr p, uptr size) const { }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_libc.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator_internal.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_allocator.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
9 warnings generated.
[32/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_flag_parser.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flag_parser.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_flag_parser.h:18:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_flag_parser.h:18:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flag_parser.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flag_parser.cpp:13:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_flag_parser.h(24,34): warning: unused parameter 'value' [-Wunused-parameter]
  virtual bool Parse(const char *value) { return false; }
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_flag_parser.cpp:13:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_flag_parser.h:18:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
6 warnings generated.
[33/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_linux.cpp.obj
[34/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_type_traits.cpp.obj
[35/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_termination.cpp.obj
[35/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_termination.cpp.obj
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_termination.cpp:14:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(817,44): warning: unused parameter 'buffer' [-Wunused-parameter]
INLINE void LogFullErrorReport(const char *buffer) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(824,46): warning: unused parameter 's' [-Wunused-parameter]
INLINE void WriteOneLineToSyslog(const char *s) {}
                                             ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(825,44): warning: unused parameter 'str' [-Wunused-parameter]
INLINE void LogMessageOnPrintf(const char *str) {}
                                           ^
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h(842,41): warning: unused parameter 'buffer_unused' [-Wunused-parameter]
INLINE void AndroidLogWrite(const char *buffer_unused) {}
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common\sanitizer_termination.cpp:14:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_common.h:23:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_mutex.h:16:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
In file included from D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic.h:63:
D:\a\1\s\src\llvm-project\compiler-rt\lib\sanitizer_common/sanitizer_atomic_clang.h(76,57): warning: unused parameter 'mo' [-Wunused-parameter]
                                           memory_order mo) {
5 warnings generated.
[36/111] Building CXX object lib\sanitizer_common\CMakeFiles\RTSanitizerCommon.x86_64.dir\sanitizer_suppressions.cpp.obj
---
warning: C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\include\vcruntime.h(245,47): note: expanded from macro '_CRT_DEPRECATE_TEXT'
warning: #define _CRT_DEPRECATE_TEXT(_Text) __declspec(deprecated(_Text))
warning:                                               ^
warning: ../llvm-project/compiler-rt\lib\profile\InstrProfilingFile.c(398,21): warning: 'getenv' is deprecated: This function or variable may be unsafe. Consider using _dupenv_s instead. To disable deprecation, use _CRT_SECURE_NO_WARNINGS. See online help for details. [-Wdeprecated-declarations]
warning:   int initialized = getenv(LPROF_INIT_ONCE_ENV) != NULL;
warning: C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\ucrt\stdlib.h(1190,20): note: 'getenv' has been explicitly marked deprecated here
warning:     _Check_return_ _CRT_INSECURE_DEPRECATE(_dupenv_s)
warning:                    ^
warning: C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\include\vcruntime.h(255,55): note: expanded from macro '_CRT_INSECURE_DEPRECATE'
---
   Compiling rustc_parse_format v0.0.0 (D:\a\1\s\src\librustc_parse_format)
   Compiling chalk-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (D:\a\1\s\src\librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (D:\a\1\s\src\librustc_hir)
   Compiling rustc_query_system v0.0.0 (D:\a\1\s\src\librustc_query_system)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (D:\a\1\s\src\librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (D:\a\1\s\src\librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (D:\a\1\s\src\librustc_ast_lowering)
---
.................................................................................................... 1000/10292
......i............................................................................................. 1100/10292
........................................i........................................................... 1200/10292
.............i...................................................................................... 1300/10292
....................................................iiii............................................ 1400/10292
.................................................................................................... 1600/10292
.................................................................................................... 1700/10292
.......................................................................................i............ 1800/10292
.................................................................................................... 1900/10292
---
..............................i...............i..................................................... 5200/10292
.................................................................................................... 5300/10292
..............................................................................i..................... 5400/10292
...........................................................................i........................ 5500/10292
.........................................................................................ii.ii...... 5600/10292
..i...i............................................................................................. 5700/10292
........................................i..................................i........................ 5900/10292
..................i...........................................................................ii.... 6000/10292
.................................i.................................................................. 6100/10292
.................................................................................................... 6200/10292
.................................................................................................... 6200/10292
.................................................................................................... 6300/10292
........................................................ii...i..ii...........i...................... 6400/10292
......................test [ui] ui\mpsc_stress.rs has been running for over 60 seconds
.................................................................................................... 6600/10292
.................................................................................................... 6700/10292
.................................................................................................... 6700/10292
......................................................................................i..i..ii...... 6800/10292
.................................................................................................... 7000/10292
.................................................................................................... 7100/10292
...........................................i........................................................ 7200/10292
.................................................................................................... 7300/10292
---
.................................................................................................... 8200/10292
.................................................................................................... 8300/10292
...................................................................................i................ 8400/10292
.................................................................................................... 8500/10292
......................................FiiiiF.FiFFi.F..........F..................................... 8600/10292
.....................................................................ii............................. 8700/10292
.................................................................................................... 8900/10292
............................i....................................................................... 9000/10292
.................................................................................................... 9100/10292
.................................................................................................... 9200/10292
.................................................................................................... 9200/10292
..........iiiiiii................................................................i................i. 9300/10292
.................................................................................................... 9500/10292
.................................................................................................... 9600/10292
...........................................................................i........................ 9700/10292
.................................................................................................... 9800/10292
---
---- [ui] ui\sanitize\address.rs stdout ----

error: error pattern ' AddressSanitizer: stack-buffer-overflow' not found!

error: error pattern ' 'xs' (line 15) <== Memory access at offset' not found!
error: multiple error patterns not found
status: exit code: 1
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\address\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==4376==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------



---- [ui] ui\sanitize\badfree.rs stdout ----

error: error pattern ' AddressSanitizer: SEGV' not found!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\badfree\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==5296==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------



---- [ui] ui\sanitize\leak.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\bin;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "D:\\a\\1\\s\\src/test\\ui\\sanitize\\leak.rs" "-Zthreads=1" "--target=x86_64-pc-windows-msvc" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\leak\\a.exe" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "-Z" "sanitizer=leak" "-O" "-L" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\leak\\auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: LeakSanitizer only works with targets: aarch64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-unknown-linux-gnu
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui\sanitize\issue-72154-lifetime-markers.rs stdout ----

error: test run failed!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\issue-72154-lifetime-markers\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==5308==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------



---- [ui] ui\sanitize\new-llvm-pass-manager-thin-lto.rs#opt0 stdout ----

error in revision `opt0`: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\new-llvm-pass-manager-thin-lto.opt0\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==3324==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------



---- [ui] ui\sanitize\use-after-scope.rs stdout ----

error: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\use-after-scope\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==4832==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------



---- [ui] ui\sanitize\new-llvm-pass-manager-thin-lto.rs#opt1 stdout ----

error in revision `opt1`: error pattern ' ERROR: AddressSanitizer: stack-use-after-scope' not found!
status: exit code: 1
command: PATH="D:\a\1\s\build\x86_64-pc-windows-msvc\stage2\lib\rustlib\x86_64-pc-windows-msvc\lib;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64;C:\Program Files (x86)\Microsoft Visual Studio\2017\Enterprise\VC\Tools\MSVC\14.16.27023\bin\HostX64\x64;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0-bootstrap-tools\x86_64-pc-windows-msvc\release\deps;D:\a\1\s\build\x86_64-pc-windows-msvc\stage0\bin;D:\a\1\s\ninja;D:\a\1\s\msys2\mingw64\bin;C:\hostedtoolcache\windows\Python\3.8.3\x64\Scripts;C:\hostedtoolcache\windows\Python\3.8.3\x64;D:\a\1\s\msys2\usr\bin;C:\Program Files (x86)\Inno Setup 5;D:\a\1\s\sccache;C:\agents\2.170.1\externals\git\cmd;C:\Program Files\Mercurial;C:\Program Files\MongoDB\Server\4.2\bin;C:\ProgramData\kind;C:\vcpkg;C:\cf-cli;C:\Program Files (x86)\NSIS;C:\Program Files\Mercurial;C:\hostedtoolcache\windows\stack\2.3.1\x64;C:\ProgramData\chocolatey\lib\ghc.8.10.1\tools\ghc-8.10.1\bin;C:\Program Files\dotnet;C:\mysql-5.7.21-winx64\bin;C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64\bin;C:\SeleniumWebDrivers\GeckoDriver;C:\Program Files (x86)\sbt\bin;C:\Rust\.cargo\bin;C:\hostedtoolcache\windows\go\1.14.4\x64\bin;C:\Program Files (x86)\GitHub CLI;C:\Program Files\Git\bin;C:\hostedtoolcache\windows\Python\3.7.7\x64\Scripts;C:\hostedtoolcache\windows\Python\3.7.7\x64;C:\hostedtoolcache\windows\Ruby\2.5.8\x64\bin;C:\Program Files\PostgreSQL\12\bin;C:\npm\prefix;C:\Program Files (x86)\Microsoft SDKs\Azure\CLI2\wbin;C:\Program Files\Microsoft MPI\Bin;C:\windows\system32;C:\windows;C:\windows\System32\Wbem;C:\windows\System32\WindowsPowerShell\v1.0;C:\ProgramData\Chocolatey\bin;C:\Program Files\Docker;C:\Program Files\PowerShell\7;C:\Program Files\dotnet;C:\Program Files\Microsoft SQL Server\130\Tools\Binn;C:\Program Files (x86)\Microsoft SQL Server\110\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\120\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\130\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\140\DTS\Binn;C:\Program Files (x86)\Microsoft SQL Server\150\DTS\Binn;C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit;C:\Program Files\Microsoft Service Fabric\bin\Fabric\Fabric.Code;C:\Program Files\Microsoft SDKs\Service Fabric\Tools\ServiceFabricLocalClusterManager;C:\Program Files\nodejs;C:\Program Files\PostgreSQL\12\bin;C:\Program Files\OpenSSL\bin;C:\Strawberry\c\bin;C:\Strawberry\perl\site\bin;C:\Strawberry\perl\bin;C:\Program Files\Git\cmd;C:\Program Files\Git\mingw64\bin;C:\Program Files\Git\usr\bin;C:\tools\php;C:\Program Files (x86)\sbt\bin;C:\Program Files (x86)\Subversion\bin;C:\SeleniumWebDrivers\ChromeDriver;C:\SeleniumWebDrivers\EdgeDriver;C:\ProgramData\chocolatey\lib\maven\apache-maven-3.6.3\bin;C:\Program Files\CMake\bin;C:\Program Files\Amazon\AWSCLIV2;C:\Program Files\Amazon\AWSSAMCLI\bin;C:\Users\VssAdministrator\.dotnet\tools;C:\Users\VssAdministrator\AppData\Local\Microsoft\WindowsApps;D:\a\1\s\msys2\mingw64\bin\" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui\\sanitize\\new-llvm-pass-manager-thin-lto.opt1\\a.exe"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
==3900==AddressSanitizer CHECK failed: D:\a\1\s\src\llvm-project\compiler-rt\lib\asan\..\sanitizer_common/sanitizer_common_interceptors.inc:9765 "((__interception::real_memcpy)) != (0)" (0x0, 0x0)
    <empty stack>

------------------------------------------


---
test result: FAILED. 10196 passed; 7 failed; 89 ignored; 0 measured; 0 filtered out



command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--src-base" "D:\\a\\1\\s\\src/test\\ui" "--build-base" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\test\\ui" "--stage-id" "stage2-x86_64-pc-windows-msvc" "--mode" "ui" "--target" "x86_64-pc-windows-msvc" "--host" "x86_64-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python3.exe" "--lldb-python" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\python3.exe" "--gdb" "D:\\a\\1\\s\\msys2\\mingw64\\bin\\gdb" "--quiet" "--llvm-version" "10.0.1-rust-1.46.0-nightly" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test src/test/ui src/test/compile-fail src/tools/linkchecker
Build completed unsuccessfully in 1:52:57
Build completed unsuccessfully in 1:52:57
make: *** [Makefile:81: ci-subset-2] Error 1
  local time: Wed Jun 10 21:15:43 CUT 2020
  network time: Wed, 10 Jun 2020 21:15:43 GMT
== end clock drift check ==


##[error]Bash exited with code '2'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73221/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73221/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3636) (python)
Terminate orphan process: pid (5240) (sccache)
