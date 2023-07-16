plain
2019-07-12T15:31:11.5072622Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-12T15:31:11.5072669Z 
2019-07-12T15:31:11.5072882Z   git checkout -b <new-branch-name>
2019-07-12T15:31:11.5072922Z 
2019-07-12T15:31:11.5073400Z HEAD is now at 37ed90a76 Auto merge of #62592 - nikic:actually-update-llvm, r=alexcrichton
2019-07-12T15:31:11.5213602Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-12T15:31:11.5217354Z ==============================================================================
2019-07-12T15:31:11.5217467Z Task         : Bash
2019-07-12T15:31:11.5217538Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-12T15:57:14.1503044Z -- Detecting CXX compiler ABI info
2019-07-12T15:57:14.2171672Z -- Detecting CXX compiler ABI info - done
2019-07-12T15:57:14.2226833Z -- Detecting CXX compile features
2019-07-12T15:57:14.5438709Z -- Detecting CXX compile features - done
2019-07-12T15:57:14.5648772Z -- Could NOT find Z3: Found unsuitable version "0.0.0", but required is at least "4.7.1" (found Z3_LIBRARIES-NOTFOUND)
2019-07-12T15:57:14.6413981Z -- Looking for dlfcn.h - found
2019-07-12T15:57:14.6418239Z -- Looking for errno.h
2019-07-12T15:57:14.7074610Z -- Looking for errno.h - found
2019-07-12T15:57:14.7078489Z -- Looking for fcntl.h
---
2019-07-12T15:57:20.4212287Z -- Performing Test C_SUPPORTS_FDATA_SECTIONS
2019-07-12T15:57:20.4876561Z -- Performing Test C_SUPPORTS_FDATA_SECTIONS - Success
2019-07-12T15:57:20.4888473Z -- Performing Test CXX_SUPPORTS_FDATA_SECTIONS
2019-07-12T15:57:20.5593882Z -- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
2019-07-12T15:57:20.5613762Z -- Looking for os_signpost_interval_begin
2019-07-12T15:57:20.5840675Z -- Looking for os_signpost_interval_begin - not found
2019-07-12T15:57:20.5962518Z -- Constructing LLVMBuild project information
2019-07-12T15:57:20.5962518Z -- Constructing LLVMBuild project information
2019-07-12T15:57:20.8042416Z -- Found Git: /usr/bin/git (found version "2.7.4") 
2019-07-12T15:57:21.2020261Z -- Targeting X86
2019-07-12T15:57:21.3227691Z -- Targeting ARM
2019-07-12T15:57:21.4467192Z -- Targeting AArch64
2019-07-12T15:57:21.5753493Z -- Targeting Mips
---
2019-07-12T15:57:41.9903651Z [  5%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Minidump.cpp.o
2019-07-12T15:57:42.0448301Z [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringExtras.cpp.o
2019-07-12T15:57:42.1971894Z [  5%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocument.cpp.o
2019-07-12T15:57:42.2985366Z [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringMap.cpp.o
2019-07-12T15:57:42.4894992Z [  5%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackDocumentYAML.cpp.o
2019-07-12T15:57:42.7156402Z [  5%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackReader.cpp.o
2019-07-12T15:57:42.7544562Z [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringSaver.cpp.o
2019-07-12T15:57:42.9595854Z [  5%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackWriter.cpp.o
2019-07-12T15:57:42.9848498Z [  5%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/StringRef.cpp.o
---
2019-07-12T15:57:59.3686398Z [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/CodeViewYAMLSymbols.cpp.o
2019-07-12T15:57:59.4932747Z [ 11%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/ExecuteStage.cpp.o
2019-07-12T15:57:59.6366882Z [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/CodeViewYAMLTypeHashing.cpp.o
2019-07-12T15:57:59.7175552Z [ 11%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/InstructionTables.cpp.o
2019-07-12T15:57:59.9491015Z [ 11%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/MicroOpQueueStage.cpp.o
2019-07-12T15:58:00.1969109Z [ 11%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/RetireStage.cpp.o
2019-07-12T15:58:00.2227898Z [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/COFFYAML.cpp.o
2019-07-12T15:58:00.4567369Z [ 11%] Building CXX object lib/MCA/CMakeFiles/LLVMMCA.dir/Stages/Stage.cpp.o
2019-07-12T15:58:00.4946262Z [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/DWARFEmitter.cpp.o
---
2019-07-12T15:58:01.7864861Z [ 11%] Building CXX object lib/Option/CMakeFiles/LLVMOption.dir/OptTable.cpp.o
2019-07-12T15:58:01.8895777Z [ 11%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/ObjectYAML.cpp.o
2019-07-12T15:58:02.0534038Z [ 11%] Linking CXX static library ../libLLVMOption.a
2019-07-12T15:58:02.0977830Z [ 11%] Built target LLVMOption
2019-07-12T15:58:02.1247600Z Scanning dependencies of target LLVMRemarks
2019-07-12T15:58:02.1357169Z [ 12%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/Remark.cpp.o
2019-07-12T15:58:02.1622717Z [ 12%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/MinidumpYAML.cpp.o
2019-07-12T15:58:02.3640665Z [ 12%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkParser.cpp.o
2019-07-12T15:58:02.4280014Z [ 12%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/WasmYAML.cpp.o
2019-07-12T15:58:02.6182215Z [ 12%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/RemarkStringTable.cpp.o
2019-07-12T15:58:02.6781401Z [ 12%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/XCOFFYAML.cpp.o
2019-07-12T15:58:02.8558861Z [ 12%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkParser.cpp.o
2019-07-12T15:58:02.9416533Z [ 12%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/YAML.cpp.o
2019-07-12T15:58:03.1136542Z [ 12%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkSerializer.cpp.o
2019-07-12T15:58:03.2719498Z [ 12%] Built target LLVMObjectYAML
2019-07-12T15:58:03.3650120Z [ 12%] Linking CXX static library ../libLLVMRemarks.a
2019-07-12T15:58:03.3936398Z [ 12%] Built target LLVMRemarks
2019-07-12T15:58:03.4128355Z Scanning dependencies of target LLVMDebugInfoGSYM
2019-07-12T15:58:03.4128355Z Scanning dependencies of target LLVMDebugInfoGSYM
2019-07-12T15:58:03.4234797Z [ 12%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/FunctionInfo.cpp.o
2019-07-12T15:58:03.4529032Z [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAbbreviationDeclaration.cpp.o
2019-07-12T15:58:03.4529032Z [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAbbreviationDeclaration.cpp.o
2019-07-12T15:58:03.6717248Z [ 12%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/InlineInfo.cpp.o
2019-07-12T15:58:03.7355416Z [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAddressRange.cpp.o
2019-07-12T15:58:03.9106517Z [ 12%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/Range.cpp.o
2019-07-12T15:58:03.9800838Z [ 12%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAcceleratorTable.cpp.o
2019-07-12T15:58:04.1474040Z [ 13%] Linking CXX static library ../../libLLVMDebugInfoGSYM.a
2019-07-12T15:58:04.1879916Z [ 13%] Built target LLVMDebugInfoGSYM
2019-07-12T15:58:04.3016793Z [ 13%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFContext.cpp.o
2019-07-12T15:58:04.4851605Z [ 13%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDataExtractor.cpp.o
2019-07-12T15:58:04.6275623Z [ 13%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFDebugAbbrev.cpp.o
2019-07-12T15:58:04.7978996Z Scanning dependencies of target LLVMDebugInfoMSF
---
2019-07-12T15:58:18.9287287Z [ 18%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/MachO/PackedVersion.cpp.o
2019-07-12T15:58:19.0376630Z [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeTypedef.cpp.o
2019-07-12T15:58:19.1369560Z [ 18%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/MachO/Symbol.cpp.o
2019-07-12T15:58:19.3049122Z [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeUDT.cpp.o
2019-07-12T15:58:19.3698340Z [ 18%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/MachO/TextStub.cpp.o
2019-07-12T15:58:19.5462028Z [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeVTable.cpp.o
2019-07-12T15:58:19.6874783Z [ 18%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/MachO/TextStubCommon.cpp.o
2019-07-12T15:58:19.9546167Z [ 18%] Linking CXX static library ../libLLVMTextAPI.a
2019-07-12T15:58:19.9834634Z [ 18%] Built target LLVMTextAPI
2019-07-12T15:58:20.0204563Z [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolUnknown.cpp.o
2019-07-12T15:58:20.0295930Z Scanning dependencies of target LLVMXRay
---
2019-07-12T15:59:07.6867025Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LoopVersioningLICM.cpp.o
2019-07-12T15:59:08.0608119Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerAtomic.cpp.o
2019-07-12T15:59:08.3857511Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerExpectIntrinsic.cpp.o
2019-07-12T15:59:08.7254409Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerGuardIntrinsic.cpp.o
2019-07-12T15:59:09.0540232Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/LowerWidenableCondition.cpp.o
2019-07-12T15:59:09.7235593Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/MemCpyOptimizer.cpp.o
2019-07-12T15:59:10.0715431Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/MergeICmps.cpp.o
2019-07-12T15:59:10.4186228Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/MergedLoadStoreMotion.cpp.o
2019-07-12T15:59:10.7606483Z [ 27%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/NaryReassociate.cpp.o
---
2019-07-12T16:00:11.3757967Z [ 37%] Building CXX object lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/ExternalFunctions.cpp.o
2019-07-12T16:00:11.7160519Z [ 37%] Building CXX object lib/ExecutionEngine/Interpreter/CMakeFiles/LLVMInterpreter.dir/Interpreter.cpp.o
2019-07-12T16:00:12.0440881Z [ 37%] Linking CXX static library ../../libLLVMInterpreter.a
2019-07-12T16:00:12.0759585Z [ 37%] Built target LLVMInterpreter
2019-07-12T16:00:12.1094513Z Scanning dependencies of target LLVMJITLink
2019-07-12T16:00:12.1297716Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLink.cpp.o
2019-07-12T16:00:12.3942908Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkGeneric.cpp.o
2019-07-12T16:00:12.6628273Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/JITLinkMemoryManager.cpp.o
2019-07-12T16:00:12.9233356Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/EHFrameSupport.cpp.o
2019-07-12T16:00:14.1511148Z [ 37%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/GDBRegistrationListener.cpp.o
2019-07-12T16:00:15.7282285Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO.cpp.o
2019-07-12T16:00:16.0761469Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_x86_64.cpp.o
2019-07-12T16:00:16.1495896Z [ 37%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/SectionMemoryManager.cpp.o
2019-07-12T16:00:16.3613088Z [ 37%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachOAtomGraphBuilder.cpp.o
2019-07-12T16:00:16.6343088Z [ 37%] Linking CXX static library ../../libLLVMJITLink.a
2019-07-12T16:00:16.6654530Z [ 37%] Built target LLVMJITLink
2019-07-12T16:00:16.7134638Z [ 37%] Building CXX object lib/ExecutionEngine/MCJIT/CMakeFiles/LLVMMCJIT.dir/MCJIT.cpp.o
2019-07-12T16:00:18.0477789Z [ 37%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/TargetSelect.cpp.o
2019-07-12T16:00:18.3760205Z [ 37%] Linking CXX static library ../libLLVMExecutionEngine.a
2019-07-12T16:00:18.4052089Z [ 37%] Built target LLVMExecutionEngine
---
2019-07-12T16:01:28.2478650Z [ 44%] Built target RcTableGen
2019-07-12T16:01:28.2573207Z Scanning dependencies of target ObjcopyOptsTableGen
2019-07-12T16:01:28.2673950Z [ 44%] Building ObjcopyOpts.inc...
2019-07-12T16:01:28.2788872Z [ 44%] Built target ObjcopyOptsTableGen
2019-07-12T16:01:28.2890968Z Scanning dependencies of target LipoOptsTableGen
2019-07-12T16:01:28.2996762Z [ 44%] Building LipoOpts.inc...
2019-07-12T16:01:28.3101717Z [ 44%] Built target LipoOptsTableGen
2019-07-12T16:01:28.3308130Z [ 44%] Building Opts.inc...
2019-07-12T16:01:28.3451915Z [ 44%] Built target MtTableGen
2019-07-12T16:01:28.3558680Z Scanning dependencies of target CvtResTableGen
2019-07-12T16:01:28.3665089Z [ 44%] Building Opts.inc...
---
2019-07-12T16:06:33.2431148Z [ 56%] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/RegBankSelect.cpp.o
2019-07-12T16:06:33.5279222Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/StackProtector.cpp.o
2019-07-12T16:06:33.9783806Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/StackSlotColoring.cpp.o
2019-07-12T16:06:39.4483483Z [ 56%] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/RegisterBank.cpp.o
2019-07-12T16:06:40.1677422Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/SwiftErrorValueTracking.cpp.o
2019-07-12T16:06:40.6057050Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/SwitchLoweringUtils.cpp.o
2019-07-12T16:06:46.1348203Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TailDuplication.cpp.o
2019-07-12T16:06:46.4934180Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TailDuplicator.cpp.o
2019-07-12T16:06:47.7539413Z [ 56%] Building CXX object lib/CodeGen/GlobalISel/CMakeFiles/LLVMGlobalISel.dir/Utils.cpp.o
2019-07-12T16:06:52.4039740Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetFrameLoweringImpl.cpp.o
2019-07-12T16:06:52.4039740Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetFrameLoweringImpl.cpp.o
2019-07-12T16:06:52.7559759Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetInstrInfo.cpp.o
2019-07-12T16:06:52.9889249Z [ 56%] Linking CXX static library ../../libLLVMGlobalISel.a
2019-07-12T16:06:53.0306162Z [ 56%] Built target LLVMGlobalISel
2019-07-12T16:06:53.0824531Z Scanning dependencies of target LLVMBitReader
2019-07-12T16:06:53.0974985Z [ 56%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeAnalyzer.cpp.o
2019-07-12T16:06:53.2534829Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetLoweringBase.cpp.o
2019-07-12T16:06:53.4196492Z /checkout/src/llvm-project/llvm/lib/Bitcode/Reader/BitcodeAnalyzer.cpp: In member function 'llvm::Error llvm::BitcodeAnalyzer::analyze(llvm::Optional<llvm::BCDumpOptions>, llvm::Optional<llvm::StringRef>)':
2019-07-12T16:06:53.4197203Z /checkout/src/llvm-project/llvm/lib/Bitcode/Reader/BitcodeAnalyzer.cpp:549:23: warning: variable 'BlockInfoStreamType' set but not used [-Wunused-but-set-variable]
2019-07-12T16:06:53.4197323Z      CurStreamTypeType BlockInfoStreamType;
2019-07-12T16:06:53.4311404Z [ 56%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitReader.cpp.o
2019-07-12T16:06:53.6920796Z [ 56%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeReader.cpp.o
2019-07-12T16:07:00.0175609Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetLoweringObjectFileImpl.cpp.o
2019-07-12T16:07:00.4697433Z [ 56%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/TargetOptionsImpl.cpp.o
---
2019-07-12T16:07:57.3622244Z Scanning dependencies of target LLVMInstCombine
2019-07-12T16:07:57.3899379Z [ 59%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstructionCombining.cpp.o
2019-07-12T16:08:02.2750292Z [ 59%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopRotationUtils.cpp.o
2019-07-12T16:08:06.8014280Z [ 59%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAddSub.cpp.o
2019-07-12T16:08:07.2801903Z [ 59%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineAtomicRMW.cpp.o
2019-07-12T16:08:08.0152528Z [ 59%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCalls.cpp.o
2019-07-12T16:08:08.4110223Z [ 60%] Building CXX object lib/Transforms/InstCombine/CMakeFiles/LLVMInstCombine.dir/InstCombineCasts.cpp.o
2019-07-12T16:08:08.6642894Z [ 60%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopSimplify.cpp.o
2019-07-12T16:08:08.9652302Z [ 60%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LoopUnroll.cpp.o
---
2019-07-12T16:12:09.8994089Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86RegisterInfo.cpp.o
2019-07-12T16:12:12.9707249Z [ 66%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMLoadStoreOptimizer.cpp.o
2019-07-12T16:12:16.6478199Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86RetpolineThunks.cpp.o
2019-07-12T16:12:17.2569616Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SelectionDAGInfo.cpp.o
2019-07-12T16:12:21.5420489Z [ 66%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMLowOverheadLoops.cpp.o
2019-07-12T16:12:23.9385798Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeLoadHardening.cpp.o
2019-07-12T16:12:27.7052766Z [ 66%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMMCInstLower.cpp.o
2019-07-12T16:12:33.1362465Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86Subtarget.cpp.o
2019-07-12T16:12:33.8132234Z [ 66%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetMachine.cpp.o
---
2019-07-12T16:14:18.5553848Z Scanning dependencies of target LLVMMipsCodeGen
2019-07-12T16:14:18.6183486Z [ 68%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16FrameLowering.cpp.o
2019-07-12T16:14:22.9085378Z [ 68%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ConditionalCompares.cpp.o
2019-07-12T16:14:23.3892138Z [ 68%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64DeadRegisterDefinitionsPass.cpp.o
2019-07-12T16:14:23.8872512Z [ 68%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64ExpandImm.cpp.o
2019-07-12T16:14:24.6808410Z [ 68%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16HardFloat.cpp.o
2019-07-12T16:14:30.4441314Z [ 68%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64FalkorHWPFFix.cpp.o
2019-07-12T16:14:30.6174970Z [ 68%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16HardFloatInfo.cpp.o
2019-07-12T16:14:30.7611610Z [ 68%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/Mips16InstrInfo.cpp.o
---
2019-07-12T16:19:28.9128320Z [ 76%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCVSXFMAMutate.cpp.o
2019-07-12T16:19:33.1176967Z [ 76%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZMCInstLower.cpp.o
2019-07-12T16:19:35.0001482Z [ 76%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCVSXSwapRemoval.cpp.o
2019-07-12T16:19:35.4843359Z [ 76%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCExpandISEL.cpp.o
2019-07-12T16:19:39.1160692Z [ 76%] Building CXX object lib/Target/SystemZ/CMakeFiles/LLVMSystemZCodeGen.dir/SystemZPostRewrite.cpp.o
2019-07-12T16:19:41.3245120Z [ 76%] Building CXX object lib/Target/PowerPC/CMakeFiles/LLVMPowerPCCodeGen.dir/PPCPreEmitPeephole.cpp.o
2019-07-12T16:19:41.7463431Z [ 76%] Linking CXX static library ../../libLLVMPowerPCCodeGen.a
2019-07-12T16:19:41.8015334Z [ 76%] Built target LLVMPowerPCCodeGen
2019-07-12T16:19:41.8722392Z Scanning dependencies of target LLVMSystemZAsmParser
---
2019-07-12T16:26:13.6282792Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/MCInstrDescView.cpp.o
2019-07-12T16:26:13.8885935Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/PerfHelper.cpp.o
2019-07-12T16:26:14.6267110Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterAliasing.cpp.o
2019-07-12T16:26:14.6267962Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/RegisterValue.cpp.o
2019-07-12T16:26:14.6318230Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/SchedClassResolution.cpp.o
2019-07-12T16:26:15.3587700Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Target.cpp.o
2019-07-12T16:26:15.7776033Z [ 86%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Uops.cpp.o
2019-07-12T16:26:16.2104507Z [ 86%] Linking CXX static library ../../../lib/libLLVMExegesis.a
2019-07-12T16:26:16.2464905Z [ 86%] Built target LLVMExegesis
---
2019-07-12T16:27:05.1368586Z Scanning dependencies of target bugpoint
2019-07-12T16:27:05.1544618Z [ 93%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/BugDriver.cpp.o
2019-07-12T16:27:05.1961350Z [ 93%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/ELF/Object.cpp.o
2019-07-12T16:27:05.4188099Z [ 93%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/CrashDebugger.cpp.o
2019-07-12T16:27:05.5673771Z [ 93%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/MachO/MachOObjcopy.cpp.o
2019-07-12T16:27:05.9368456Z [ 93%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/MachO/MachOReader.cpp.o
2019-07-12T16:27:08.6736862Z [ 93%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/MachO/MachOWriter.cpp.o
2019-07-12T16:27:11.3155422Z [ 93%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/MachO/Object.cpp.o
2019-07-12T16:27:12.3971434Z [ 93%] Built target llvm-objcopy
2019-07-12T16:27:12.4192925Z Scanning dependencies of target llvm-lipo
2019-07-12T16:27:12.4192925Z Scanning dependencies of target llvm-lipo
2019-07-12T16:27:12.4318414Z [ 93%] Building CXX object tools/llvm-lipo/CMakeFiles/llvm-lipo.dir/llvm-lipo.cpp.o
2019-07-12T16:27:12.7106075Z [ 93%] Linking CXX executable ../../bin/llvm-lipo
2019-07-12T16:27:13.4130438Z [ 93%] Built target llvm-lipo
2019-07-12T16:27:13.6152177Z [ 93%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/attributes.c.o
2019-07-12T16:27:13.7241252Z [ 93%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/calc.c.o
2019-07-12T16:27:13.8420790Z [ 93%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/debuginfo.c.o
2019-07-12T16:27:13.9501306Z [ 93%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/diagnostic.c.o
---
2019-07-12T16:27:59.1890316Z [ 95%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/coff2yaml.cpp.o
2019-07-12T16:27:59.2906494Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/CodeRegionGenerator.cpp.o
2019-07-12T16:27:59.5146563Z [ 96%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/dwarf2yaml.cpp.o
2019-07-12T16:27:59.5841273Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/PipelinePrinter.cpp.o
2019-07-12T16:27:59.9054807Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/BottleneckAnalysis.cpp.o
2019-07-12T16:28:00.4833860Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/InstructionInfoView.cpp.o
2019-07-12T16:28:00.7482501Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/RegisterFileStatistics.cpp.o
2019-07-12T16:28:01.0269820Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/ResourcePressureView.cpp.o
2019-07-12T16:28:01.2884687Z [ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/RetireControlUnitStatistics.cpp.o
---
2019-07-12T16:29:08.4488723Z [ 99%] Building CXX object tools/llvm-special-case-list-fuzzer/CMakeFiles/llvm-special-case-list-fuzzer.dir/special-case-list-fuzzer.cpp.o
2019-07-12T16:29:08.4755364Z [100%] Linking CXX executable ../../bin/llvm-readobj
2019-07-12T16:29:08.7256140Z [100%] Linking CXX executable ../../bin/llvm-special-case-list-fuzzer
2019-07-12T16:29:09.2665356Z [100%] Built target llvm-readobj
2019-07-12T16:29:09.3065057Z Scanning dependencies of target llvm-jitlink
2019-07-12T16:29:09.3209473Z [100%] Building CXX object tools/llvm-jitlink/CMakeFiles/llvm-jitlink.dir/llvm-jitlink.cpp.o
2019-07-12T16:29:09.4842211Z Scanning dependencies of target llvm-isel-fuzzer
2019-07-12T16:29:10.1732801Z [100%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/DummyISelFuzzer.cpp.o
2019-07-12T16:29:10.1732801Z [100%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/DummyISelFuzzer.cpp.o
2019-07-12T16:29:10.1733835Z [100%] Building CXX object tools/llvm-jitlink/CMakeFiles/llvm-jitlink.dir/llvm-jitlink-macho.cpp.o
2019-07-12T16:29:10.1734432Z [100%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/llvm-isel-fuzzer.cpp.o
2019-07-12T16:29:10.1734897Z [100%] Linking CXX executable ../../bin/llvm-jitlink
2019-07-12T16:29:10.1735370Z [100%] Linking CXX executable ../../bin/llvm-isel-fuzzer
2019-07-12T16:29:10.6924913Z [100%] Built target llvm-jitlink
2019-07-12T16:29:10.7378689Z [100%] Building CXX object tools/llvm-stress/CMakeFiles/llvm-stress.dir/llvm-stress.cpp.o
2019-07-12T16:29:10.7941309Z [100%] Built target llvm-isel-fuzzer
2019-07-12T16:29:10.8205652Z Scanning dependencies of target llvm-undname
2019-07-12T16:29:10.8415927Z [100%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
---
2019-07-12T16:29:29.7095023Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MappedBlockStream.h
2019-07-12T16:29:29.7095583Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFCommon.h
2019-07-12T16:29:29.7095961Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFError.h
2019-07-12T16:29:29.7096313Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/MSF/MSFBuilder.h
2019-07-12T16:29:29.7096822Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM
2019-07-12T16:29:29.7097268Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/Range.h
2019-07-12T16:29:29.7098516Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/StringTable.h
2019-07-12T16:29:29.7099280Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/LineEntry.h
2019-07-12T16:29:29.7099644Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/InlineInfo.h
2019-07-12T16:29:29.7100989Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/FileEntry.h
2019-07-12T16:29:29.7101570Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/DebugInfo/GSYM/FunctionInfo.h
2019-07-12T16:29:29.7103823Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/SetTheory.h
2019-07-12T16:29:29.7104221Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/Record.h
2019-07-12T16:29:29.7104771Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/StringToOffsetTable.h
2019-07-12T16:29:29.7106883Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TableGen/SearchableTable.td
---
2019-07-12T16:29:29.7260118Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LinkAllAsmWriterComponents.h
2019-07-12T16:29:29.7260687Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/AsmPrinterHandler.h
2019-07-12T16:29:29.7261008Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/TargetLowering.h
2019-07-12T16:29:29.7262185Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/DFAPacketizer.h
2019-07-12T16:29:29.7263403Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SwitchLoweringUtils.h
2019-07-12T16:29:29.7265209Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/PreISelIntrinsicLowering.h
2019-07-12T16:29:29.7265863Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachinePassRegistry.h
2019-07-12T16:29:29.7266250Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/LiveRangeEdit.h
2019-07-12T16:29:29.7267587Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MIRPrinter.h
---
2019-07-12T16:29:29.7495293Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineCombinerPattern.h
2019-07-12T16:29:29.7495720Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/MachineOperand.h
2019-07-12T16:29:29.7496086Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGISel.h
2019-07-12T16:29:29.7497463Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/UnreachableBlockElim.h
2019-07-12T16:29:29.7510991Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/CSEConfigBase.h
2019-07-12T16:29:29.7544187Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/FastISel.h
2019-07-12T16:29:29.7544599Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGNodes.h
2019-07-12T16:29:29.7545156Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/SelectionDAGTargetInfo.h
2019-07-12T16:29:29.7545576Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/CodeGen/IntrinsicLowering.h
---
2019-07-12T16:29:29.7550734Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/Stage.h
2019-07-12T16:29:29.7551098Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/InstructionTables.h
2019-07-12T16:29:29.7551451Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/ExecuteStage.h
2019-07-12T16:29:29.7551830Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/DispatchStage.h
2019-07-12T16:29:29.7552194Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/MicroOpQueueStage.h
2019-07-12T16:29:29.7553346Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Stages/RetireStage.h
2019-07-12T16:29:29.7554076Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/HWEventListener.h
2019-07-12T16:29:29.7554442Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/InstrBuilder.h
2019-07-12T16:29:29.7554776Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MCA/Instruction.h
---
2019-07-12T16:29:29.7571794Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Binary.h
2019-07-12T16:29:29.7572114Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/StackMapParser.h
2019-07-12T16:29:29.7572408Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/Minidump.h
2019-07-12T16:29:29.7573398Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/CVDebugRecord.h
2019-07-12T16:29:29.7573755Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/XCOFFObjectFile.h
2019-07-12T16:29:29.7579927Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/WindowsMachineFlag.h
2019-07-12T16:29:29.7580316Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/ELFObjectFile.h
2019-07-12T16:29:29.7580615Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/IRSymtab.h
2019-07-12T16:29:29.7581100Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Object/RelocationResolver.h
---
2019-07-12T16:29:29.7655766Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Option/Arg.h
2019-07-12T16:29:29.7656115Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/PassSupport.h
2019-07-12T16:29:29.7656443Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat
2019-07-12T16:29:29.7656911Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/Wasm.h
2019-07-12T16:29:29.7670755Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/XCOFF.h
2019-07-12T16:29:29.7676575Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/Sparc.def
2019-07-12T16:29:29.7677099Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/ARC.def
2019-07-12T16:29:29.7677441Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/i386.def
2019-07-12T16:29:29.7678114Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/BinaryFormat/ELFRelocs/SystemZ.def
---
2019-07-12T16:29:29.7878099Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProfWriter.h
2019-07-12T16:29:29.7878446Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/SampleProf.h
2019-07-12T16:29:29.7878944Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/SampleProfWriter.h
2019-07-12T16:29:29.7879288Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ProfileData/InstrProfData.inc
2019-07-12T16:29:29.7879600Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Remarks
2019-07-12T16:29:29.7879923Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Remarks/Remark.h
2019-07-12T16:29:29.7880243Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Remarks/RemarkSerializer.h
2019-07-12T16:29:29.7880574Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Remarks/RemarkParser.h
2019-07-12T16:29:29.7880909Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Remarks/RemarkStringTable.h
2019-07-12T16:29:29.7881708Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/Demangle.h
2019-07-12T16:29:29.7882120Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/DemangleConfig.h
2019-07-12T16:29:29.7882681Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/StringView.h
2019-07-12T16:29:29.7883235Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Demangle/Utility.h
---
2019-07-12T16:29:29.7890118Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/SyntheticCountsUtils.h
2019-07-12T16:29:29.7890450Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LoopInfoImpl.h
2019-07-12T16:29:29.7890787Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/LegacyDivergenceAnalysis.h
2019-07-12T16:29:29.7891129Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DomTreeUpdater.h
2019-07-12T16:29:29.7891467Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/VecFuncs.def
2019-07-12T16:29:29.7892488Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/RegionIterator.h
2019-07-12T16:29:29.7906218Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/DOTGraphTraitsPass.h
2019-07-12T16:29:29.7908576Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/ConstantFolding.h
2019-07-12T16:29:29.7910697Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Analysis/MemorySSAUpdater.h
---
2019-07-12T16:29:29.8047810Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML
2019-07-12T16:29:29.8048132Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLSymbols.h
2019-07-12T16:29:29.8048457Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypes.h
2019-07-12T16:29:29.8048809Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLDebugSections.h
2019-07-12T16:29:29.8049139Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/XCOFFYAML.h
2019-07-12T16:29:29.8049892Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/DWARFEmitter.h
2019-07-12T16:29:29.8050228Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/WasmYAML.h
2019-07-12T16:29:29.8050228Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/WasmYAML.h
2019-07-12T16:29:29.8050561Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/MinidumpYAML.h
2019-07-12T16:29:29.8051205Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/MachOYAML.h
2019-07-12T16:29:29.8051536Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypeHashing.h
2019-07-12T16:29:29.8051879Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/YAML.h
2019-07-12T16:29:29.8052192Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ObjectYAML/DWARFYAML.h
---
2019-07-12T16:29:29.8105763Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI
2019-07-12T16:29:29.8106099Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/ELF
2019-07-12T16:29:29.8106427Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/ELF/ELFStub.h
2019-07-12T16:29:29.8106767Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/ELF/TBEHandler.h
2019-07-12T16:29:29.8107235Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO
2019-07-12T16:29:29.8107560Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/PackedVersion.h
2019-07-12T16:29:29.8107918Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/InterfaceFile.h
2019-07-12T16:29:29.8108410Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/Architecture.def
2019-07-12T16:29:29.8109107Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/Symbol.h
2019-07-12T16:29:29.8109467Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/TextAPIReader.h
2019-07-12T16:29:29.8109789Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/Architecture.h
2019-07-12T16:29:29.8110095Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/TextAPIWriter.h
2019-07-12T16:29:29.8110418Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/TextAPI/MachO/ArchitectureSet.h
2019-07-12T16:29:29.8111088Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/WindowsManifest/WindowsManifestMerger.h
2019-07-12T16:29:29.8111420Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/InitializePasses.h
2019-07-12T16:29:29.8111731Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target
2019-07-12T16:29:29.8112183Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Target/CodeGenCWrappers.h
---
2019-07-12T16:29:29.8188131Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/IndVarSimplify.h
2019-07-12T16:29:29.8188455Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/CorrelatedValuePropagation.h
2019-07-12T16:29:29.8188787Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/InstSimplifyPass.h
2019-07-12T16:29:29.8189110Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopAccessAnalysisPrinter.h
2019-07-12T16:29:29.8189434Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LoopFuse.h
2019-07-12T16:29:29.8189763Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/MergeICmps.h
2019-07-12T16:29:29.8193043Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LowerWidenableCondition.h
2019-07-12T16:29:29.8194018Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/Reassociate.h
2019-07-12T16:29:29.8194383Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/LICM.h
2019-07-12T16:29:29.8194734Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/Float2Int.h
2019-07-12T16:29:29.8195110Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/Transforms/Scalar/NaryReassociate.h
---
2019-07-12T16:29:29.8257214Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSubtargetInfo.h
2019-07-12T16:29:29.8257533Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCTargetOptions.h
2019-07-12T16:29:29.8257831Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolMachO.h
2019-07-12T16:29:29.8258149Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/ConstantPools.h
2019-07-12T16:29:29.8258450Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSymbolXCOFF.h
2019-07-12T16:29:29.8259072Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCWin64EH.h
2019-07-12T16:29:29.8259676Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCObjectFileInfo.h
2019-07-12T16:29:29.8260036Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSection.h
2019-07-12T16:29:29.8260326Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/MC/MCSectionELF.h
---
2019-07-12T16:29:29.8283494Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetServer.h
2019-07-12T16:29:29.8283933Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/LambdaResolver.h
2019-07-12T16:29:29.8284359Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/GlobalMappingLayer.h
2019-07-12T16:29:29.8284710Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Orc/Core.h
2019-07-12T16:29:29.8285067Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink
2019-07-12T16:29:29.8285425Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink/MachO_x86_64.h
2019-07-12T16:29:29.8285816Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink/JITLinkMemoryManager.h
2019-07-12T16:29:29.8286193Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink/JITLink.h
2019-07-12T16:29:29.8286676Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink/EHFrameSupport.h
2019-07-12T16:29:29.8287082Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/JITLink/MachO.h
2019-07-12T16:29:29.8287725Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/OrcMCJITReplacement.h
2019-07-12T16:29:29.8288025Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/MCJIT.h
2019-07-12T16:29:29.8288346Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/Interpreter.h
2019-07-12T16:29:29.8288670Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/include/llvm/ExecutionEngine/OProfileWrapper.h
---
2019-07-12T16:29:29.9326395Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMObjectYAML.a
2019-07-12T16:29:29.9373916Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOption.a
2019-07-12T16:29:29.9378721Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRemarks.a
2019-07-12T16:29:29.9385954Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoDWARF.a
2019-07-12T16:29:29.9408756Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoGSYM.a
2019-07-12T16:29:29.9419941Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoCodeView.a
2019-07-12T16:29:29.9457393Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMDebugInfoPDB.a
2019-07-12T16:29:29.9517783Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMSymbolize.a
2019-07-12T16:29:29.9518149Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMExecutionEngine.a
2019-07-12T16:29:29.9518149Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMExecutionEngine.a
2019-07-12T16:29:29.9518442Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMInterpreter.a
2019-07-12T16:29:29.9518738Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMJITLink.a
2019-07-12T16:29:29.9529899Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMOrcJIT.a
2019-07-12T16:29:29.9559605Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMRuntimeDyld.a
2019-07-12T16:29:29.9583930Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMTarget.a
2019-07-12T16:29:29.9589460Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVMX86CodeGen.a
---
2019-07-12T16:29:30.2560559Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-readobj
2019-07-12T16:29:30.2580264Z -- Creating llvm-readelf
2019-07-12T16:29:30.2675603Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cov
2019-07-12T16:29:30.2686651Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-mt
2019-07-12T16:29:30.2693052Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-jitlink
2019-07-12T16:29:30.2707656Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-undname
2019-07-12T16:29:30.2712399Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-nm
2019-07-12T16:29:30.2721405Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-lto2
2019-07-12T16:29:30.2728692Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-strings
---
2019-07-12T16:29:30.3051586Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddLLVM.cmake
2019-07-12T16:29:30.3054785Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindOCaml.cmake
2019-07-12T16:29:30.3068285Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./ChooseMSVCCRT.cmake
2019-07-12T16:29:30.3068749Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./GenerateVersionFromVCS.cmake
2019-07-12T16:29:30.3069598Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindZ3.cmake
2019-07-12T16:29:30.3070217Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddSphinxTarget.cmake
2019-07-12T16:29:30.3070758Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindSphinx.cmake
2019-07-12T16:29:30.3071059Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddOCaml.cmake
2019-07-12T16:29:30.3071059Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddOCaml.cmake
2019-07-12T16:29:30.3071358Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./UseLibtool.cmake
2019-07-12T16:29:30.3072328Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddLLVMDefinitions.cmake
2019-07-12T16:29:30.3072635Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./CheckLinkerFlag.cmake
2019-07-12T16:29:30.3073278Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./LLVMProcessSources.cmake
2019-07-12T16:29:30.3074030Z -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./TableGen.cmake
---
2019-07-12T16:31:36.3033461Z [RUSTC-TIMING] rustc_std_workspace_core test:false 0.033
2019-07-12T16:31:37.3460942Z [RUSTC-TIMING] libc test:false 1.040
2019-07-12T16:31:39.2669301Z error: failed to run custom build command for `compiler_builtins v0.1.16`
2019-07-12T16:31:39.2689560Z 
2019-07-12T16:31:39.2689872Z Caused by:
2019-07-12T16:31:39.2690948Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/compiler_builtins-d25dbe74bdbe941f/build-script-build` (exit code: 101)
2019-07-12T16:31:39.2691174Z --- stdout
2019-07-12T16:31:39.2691352Z cargo:rerun-if-changed=build.rs
2019-07-12T16:31:39.2691612Z cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.16/compiler-rt
2019-07-12T16:31:39.2692176Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c
2019-07-12T16:31:39.2692432Z cargo:rustc-cfg=__absvdi2="optimized-c"
2019-07-12T16:31:39.2693012Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c
2019-07-12T16:31:39.2693259Z cargo:rustc-cfg=__absvsi2="optimized-c"
2019-07-12T16:31:39.2693501Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c
2019-07-12T16:31:39.2693738Z cargo:rustc-cfg=__absvti2="optimized-c"
2019-07-12T16:31:39.2693987Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c
2019-07-12T16:31:39.2694388Z cargo:rustc-cfg=__addvdi3="optimized-c"
2019-07-12T16:31:39.2694810Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c
2019-07-12T16:31:39.2695016Z cargo:rustc-cfg=__addvsi3="optimized-c"
2019-07-12T16:31:39.2695263Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c
2019-07-12T16:31:39.2695486Z cargo:rustc-cfg=__addvti3="optimized-c"
2019-07-12T16:31:39.2695761Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
2019-07-12T16:31:39.2695986Z cargo:rustc-cfg=__aeabi_cdcmp="optimized-c"
2019-07-12T16:31:39.2696276Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
2019-07-12T16:31:39.2696825Z cargo:rustc-cfg=__aeabi_cdcmpeq_check_nan="optimized-c"
2019-07-12T16:31:39.2697534Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
2019-07-12T16:31:39.2697805Z cargo:rustc-cfg=__aeabi_cfcmp="optimized-c"
2019-07-12T16:31:39.2698135Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
2019-07-12T16:31:39.2698407Z cargo:rustc-cfg=__aeabi_cfcmpeq_check_nan="optimized-c"
2019-07-12T16:31:39.2698724Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_div0.c
2019-07-12T16:31:39.2698980Z cargo:rustc-cfg=__aeabi_div0="optimized-c"
2019-07-12T16:31:39.2699296Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_drsub.c
2019-07-12T16:31:39.2699566Z cargo:rustc-cfg=__aeabi_drsub="optimized-c"
2019-07-12T16:31:39.2699865Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_frsub.c
2019-07-12T16:31:39.2700134Z cargo:rustc-cfg=__aeabi_frsub="optimized-c"
2019-07-12T16:31:39.2700727Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapdi2.S
2019-07-12T16:31:39.2701320Z cargo:rustc-cfg=__bswapdi2="optimized-c"
2019-07-12T16:31:39.2701547Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapsi2.S
2019-07-12T16:31:39.2701755Z cargo:rustc-cfg=__bswapsi2="optimized-c"
2019-07-12T16:31:39.2701979Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzdi2.S
2019-07-12T16:31:39.2702191Z cargo:rustc-cfg=__clzdi2="optimized-c"
2019-07-12T16:31:39.2702415Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzsi2.S
2019-07-12T16:31:39.2702622Z cargo:rustc-cfg=__clzsi2="optimized-c"
2019-07-12T16:31:39.2702842Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c
2019-07-12T16:31:39.2703065Z cargo:rustc-cfg=__clzti2="optimized-c"
2019-07-12T16:31:39.2703301Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c
2019-07-12T16:31:39.2703529Z cargo:rustc-cfg=__cmpdi2="optimized-c"
2019-07-12T16:31:39.2703871Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c
2019-07-12T16:31:39.2704095Z cargo:rustc-cfg=__cmpti2="optimized-c"
2019-07-12T16:31:39.2704345Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c
2019-07-12T16:31:39.2704552Z cargo:rustc-cfg=__ctzdi2="optimized-c"
2019-07-12T16:31:39.2704801Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c
2019-07-12T16:31:39.2705010Z cargo:rustc-cfg=__ctzsi2="optimized-c"
2019-07-12T16:31:39.2705258Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c
2019-07-12T16:31:39.2705464Z cargo:rustc-cfg=__ctzti2="optimized-c"
2019-07-12T16:31:39.2705715Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c
2019-07-12T16:31:39.2705921Z cargo:rustc-cfg=__divdc3="optimized-c"
2019-07-12T16:31:39.2706187Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divmodsi4.S
2019-07-12T16:31:39.2706572Z cargo:rustc-cfg=__divmodsi4="optimized-c"
2019-07-12T16:31:39.2707375Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c
2019-07-12T16:31:39.2707647Z cargo:rustc-cfg=__divsc3="optimized-c"
2019-07-12T16:31:39.2707977Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divsi3.S
2019-07-12T16:31:39.2708245Z cargo:rustc-cfg=__divsi3="optimized-c"
2019-07-12T16:31:39.2708569Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c
2019-07-12T16:31:39.2708840Z cargo:rustc-cfg=__divxc3="optimized-c"
2019-07-12T16:31:39.2709170Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c
2019-07-12T16:31:39.2709458Z cargo:rustc-cfg=__extendhfsf2="optimized-c"
2019-07-12T16:31:39.2709769Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c
2019-07-12T16:31:39.2710058Z cargo:rustc-cfg=__ffsti2="optimized-c"
2019-07-12T16:31:39.2710536Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S
2019-07-12T16:31:39.2711264Z cargo:rustc-cfg=__fixdfsivfp="optimized-c"
2019-07-12T16:31:39.2711496Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixsfsivfp.S
2019-07-12T16:31:39.2711705Z cargo:rustc-cfg=__fixsfsivfp="optimized-c"
2019-07-12T16:31:39.2711936Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
2019-07-12T16:31:39.2712157Z cargo:rustc-cfg=__fixunsdfsivfp="optimized-c"
2019-07-12T16:31:39.2712387Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunssfsivfp.S
2019-07-12T16:31:39.2712598Z cargo:rustc-cfg=__fixunssfsivfp="optimized-c"
2019-07-12T16:31:39.2712827Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsidfvfp.S
2019-07-12T16:31:39.2713043Z cargo:rustc-cfg=__floatsidfvfp="optimized-c"
2019-07-12T16:31:39.2713354Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsisfvfp.S
2019-07-12T16:31:39.2713610Z cargo:rustc-cfg=__floatsisfvfp="optimized-c"
2019-07-12T16:31:39.2713842Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssidfvfp.S
2019-07-12T16:31:39.2714064Z cargo:rustc-cfg=__floatunssidfvfp="optimized-c"
2019-07-12T16:31:39.2714309Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssisfvfp.S
2019-07-12T16:31:39.2714514Z cargo:rustc-cfg=__floatunssisfvfp="optimized-c"
2019-07-12T16:31:39.2714751Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c
2019-07-12T16:31:39.2714958Z cargo:rustc-cfg=__int_util="optimized-c"
2019-07-12T16:31:39.2715212Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/modsi3.S
2019-07-12T16:31:39.2715417Z cargo:rustc-cfg=__modsi3="optimized-c"
2019-07-12T16:31:39.2715673Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c
2019-07-12T16:31:39.2715882Z cargo:rustc-cfg=__muldc3="optimized-c"
2019-07-12T16:31:39.2716602Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c
2019-07-12T16:31:39.2717169Z cargo:rustc-cfg=__mulsc3="optimized-c"
2019-07-12T16:31:39.2717489Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c
2019-07-12T16:31:39.2717755Z cargo:rustc-cfg=__mulvdi3="optimized-c"
2019-07-12T16:31:39.2718080Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c
2019-07-12T16:31:39.2718342Z cargo:rustc-cfg=__mulvsi3="optimized-c"
2019-07-12T16:31:39.2718664Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c
2019-07-12T16:31:39.2718930Z cargo:rustc-cfg=__mulvti3="optimized-c"
2019-07-12T16:31:39.2719248Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c
2019-07-12T16:31:39.2719526Z cargo:rustc-cfg=__mulxc3="optimized-c"
2019-07-12T16:31:39.2719843Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c
2019-07-12T16:31:39.2720133Z cargo:rustc-cfg=__negdf2="optimized-c"
2019-07-12T16:31:39.2720541Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negdf2vfp.S
2019-07-12T16:31:39.2720752Z cargo:rustc-cfg=__negdf2vfp="optimized-c"
2019-07-12T16:31:39.2720974Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c
2019-07-12T16:31:39.2721190Z cargo:rustc-cfg=__negdi2="optimized-c"
2019-07-12T16:31:39.2721431Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c
2019-07-12T16:31:39.2721655Z cargo:rustc-cfg=__negsf2="optimized-c"
2019-07-12T16:31:39.2721900Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negsf2vfp.S
2019-07-12T16:31:39.2722285Z cargo:rustc-cfg=__negsf2vfp="optimized-c"
2019-07-12T16:31:39.2722498Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c
2019-07-12T16:31:39.2722722Z cargo:rustc-cfg=__negti2="optimized-c"
2019-07-12T16:31:39.2722962Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c
2019-07-12T16:31:39.2723177Z cargo:rustc-cfg=__negvdi2="optimized-c"
2019-07-12T16:31:39.2723406Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c
2019-07-12T16:31:39.2723629Z cargo:rustc-cfg=__negvsi2="optimized-c"
2019-07-12T16:31:39.2723872Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c
2019-07-12T16:31:39.2724075Z cargo:rustc-cfg=__negvti2="optimized-c"
2019-07-12T16:31:39.2724322Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c
2019-07-12T16:31:39.2724528Z cargo:rustc-cfg=__paritydi2="optimized-c"
2019-07-12T16:31:39.2724772Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c
2019-07-12T16:31:39.2724977Z cargo:rustc-cfg=__paritysi2="optimized-c"
2019-07-12T16:31:39.2725303Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c
2019-07-12T16:31:39.2725548Z cargo:rustc-cfg=__parityti2="optimized-c"
2019-07-12T16:31:39.2725802Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c
2019-07-12T16:31:39.2726014Z cargo:rustc-cfg=__popcountdi2="optimized-c"
2019-07-12T16:31:39.2726265Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c
2019-07-12T16:31:39.2726636Z cargo:rustc-cfg=__popcountsi2="optimized-c"
2019-07-12T16:31:39.2727598Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c
2019-07-12T16:31:39.2727884Z cargo:rustc-cfg=__popcountti2="optimized-c"
2019-07-12T16:31:39.2728197Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c
2019-07-12T16:31:39.2728446Z cargo:rustc-cfg=__powixf2="optimized-c"
2019-07-12T16:31:39.2728787Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
2019-07-12T16:31:39.2729082Z cargo:rustc-cfg=__restore_vfp_d8_d15_regs="optimized-c"
2019-07-12T16:31:39.2729525Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
2019-07-12T16:31:39.2729818Z cargo:rustc-cfg=__save_vfp_d8_d15_regs="optimized-c"
2019-07-12T16:31:39.2730109Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c
2019-07-12T16:31:39.2730528Z cargo:rustc-cfg=__subvdi3="optimized-c"
2019-07-12T16:31:39.2730756Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c
2019-07-12T16:31:39.2730981Z cargo:rustc-cfg=__subvsi3="optimized-c"
2019-07-12T16:31:39.2731373Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c
2019-07-12T16:31:39.2731597Z cargo:rustc-cfg=__subvti3="optimized-c"
2019-07-12T16:31:39.2732007Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch16.S
2019-07-12T16:31:39.2732393Z cargo:rustc-cfg=__switch16="optimized-c"
2019-07-12T16:31:39.2741537Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch32.S
2019-07-12T16:31:39.2742142Z cargo:rustc-cfg=__switch32="optimized-c"
2019-07-12T16:31:39.2742372Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch8.S
2019-07-12T16:31:39.2742586Z cargo:rustc-cfg=__switch8="optimized-c"
2019-07-12T16:31:39.2742826Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switchu8.S
2019-07-12T16:31:39.2743021Z cargo:rustc-cfg=__switchu8="optimized-c"
2019-07-12T16:31:39.2743271Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_4.S
2019-07-12T16:31:39.2743480Z cargo:rustc-cfg=__sync_fetch_and_add_4="optimized-c"
2019-07-12T16:31:39.2743734Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S
2019-07-12T16:31:39.2743945Z cargo:rustc-cfg=__sync_fetch_and_add_8="optimized-c"
2019-07-12T16:31:39.2744205Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_and_4.S
2019-07-12T16:31:39.2744423Z cargo:rustc-cfg=__sync_fetch_and_and_4="optimized-c"
2019-07-12T16:31:39.2744678Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_and_8.S
2019-07-12T16:31:39.2744884Z cargo:rustc-cfg=__sync_fetch_and_and_8="optimized-c"
2019-07-12T16:31:39.2745137Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_max_4.S
2019-07-12T16:31:39.2745345Z cargo:rustc-cfg=__sync_fetch_and_max_4="optimized-c"
2019-07-12T16:31:39.2745598Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_max_8.S
2019-07-12T16:31:39.2745806Z cargo:rustc-cfg=__sync_fetch_and_max_8="optimized-c"
2019-07-12T16:31:39.2746058Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_min_4.S
2019-07-12T16:31:39.2746570Z cargo:rustc-cfg=__sync_fetch_and_min_4="optimized-c"
2019-07-12T16:31:39.2747295Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_min_8.S
2019-07-12T16:31:39.2747591Z cargo:rustc-cfg=__sync_fetch_and_min_8="optimized-c"
2019-07-12T16:31:39.2747902Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_4.S
2019-07-12T16:31:39.2748187Z cargo:rustc-cfg=__sync_fetch_and_nand_4="optimized-c"
2019-07-12T16:31:39.2748498Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_8.S
2019-07-12T16:31:39.2748785Z cargo:rustc-cfg=__sync_fetch_and_nand_8="optimized-c"
2019-07-12T16:31:39.2749090Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_or_4.S
2019-07-12T16:31:39.2749377Z cargo:rustc-cfg=__sync_fetch_and_or_4="optimized-c"
2019-07-12T16:31:39.2749683Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_or_8.S
2019-07-12T16:31:39.2749974Z cargo:rustc-cfg=__sync_fetch_and_or_8="optimized-c"
2019-07-12T16:31:39.2750689Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_4.S
2019-07-12T16:31:39.2750920Z cargo:rustc-cfg=__sync_fetch_and_sub_4="optimized-c"
2019-07-12T16:31:39.2751337Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_8.S
2019-07-12T16:31:39.2751723Z cargo:rustc-cfg=__sync_fetch_and_sub_8="optimized-c"
2019-07-12T16:31:39.2752151Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_4.S
2019-07-12T16:31:39.2752387Z cargo:rustc-cfg=__sync_fetch_and_umax_4="optimized-c"
2019-07-12T16:31:39.2752668Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_8.S
2019-07-12T16:31:39.2752902Z cargo:rustc-cfg=__sync_fetch_and_umax_8="optimized-c"
2019-07-12T16:31:39.2753200Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_4.S
2019-07-12T16:31:39.2753447Z cargo:rustc-cfg=__sync_fetch_and_umin_4="optimized-c"
2019-07-12T16:31:39.2753734Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_8.S
2019-07-12T16:31:39.2753967Z cargo:rustc-cfg=__sync_fetch_and_umin_8="optimized-c"
2019-07-12T16:31:39.2754252Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_4.S
2019-07-12T16:31:39.2754500Z cargo:rustc-cfg=__sync_fetch_and_xor_4="optimized-c"
2019-07-12T16:31:39.2754768Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_8.S
2019-07-12T16:31:39.2755017Z cargo:rustc-cfg=__sync_fetch_and_xor_8="optimized-c"
2019-07-12T16:31:39.2755449Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_synchronize.S
2019-07-12T16:31:39.2755683Z cargo:rustc-cfg=__sync_synchronize="optimized-c"
2019-07-12T16:31:39.2755934Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c
2019-07-12T16:31:39.2756320Z cargo:rustc-cfg=__truncdfhf2="optimized-c"
2019-07-12T16:31:39.2756719Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c
2019-07-12T16:31:39.2757317Z cargo:rustc-cfg=__truncdfsf2="optimized-c"
2019-07-12T16:31:39.2757610Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
2019-07-12T16:31:39.2757877Z cargo:rustc-cfg=__truncsfhf2="optimized-c"
2019-07-12T16:31:39.2758166Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
2019-07-12T16:31:39.2758432Z cargo:rustc-cfg=__ucmpdi2="optimized-c"
2019-07-12T16:31:39.2758719Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
2019-07-12T16:31:39.2759013Z cargo:rustc-cfg=__ucmpti2="optimized-c"
2019-07-12T16:31:39.2759334Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/udivmodsi4.S
2019-07-12T16:31:39.2759720Z cargo:rustc-cfg=__udivmodsi4="optimized-c"
2019-07-12T16:31:39.2760098Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/udivsi3.S
2019-07-12T16:31:39.2760541Z cargo:rustc-cfg=__udivsi3="optimized-c"
2019-07-12T16:31:39.2760805Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/umodsi3.S
2019-07-12T16:31:39.2761010Z cargo:rustc-cfg=__umodsi3="optimized-c"
2019-07-12T16:31:39.2761425Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c
2019-07-12T16:31:39.2761634Z cargo:rustc-cfg=apple_versioning="optimized-c"
2019-07-12T16:31:39.2761844Z TARGET = Some("armv7-unknown-linux-gnueabihf")
2019-07-12T16:31:39.2761903Z OPT_LEVEL = Some("2")
2019-07-12T16:31:39.2762107Z HOST = Some("x86_64-unknown-linux-gnu")
2019-07-12T16:31:39.2762335Z CC_armv7-unknown-linux-gnueabihf = Some("sccache armv7-unknown-linux-gnueabihf-gcc")
2019-07-12T16:31:39.2762613Z CFLAGS_armv7-unknown-linux-gnueabihf = Some("-ffunction-sections -fdata-sections -fPIC -march=armv7-a")
2019-07-12T16:31:39.2762758Z CRATE_CC_NO_DEFAULTS = None
2019-07-12T16:31:39.2762827Z DEBUG = Some("false")
2019-07-12T16:31:39.2762887Z CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7")
2019-07-12T16:31:39.2763163Z CC_armv7-unknown-linux-gnueabihf = Some("sccache armv7-unknown-linux-gnueabihf-gcc")
2019-07-12T16:31:39.2763419Z CFLAGS_armv7-unknown-linux-gnueabihf = Some("-ffunction-sections -fdata-sections -fPIC -march=armv7-a")
2019-07-12T16:31:39.2763504Z CRATE_CC_NO_DEFAULTS = None
2019-07-12T16:31:39.2763564Z CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7")
2019-07-12T16:31:39.2764235Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
2019-07-12T16:31:39.2764433Z exit code: 0
2019-07-12T16:31:39.2765085Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/absvsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c"
2019-07-12T16:31:39.2765264Z exit code: 0
2019-07-12T16:31:39.2765915Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/absvti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c"
2019-07-12T16:31:39.2766112Z exit code: 0
2019-07-12T16:31:39.2767297Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/addvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c"
2019-07-12T16:31:39.2767537Z exit code: 0
2019-07-12T16:31:39.2768437Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/addvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c"
2019-07-12T16:31:39.2768699Z exit code: 0
2019-07-12T16:31:39.2769544Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/addvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c"
2019-07-12T16:31:39.2769781Z exit code: 0
2019-07-12T16:31:39.2770755Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_cdcmp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S"
2019-07-12T16:31:39.2770950Z exit code: 0
2019-07-12T16:31:39.2771805Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_cdcmpeq_check_nan.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c"
2019-07-12T16:31:39.2772015Z exit code: 0
2019-07-12T16:31:39.2774395Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_cfcmp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S"
2019-07-12T16:31:39.2774627Z exit code: 0
2019-07-12T16:31:39.2775617Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_cfcmpeq_check_nan.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c"
2019-07-12T16:31:39.2775834Z exit code: 0
2019-07-12T16:31:39.2777105Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_div0.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_div0.c"
2019-07-12T16:31:39.2777440Z exit code: 0
2019-07-12T16:31:39.2779011Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_drsub.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_drsub.c"
2019-07-12T16:31:39.2779272Z exit code: 0
2019-07-12T16:31:39.2780106Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/aeabi_frsub.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_frsub.c"
2019-07-12T16:31:39.2780453Z exit code: 0
2019-07-12T16:31:39.2781424Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/bswapdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapdi2.S"
2019-07-12T16:31:39.2781627Z exit code: 0
2019-07-12T16:31:39.2782808Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/bswapsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapsi2.S"
2019-07-12T16:31:39.2783010Z exit code: 0
2019-07-12T16:31:39.2783656Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/clzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzdi2.S"
2019-07-12T16:31:39.2783852Z exit code: 0
2019-07-12T16:31:39.2784524Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/clzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzsi2.S"
2019-07-12T16:31:39.2784727Z exit code: 0
2019-07-12T16:31:39.2785433Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/clzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c"
2019-07-12T16:31:39.2785633Z exit code: 0
2019-07-12T16:31:39.2786321Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/cmpdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c"
2019-07-12T16:31:39.2786685Z exit code: 0
2019-07-12T16:31:39.2788158Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/cmpti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c"
2019-07-12T16:31:39.2788489Z exit code: 0
2019-07-12T16:31:39.2789321Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/ctzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c"
2019-07-12T16:31:39.2789551Z exit code: 0
2019-07-12T16:31:39.2790350Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/ctzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c"
2019-07-12T16:31:39.2790754Z exit code: 0
2019-07-12T16:31:39.2791760Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/ctzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c"
2019-07-12T16:31:39.2791951Z exit code: 0
2019-07-12T16:31:39.2792638Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/divdc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c"
2019-07-12T16:31:39.2792826Z exit code: 0
2019-07-12T16:31:39.2793584Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/divmodsi4.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divmodsi4.S"
2019-07-12T16:31:39.2793790Z exit code: 0
2019-07-12T16:31:39.2794496Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/divsc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c"
2019-07-12T16:31:39.2794679Z exit code: 0
2019-07-12T16:31:39.2795669Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/divsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divsi3.S"
2019-07-12T16:31:39.2795937Z exit code: 0
2019-07-12T16:31:39.2797330Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/divxc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c"
2019-07-12T16:31:39.2797747Z exit code: 0
2019-07-12T16:31:39.2799943Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/extendhfsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c"
2019-07-12T16:31:39.2800223Z exit code: 0
2019-07-12T16:31:39.2801274Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/ffsti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c"
2019-07-12T16:31:39.2801714Z exit code: 0
2019-07-12T16:31:39.2802933Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/fixdfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S"
2019-07-12T16:31:39.2803298Z exit code: 0
2019-07-12T16:31:39.2804090Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/fixsfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixsfsivfp.S"
2019-07-12T16:31:39.2804306Z exit code: 0
2019-07-12T16:31:39.2805399Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/fixunsdfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S"
2019-07-12T16:31:39.2805770Z exit code: 0
2019-07-12T16:31:39.2809268Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/fixunssfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunssfsivfp.S"
2019-07-12T16:31:39.2809645Z exit code: 0
2019-07-12T16:31:39.2810547Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/floatsidfvfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsidfvfp.S"
2019-07-12T16:31:39.2810740Z exit code: 0
2019-07-12T16:31:39.2814814Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/floatsisfvfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsisfvfp.S"
2019-07-12T16:31:39.2815054Z exit code: 0
2019-07-12T16:31:39.2815780Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/floatunssidfvfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssidfvfp.S"
2019-07-12T16:31:39.2816143Z exit code: 0
2019-07-12T16:31:39.2817937Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/floatunssisfvfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssisfvfp.S"
2019-07-12T16:31:39.2818170Z exit code: 0
2019-07-12T16:31:39.2819156Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/int_util.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c"
2019-07-12T16:31:39.2819421Z exit code: 0
2019-07-12T16:31:39.2820266Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/modsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/modsi3.S"
2019-07-12T16:31:39.2820502Z exit code: 0
2019-07-12T16:31:39.2822474Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/muldc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c"
2019-07-12T16:31:39.2822807Z exit code: 0
2019-07-12T16:31:39.2823486Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/mulsc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c"
2019-07-12T16:31:39.2823675Z exit code: 0
2019-07-12T16:31:39.2824310Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/mulvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c"
2019-07-12T16:31:39.2824510Z exit code: 0
2019-07-12T16:31:39.2825159Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/mulvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c"
2019-07-12T16:31:39.2825349Z exit code: 0
2019-07-12T16:31:39.2825980Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/mulvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c"
2019-07-12T16:31:39.2826167Z exit code: 0
2019-07-12T16:31:39.2827284Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/mulxc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c"
2019-07-12T16:31:39.2827544Z exit code: 0
2019-07-12T16:31:39.2828391Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negdf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c"
2019-07-12T16:31:39.2828626Z exit code: 0
2019-07-12T16:31:39.2829456Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negdf2vfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negdf2vfp.S"
2019-07-12T16:31:39.2829772Z exit code: 0
2019-07-12T16:31:39.2830884Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c"
2019-07-12T16:31:39.2831258Z exit code: 0
2019-07-12T16:31:39.2831993Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c"
2019-07-12T16:31:39.2832365Z exit code: 0
2019-07-12T16:31:39.2833526Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negsf2vfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negsf2vfp.S"
2019-07-12T16:31:39.2833704Z exit code: 0
2019-07-12T16:31:39.2834348Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c"
2019-07-12T16:31:39.2834532Z exit code: 0
2019-07-12T16:31:39.2835233Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c"
2019-07-12T16:31:39.2835418Z exit code: 0
2019-07-12T16:31:39.2836082Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negvsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c"
2019-07-12T16:31:39.2836272Z exit code: 0
2019-07-12T16:31:39.2837295Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/negvti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c"
2019-07-12T16:31:39.2837618Z exit code: 0
2019-07-12T16:31:39.2838461Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/paritydi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c"
2019-07-12T16:31:39.2838700Z exit code: 0
2019-07-12T16:31:39.2839553Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/paritysi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c"
2019-07-12T16:31:39.2839800Z exit code: 0
2019-07-12T16:31:39.2840762Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/parityti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c"
2019-07-12T16:31:39.2840964Z exit code: 0
2019-07-12T16:31:39.2841638Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/popcountdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c"
2019-07-12T16:31:39.2841830Z exit code: 0
2019-07-12T16:31:39.2842549Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/popcountsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c"
2019-07-12T16:31:39.2842757Z exit code: 0
2019-07-12T16:31:39.2843452Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/popcountti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c"
2019-07-12T16:31:39.2843647Z exit code: 0
2019-07-12T16:31:39.2844299Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/powixf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c"
2019-07-12T16:31:39.2844554Z exit code: 0
2019-07-12T16:31:39.2845269Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/restore_vfp_d8_d15_regs.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S"
2019-07-12T16:31:39.2845476Z exit code: 0
2019-07-12T16:31:39.2846169Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/save_vfp_d8_d15_regs.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S"
2019-07-12T16:31:39.2846360Z exit code: 0
2019-07-12T16:31:39.2847651Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/subvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c"
2019-07-12T16:31:39.2847906Z exit code: 0
2019-07-12T16:31:39.2848718Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/subvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c"
2019-07-12T16:31:39.2848954Z exit code: 0
2019-07-12T16:31:39.2849827Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/subvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c"
2019-07-12T16:31:39.2850081Z exit code: 0
2019-07-12T16:31:39.2851237Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/switch16.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch16.S"
2019-07-12T16:31:39.2851425Z exit code: 0
2019-07-12T16:31:39.2852040Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/switch32.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch32.S"
2019-07-12T16:31:39.2852291Z exit code: 0
2019-07-12T16:31:39.2853183Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/switch8.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch8.S"
2019-07-12T16:31:39.2853373Z exit code: 0
2019-07-12T16:31:39.2853997Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/switchu8.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switchu8.S"
2019-07-12T16:31:39.2854179Z exit code: 0
2019-07-12T16:31:39.2854872Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/sync_fetch_and_add_4.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_4.S"
2019-07-12T16:31:39.2855067Z exit code: 0
2019-07-12T16:31:39.2855707Z running: "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/sync_fetch_and_add_8.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S"
2019-07-12T16:31:39.2856095Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S: Assembler messages:
2019-07-12T16:31:39.2856474Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S:21: Error: bad instruction `push{r4, r5,r6,lr}'
2019-07-12T16:31:39.2856617Z 
2019-07-12T16:31:39.2857237Z --- stderr
2019-07-12T16:31:39.2857481Z thread 'main' panicked at '
2019-07-12T16:31:39.2857528Z 
2019-07-12T16:31:39.2857528Z 
2019-07-12T16:31:39.2858534Z Internal error occurred: Command "sccache" "armv7-unknown-linux-gnueabihf-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-ffunction-sections" "-fdata-sections" "-fPIC" "-march=armv7-a" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-fomit-frame-pointer" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/armv7-unknown-linux-gnueabihf/release/build/compiler_builtins-4e168857fbf90769/out/sync_fetch_and_add_8.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S" with args "armv7-unknown-linux-gnueabihf-gcc" did not execute successfully (status code exit code: 1).
2019-07-12T16:31:39.2859092Z ', /cargo/registry/src/github.com-1ecc6299db9ec823/cc-1.0.35/src/lib.rs:2398:5
2019-07-12T16:31:39.2859296Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-12T16:31:39.2859355Z 
2019-07-12T16:31:39.2859743Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host armv7-unknown-linux-gnueabihf --target armv7-unknown-linux-gnueabihf
2019-07-12T16:31:39.2859743Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host armv7-unknown-linux-gnueabihf --target armv7-unknown-linux-gnueabihf
2019-07-12T16:31:39.2859849Z Build completed unsuccessfully in 0:55:27
2019-07-12T16:31:39.2860552Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "armv7-unknown-linux-gnueabihf" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-12T16:31:39.2860691Z expected success, got: exit code: 101
2019-07-12T16:31:45.0570776Z ##[error]Bash exited with code '1'.
2019-07-12T16:31:45.0605100Z ##[section]Starting: Upload CPU usage statistics
2019-07-12T16:31:45.0608525Z ==============================================================================
2019-07-12T16:31:45.0608616Z Task         : Bash
2019-07-12T16:31:45.0608705Z Description  : Run a Bash script on macOS, Linux, or Windows
