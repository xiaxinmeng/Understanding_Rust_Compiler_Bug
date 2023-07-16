plain
Updating files:  98% (34796/35506)
Updating files:  99% (35151/35506)
Updating files: 100% (35506/35506)
Updating files: 100% (35506/35506), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'1b51d334c169f3e6e927a158472500e2b25cbcf6'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/book.active=true
---
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Success
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX - Failed
-- Looking for _M_X64
-- Looking for _M_X64 - found
-- The ASM_MASM compiler identification is MSVC
-- Found assembler: C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/Hostx64/x64/ml64.exe
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
---
-- Configuring done
-- Generating done
-- Build files have been written to: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "8"
[1/3117] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_dispatch.c.obj
[2/3117] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3.c.obj
[4/3117] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\DLangDemangle.cpp.obj
[5/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ABIBreak.cpp.obj
[6/3117] Building CXX object lib\Demangle\CMakeFiles\LLVMDemangle.dir\MicrosoftDemangleNodes.cpp.obj
[7/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ARMBuildAttrs.cpp.obj
---
[44/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\CodeGenCoverage.cpp.obj
[45/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\DAGDeltaAlgorithm.cpp.obj
[46/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Debug.cpp.obj
[47/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ELFAttributes.cpp.obj
[48/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\CSKYTargetParser.cpp.obj
[50/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\DeltaAlgorithm.cpp.obj
[51/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Error.cpp.obj
[52/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\ELFAttributeParser.cpp.obj
[53/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\CSKYAttributeParser.cpp.obj
---
[130/3117] Building C object lib\Support\CMakeFiles\LLVMSupport.dir\regexec.c.obj
[131/3117] Building C object lib\Support\CMakeFiles\LLVMSupport.dir\regfree.c.obj
[132/3117] Building C object lib\Support\CMakeFiles\LLVMSupport.dir\regcomp.c.obj
[133/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\xxhash.cpp.obj
[134/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\UnicodeNameToCodepointGenerated.cpp.obj
[136/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\VersionTuple.cpp.obj
[137/3117] Building C object lib\Support\CMakeFiles\LLVMSupport.dir\regstrlcpy.c.obj
[138/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\YAMLParser.cpp.obj
[139/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Z3Solver.cpp.obj
---
[149/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Watchdog.cpp.obj
[150/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Process.cpp.obj
[151/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Host.cpp.obj
[152/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Errno.cpp.obj
[153/3117] Building ASM_MASM object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_sse41_x86-64_windows_msvc.asm.obj
Microsoft (R) Macro Assembler (x64) Version 14.29.30146.0

Copyright (C) Microsoft Corporation.  All rights reserved.



 Assembling: D:\a\rust\rust\src\llvm-project\llvm\lib\Support\BLAKE3\blake3_sse41_x86-64_windows_msvc.asm

[154/3117] Building ASM_MASM object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_sse2_x86-64_windows_msvc.asm.obj
Microsoft (R) Macro Assembler (x64) Version 14.29.30146.0

Copyright (C) Microsoft Corporation.  All rights reserved.



 Assembling: D:\a\rust\rust\src\llvm-project\llvm\lib\Support\BLAKE3\blake3_sse2_x86-64_windows_msvc.asm
[155/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Program.cpp.obj
[155/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Program.cpp.obj
[156/3117] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_neon.c.obj
[157/3117] Building C object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_portable.c.obj
[159/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Threading.cpp.obj
[160/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\DynamicLibrary.cpp.obj
[160/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\DynamicLibrary.cpp.obj
[161/3117] Building ASM_MASM object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_avx2_x86-64_windows_msvc.asm.obj
Microsoft (R) Macro Assembler (x64) Version 14.29.30146.0

Copyright (C) Microsoft Corporation.  All rights reserved.



 Assembling: D:\a\rust\rust\src\llvm-project\llvm\lib\Support\BLAKE3\blake3_avx2_x86-64_windows_msvc.asm

[162/3117] Building ASM_MASM object lib\Support\BLAKE3\CMakeFiles\LLVMSupportBlake3.dir\blake3_avx512_x86-64_windows_msvc.asm.obj
Microsoft (R) Macro Assembler (x64) Version 14.29.30146.0

Copyright (C) Microsoft Corporation.  All rights reserved.



 Assembling: D:\a\rust\rust\src\llvm-project\llvm\lib\Support\BLAKE3\blake3_avx512_x86-64_windows_msvc.asm
[163/3117] Building CXX object lib\TableGen\CMakeFiles\LLVMTableGen.dir\StringMatcher.cpp.obj
[164/3117] Building CXX object lib\Support\CMakeFiles\LLVMSupport.dir\Path.cpp.obj
[165/3117] Building CXX object lib\TableGen\CMakeFiles\LLVMTableGen.dir\Parser.cpp.obj
[166/3117] Linking CXX static library lib\LLVMSupport.lib
---
[199/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\FastISelEmitter.cpp.obj
[200/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\InfoByHwMode.cpp.obj
[201/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAPacketizerEmitter.cpp.obj
[202/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenSchedule.cpp.obj
[203/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\DXILEmitter.cpp.obj
[205/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\SDNodeProperties.cpp.obj
[206/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptRSTEmitter.cpp.obj
[207/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrInfoEmitter.cpp.obj
[208/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetFeatureInfo.cpp.obj
---
[229/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj
[230/3117] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\CodeExpander.cpp.obj
[231/3117] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagEdge.cpp.obj
[232/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj
[233/3117] Building CXX object utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86MnemonicTables.cpp.obj
[235/3117] Generating VCSRevision.h
[236/3117] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDag.cpp.obj
[237/3117] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagPredicateDependencyEdge.cpp.obj
[238/3117] Building CXX object utils\TableGen\GlobalISel\CMakeFiles\LLVMTableGenGlobalISel.dir\GIMatchDagOperands.cpp.obj
---
[284/3117] Building IntrinsicsPowerPC.h...
[285/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfo.cpp.obj
[286/3117] Building IntrinsicsR600.h...
[287/3117] Building IntrinsicsAArch64.h...
[288/3117] Building IntrinsicsSPIRV.h...
[289/3117] Building IntrinsicsDirectX.h...
[291/3117] Building OMP.inc...
[292/3117] Building OMP.h.inc...
[293/3117] Building ACC.inc...
[294/3117] Linking CXX static library lib\LLVMFileCheck.lib
---
[902/3117] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\HeatUtils.cpp.obj
[903/3117] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\IRSimilarityIdentifier.cpp.obj
[904/3117] Building CXX object lib\Analysis\CMakeFiles\LLVMAnalysis.dir\DomPrinter.cpp.obj
[905/3117] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\SimpleLoopUnswitch.cpp.obj
[906/3117] Building CXX object lib\Transforms\Scalar\CMakeFiles\LLVMScalarOpts.dir\TLSVariableHoist.cpp.obj
[908/3117] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\Annotation2Metadata.cpp.obj
[909/3117] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\AlwaysInliner.cpp.obj
[910/3117] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\ConstantMerge.cpp.obj
[911/3117] Building CXX object lib\Transforms\IPO\CMakeFiles\LLVMipo.dir\ForceFunctionAttrs.cpp.obj
---
[1079/3117] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\ThinLTOCodeGenerator.cpp.obj
[1080/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCCodeEmitter.cpp.obj
[1081/3117] Building CXX object lib\LTO\CMakeFiles\LLVMLTO.dir\LTOBackend.cpp.obj
[1082/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCCodeView.cpp.obj
[1083/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCDXContainerWriter.cpp.obj
[1085/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmStreamer.cpp.obj
[1086/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCDXContainerStreamer.cpp.obj
[1087/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoXCOFF.cpp.obj
[1088/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCInst.cpp.obj
---
[1100/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSectionCOFF.cpp.obj
[1101/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCContext.cpp.obj
[1102/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSectionWasm.cpp.obj
[1103/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSubtargetInfo.cpp.obj
[1104/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSPIRVStreamer.cpp.obj
[1106/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSectionDXContainer.cpp.obj
[1107/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCSectionELF.cpp.obj
[1108/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCRegisterInfo.cpp.obj
[1109/3117] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCNullStreamer.cpp.obj
---
[1190/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\wasm\WasmReader.cpp.obj
[1191/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\wasm\WasmObject.cpp.obj
[1192/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\MachO\MachOWriter.cpp.obj
[1193/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\MachO\MachOObject.cpp.obj
[1194/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\XCOFF\XCOFFReader.cpp.obj
[1196/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\XCOFF\XCOFFWriter.cpp.obj
[1197/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\wasm\WasmObjcopy.cpp.obj
[1198/3117] Building CXX object lib\ObjCopy\CMakeFiles\LLVMObjCopy.dir\MachO\MachOLayoutBuilder.cpp.obj
[1199/3117] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\COFFModuleDefinition.cpp.obj
---
[1236/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\DWARFYAML.cpp.obj
[1237/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\ArchiveYAML.cpp.obj
[1238/3117] Building CXX object lib\Object\CMakeFiles\LLVMObject.dir\ELFObjectFile.cpp.obj
[1239/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\CodeViewYAMLTypes.cpp.obj
[1240/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\DXContainerYAML.cpp.obj
[1242/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\ELFYAML.cpp.obj
[1243/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\MinidumpEmitter.cpp.obj
[1244/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\WasmYAML.cpp.obj
[1245/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\CodeViewYAMLDebugSections.cpp.obj
[1245/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\CodeViewYAMLDebugSections.cpp.obj
[1246/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\CodeViewYAMLSymbols.cpp.obj
[1247/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\MinidumpYAML.cpp.obj
[1248/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\YAML.cpp.obj
[1249/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\OffloadEmitter.cpp.obj
[1250/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\MachOYAML.cpp.obj
[1251/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\ELFEmitter.cpp.obj
[1252/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\MachOEmitter.cpp.obj
[1253/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\OffloadYAML.cpp.obj
[1255/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\ObjectYAML.cpp.obj
[1256/3117] Building CXX object lib\Option\CMakeFiles\LLVMOption.dir\Arg.cpp.obj
[1257/3117] Building CXX object lib\ObjectYAML\CMakeFiles\LLVMObjectYAML.dir\WasmEmitter.cpp.obj
[1258/3117] Building CXX object lib\DebugInfo\Symbolize\CMakeFiles\LLVMSymbolize.dir\MarkupFilter.cpp.obj
---
[1528/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\ELFNixPlatform.cpp.obj
[1529/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\LazyReexports.cpp.obj
[1530/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\ObjectLinkingLayer.cpp.obj
[1531/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\TaskDispatch.cpp.obj
[1532/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\MemoryMapper.cpp.obj
D:\a\rust\rust\src\llvm-project\llvm\lib\ExecutionEngine\Orc\MemoryMapper.cpp(354,9): warning: ignoring return value of function declared with 'warn_unused_result' attribute [-Wunused-result]
        joinErrors(std::move(Err),
1 warning generated.
[1533/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\ExecutorProcessControl.cpp.obj
[1534/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\ThreadSafeModule.cpp.obj
[1535/3117] Building CXX object lib\ExecutionEngine\Orc\CMakeFiles\LLVMOrcJIT.dir\LLJIT.cpp.obj
---
[1557/3117] Building CXX object lib\ExecutionEngine\RuntimeDyld\CMakeFiles\LLVMRuntimeDyld.dir\RuntimeDyldCOFF.cpp.obj
[1558/3117] Building CXX object lib\ExecutionEngine\RuntimeDyld\CMakeFiles\LLVMRuntimeDyld.dir\JITSymbol.cpp.obj
[1559/3117] Building CXX object lib\ExecutionEngine\RuntimeDyld\CMakeFiles\LLVMRuntimeDyld.dir\RuntimeDyldChecker.cpp.obj
[1560/3117] Building CXX object lib\Target\CMakeFiles\LLVMTarget.dir\TargetMachineC.cpp.obj
[1561/3117] Building CXX object lib\ExecutionEngine\Orc\TargetProcess\CMakeFiles\LLVMOrcTargetProcess.dir\ExecutorSharedMemoryMapperService.cpp.obj
D:\a\rust\rust\src\llvm-project\llvm\lib\ExecutionEngine\Orc\TargetProcess\ExecutorSharedMemoryMapperService.cpp(215,12): warning: variable 'Size' set but not used [-Wunused-but-set-variable]
           ^
1 warning generated.
[1562/3117] Building CXX object lib\Target\CMakeFiles\LLVMTarget.dir\Target.cpp.obj
[1563/3117] Linking CXX static library lib\LLVMOrcTargetProcess.lib
---
[1683/3117] Building SparcGenDisassemblerTables.inc...
[1684/3117] Building SparcGenInstrInfo.inc...
[1685/3117] Building X86GenRegisterBank.inc...
[1686/3117] Building X86GenRegisterInfo.inc...
[1687/3117] Building X86GenMnemonicTables.inc...
[1689/3117] Building CXX object lib\Target\AArch64\CMakeFiles\LLVMAArch64CodeGen.dir\AArch64BranchTargets.cpp.obj
[1690/3117] Building X86GenExegesis.inc...
[1691/3117] Building X86GenDisassemblerTables.inc...
[1692/3117] Building X86GenDAGISel.inc...
---
[1822/3117] Building CXX object lib\Target\AArch64\AsmParser\CMakeFiles\LLVMAArch64AsmParser.dir\AArch64AsmParser.cpp.obj
[1823/3117] Linking CXX static library lib\LLVMAArch64AsmParser.lib
[1824/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBranchTargets.cpp.obj
[1825/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\A15SDOptimizer.cpp.obj
[1826/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMFixCortexA57AES1742098Pass.cpp.obj
[1828/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBasicBlockInfo.cpp.obj
[1829/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMHazardRecognizer.cpp.obj
[1830/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMBaseInstrInfo.cpp.obj
[1831/3117] Building CXX object lib\Target\ARM\CMakeFiles\LLVMARMCodeGen.dir\ARMFastISel.cpp.obj
---
[2384/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86MCInstLower.cpp.obj
[2385/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86ShuffleDecodeConstantPool.cpp.obj
[2386/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86InsertPrefetch.cpp.obj
[2387/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86MachineFunctionInfo.cpp.obj
[2388/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86ReturnThunks.cpp.obj
[2390/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86VZeroUpper.cpp.obj
[2391/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86SelectionDAGInfo.cpp.obj
[2392/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86SpeculativeExecutionSideEffectSuppression.cpp.obj
[2393/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86TargetMachine.cpp.obj
---
[2418/3117] Linking CXX static library lib\LLVMX86Info.lib
[2419/3117] Linking CXX static library lib\LLVMX86Disassembler.lib
[2420/3117] Building CXX object lib\Target\X86\MCTargetDesc\CMakeFiles\LLVMX86Desc.dir\X86MCCodeEmitter.cpp.obj
[2421/3117] Building CXX object lib\Target\AVR\CMakeFiles\LLVMAVRCodeGen.dir\AVRMCInstLower.cpp.obj
[2422/3117] Building CXX object lib\Target\X86\MCTargetDesc\CMakeFiles\LLVMX86Desc.dir\X86MnemonicTables.cpp.obj
[2424/3117] Building CXX object lib\Target\AVR\CMakeFiles\LLVMAVRCodeGen.dir\AVRFrameLowering.cpp.obj
[2425/3117] Building CXX object lib\Target\AVR\CMakeFiles\LLVMAVRCodeGen.dir\AVRAsmPrinter.cpp.obj
[2426/3117] Building CXX object lib\Target\AVR\CMakeFiles\LLVMAVRCodeGen.dir\AVRISelLowering.cpp.obj
[2427/3117] Building CXX object lib\Target\X86\CMakeFiles\LLVMX86CodeGen.dir\X86ISelLowering.cpp.obj
---
[2596/3117] Linking C executable bin\count.exe
[2597/3117] Building CXX object lib\Passes\CMakeFiles\LLVMPasses.dir\PassBuilderPipelines.cpp.obj
[2598/3117] Building RC object utils\not\CMakeFiles\not.dir\__\__\resources\windows_version_resource.rc.res
[2599/3117] Building CXX object utils\PerfectShuffle\CMakeFiles\llvm-PerfectShuffle.dir\PerfectShuffle.cpp.obj
[2600/3117] Building RC object utils\UnicodeData\CMakeFiles\UnicodeNameMappingGenerator.dir\__\__\resources\windows_version_resource.rc.res
[2601/3117] Building CXX object lib\WindowsDriver\CMakeFiles\LLVMWindowsDriver.dir\MSVCPaths.cpp.obj
[2603/3117] Linking CXX static library lib\LLVMWindowsDriver.lib
[2604/3117] Building RC object utils\yaml-bench\CMakeFiles\yaml-bench.dir\__\__\resources\windows_version_resource.rc.res
[2605/3117] Creating export file for LTO
[2606/3117] Building RC object tools\llvm-ar\CMakeFiles\llvm-ar.dir\__\__\resources\windows_version_resource.rc.res
---
[2615/3117] Linking CXX executable bin\yaml-bench.exe
[2616/3117] Building RC object tools\llvm-config\CMakeFiles\llvm-config.dir\__\__\resources\windows_version_resource.rc.res
[2617/3117] Building RC object tools\llvm-lto\CMakeFiles\llvm-lto.dir\__\__\resources\windows_version_resource.rc.res
[2618/3117] Building CXX object utils\FileCheck\CMakeFiles\FileCheck.dir\FileCheck.cpp.obj
[2619/3117] Building CXX object utils\UnicodeData\CMakeFiles\UnicodeNameMappingGenerator.dir\UnicodeNameMappingGenerator.cpp.obj
[2620/3117] Linking CXX executable bin\FileCheck.exe
[2621/3117] Linking CXX executable bin\UnicodeNameMappingGenerator.exe
[2623/3117] Building CXX object tools\llvm-config\CMakeFiles\llvm-config.dir\llvm-config.cpp.obj
[2624/3117] Linking CXX executable bin\llvm-config.exe
[2625/3117] Building CXX object tools\bugpoint\CMakeFiles\bugpoint.dir\BugDriver.cpp.obj
[2626/3117] Building CXX object tools\llvm-ar\CMakeFiles\llvm-ar.dir\llvm-ar.cpp.obj
---
[2763/3117] Linking CXX executable bin\llvm-dis.exe
[2764/3117] Building CXX object tools\llvm-dwp\CMakeFiles\llvm-dwp.dir\llvm-dwp.cpp.obj
[2765/3117] Building CXX object tools\llvm-dwarfdump\CMakeFiles\llvm-dwarfdump.dir\llvm-dwarfdump.cpp.obj
[2766/3117] Linking CXX executable bin\llvm-dwarfdump.exe
[2767/3117] Building CXX object tools\llvm-dwarfutil\CMakeFiles\llvm-dwarfutil.dir\DebugInfoLinker.cpp.obj
[2769/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\Error.cpp.obj
[2770/3117] Linking CXX executable bin\llvm-dwp.exe
[2771/3117] Building CXX object tools\llvm-exegesis\CMakeFiles\llvm-exegesis.dir\llvm-exegesis.cpp.obj
[2772/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\BenchmarkRunner.cpp.obj
---
[2780/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\Target.cpp.obj
[2781/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\Clustering.cpp.obj
[2782/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\LlvmState.cpp.obj
[2783/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\RegisterAliasing.cpp.obj
[2784/3117] Building CXX object tools\llvm-dwarfutil\CMakeFiles\llvm-dwarfutil.dir\llvm-dwarfutil.cpp.obj
[2786/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\ParallelSnippetGenerator.cpp.obj
[2787/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\SnippetGenerator.cpp.obj
[2788/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\SchedClassResolution.cpp.obj
[2789/3117] Building CXX object tools\llvm-exegesis\lib\CMakeFiles\LLVMExegesis.dir\SnippetRepetitor.cpp.obj
---
[2895/3117] Building CXX object tools\llvm-objdump\CMakeFiles\llvm-objdump.dir\WasmDump.cpp.obj
[2896/3117] Linking CXX executable bin\llvm-nm.exe
[2897/3117] Building RC object tools\llvm-objdump\CMakeFiles\llvm-objdump.dir\__\__\resources\windows_version_resource.rc.res
[2898/3117] Building CXX object tools\llvm-objcopy\CMakeFiles\llvm-objcopy.dir\llvm-objcopy.cpp.obj
[2899/3117] Building CXX object tools\llvm-objdump\CMakeFiles\llvm-objdump.dir\OffloadDump.cpp.obj
[2901/3117] Building CXX object tools\llvm-objdump\CMakeFiles\llvm-objdump.dir\ELFDump.cpp.obj
[2902/3117] Linking CXX executable bin\llvm-opt-fuzzer.exe
[2903/3117] Building CXX object tools\llvm-objcopy\CMakeFiles\llvm-objcopy.dir\ObjcopyOptions.cpp.obj
[2904/3117] Building RC object tools\llvm-opt-report\CMakeFiles\llvm-opt-report.dir\__\__\resources\windows_version_resource.rc.res
---
[3002/3117] Linking CXX executable bin\llvm-rust-demangle-fuzzer.exe
[3003/3117] Building RC object tools\llvm-sim\CMakeFiles\llvm-sim.dir\__\__\resources\windows_version_resource.rc.res
[3004/3117] Building Opts.inc...
[3005/3117] Building CXX object tools\llvm-reduce\CMakeFiles\llvm-reduce.dir\deltas\ReduceOperandsSkip.cpp.obj
[3006/3117] Building CXX object tools\llvm-remark-size-diff\CMakeFiles\llvm-remark-size-diff.dir\RemarkSizeDiff.cpp.obj
[3008/3117] Linking CXX executable bin\llvm-remark-size-diff.exe
[3009/3117] Building RC object tools\llvm-size\CMakeFiles\llvm-size.dir\__\__\resources\windows_version_resource.rc.res
[3010/3117] Building RC object tools\llvm-special-case-list-fuzzer\CMakeFiles\llvm-special-case-list-fuzzer.dir\__\__\resources\windows_version_resource.rc.res
[3011/3117] Building CXX object tools\llvm-special-case-list-fuzzer\CMakeFiles\llvm-special-case-list-fuzzer.dir\DummySpecialCaseListFuzzer.cpp.obj
---
[3071/3117] Building CXX object tools\llvm-xray\CMakeFiles\llvm-xray.dir\xray-account.cpp.obj
[3072/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\archive2yaml.cpp.obj
[3073/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\obj2yaml.cpp.obj
[3074/3117] Building CXX object tools\llvm-xray\CMakeFiles\llvm-xray.dir\xray-graph-diff.cpp.obj
[3075/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\offload2yaml.cpp.obj
[3076/3117] Linking CXX executable bin\llvm-xray.exe
[3077/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\dxcontainer2yaml.cpp.obj
[3079/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\minidump2yaml.cpp.obj
[3080/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\dwarf2yaml.cpp.obj
[3081/3117] Building CXX object tools\obj2yaml\CMakeFiles\obj2yaml.dir\xcoff2yaml.cpp.obj
[3082/3117] Building CXX object tools\opt\CMakeFiles\opt.dir\AnalysisWrappers.cpp.obj
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/CSKY.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/Hexagon.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/i386.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/Lanai.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/LoongArch.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/Mips.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/MSP430.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC64.def
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/BasicBlockSectionUtils.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/BasicTTIImpl.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CalcSpillWeights.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CallingConvLower.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CFIFixup.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CodeGenPassBuilder.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CommandFlags.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CostTable.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/CodeGen/CSEConfigBase.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/Interpreter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITEventListener.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/aarch64.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/COFF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/COFF_x86_64.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/DWARFRecordSectionSplitter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/ELF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/ELF_aarch64.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/ELF_riscv.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ExecutionEngine/JITLink/ELF_x86_64.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsAArch64.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsARM.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsBPF.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsDirectX.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsHexagonDep.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsMips.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsNVVM.td
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsPowerPC.td
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCAssembler.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCCodeEmitter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCCodeView.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCContext.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDecoderOps.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler/MCDisassembler.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler/MCExternalSymbolizer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler/MCRelocationInfo.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler/MCRelocationInfo.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDisassembler/MCSymbolizer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDwarf.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDXContainerStreamer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCDXContainerWriter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCELFStreamer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCExpr.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCFixup.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCFixupKindInfo.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSectionMachO.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSectionSPIRV.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSectionWasm.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSectionXCOFF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSPIRVObjectWriter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSPIRVStreamer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSubtargetInfo.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSymbol.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSymbolCOFF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/MC/MCSymbolELF.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/ELF/ELFObjcopy.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/MachO
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/MachO/MachOConfig.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/MachO/MachOObjcopy.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/MultiFormatConfig.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/ObjCopy.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/wasm
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/wasm/WasmConfig.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/wasm/WasmObjcopy.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/XCOFF
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/XCOFF/XCOFFConfig.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjCopy/XCOFF/XCOFFObjcopy.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/Archive.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/ArchiveWriter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/Binary.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Object/COFF.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypes.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/COFFYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/DWARFEmitter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/DWARFYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/DXContainerYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/MachOYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/MinidumpYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/ObjectYAML.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ObjectYAML/OffloadYAML.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/InstrProfReader.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/InstrProfWriter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/MemProf.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/MemProfData.inc
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/MIBEntryDef.inc
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/RawMemProfReader.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/SampleProf.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/SampleProfReader.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ProfileData/SampleProfWriter.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Compression.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/ConvertUTF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CrashRecoveryContext.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CRC.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CSKYAttributeParser.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CSKYAttributes.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CSKYTargetParser.def
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/CSKYTargetParser.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DataTypes.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Debug.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DebugCounter.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Discriminator.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Discriminator.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DivisionByConstantInfo.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DJB.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DOTGraphTraits.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Duration.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/DXILOperationCommon.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/ELFAttributeParser.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/ELFAttributes.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/Endian.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Support/EndianStream.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar/SROA.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar/StraightLineStrengthReduce.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar/StructurizeCFG.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar/TailRecursionElimination.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar/TLSVariableHoist.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Scalar.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/AddDiscriminators.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Utils/AMDGPUEmitPrintf.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Vectorize/LoopVectorize.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Vectorize/SLPVectorizer.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Vectorize/VectorCombine.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/Transforms/Vectorize.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsDriver
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsDriver/MSVCPaths.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsDriver/MSVCSetupApi.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsManifest/WindowsManifestMerger.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsResource
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsResource/ResourceProcessor.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/WindowsResource/ResourceScriptToken.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsAArch64.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsAMDGPU.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsARM.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsBPF.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsDirectX.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsMips.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsNVPTX.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsPowerPC.h
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/IR/IntrinsicsR600.h
---
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/FileCheck.exe
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/llvm-PerfectShuffle.exe
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/count.exe
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/not.exe
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/UnicodeNameMappingGenerator.exe
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/lib/LTO.lib
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/LTO.dll
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/llvm-ar.exe
CMake Warning (dev) at C:/Program Files/CMake/share/cmake-3.23/Modules/GNUInstallDirs.cmake:241 (message):
---
error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/coverage-llvmir'
# Compile the test program with non-experimental coverage instrumentation, and generate LLVM IR
PATH="/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/coverage-llvmir/coverage-llvmir:D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin:/c/Program Files (x86)/Windows Kits/10/bin/x64:/c/Program Files (x86)/Windows Kits/10/bin/10.0.22000.0/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/c/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0-bootstrap-tools/x86_64-pc-windows-msvc/release/deps:/d/a/rust/rust/build/x86_64-pc-windows-msvc/stage0/bin:/d/a/rust/rust/ninja:/d/a/rust/rust/msys2/mingw64/bin:/c/hostedtoolcache/windows/Python/3.10.5/x64/Scripts:/c/hostedtoolcache/windows/Python/3.10.5/x64:/usr/bin:/d/a/rust/rust/sccache:/c/Program Files/MongoDB/Server/5.0/bin:/c/aliyun-cli:/c/vcpkg:/c/cf-cli:/c/Program Files (x86)/NSIS:/c/tools/zstd:/c/Program Files/Mercurial:/c/hostedtoolcache/windows/stack/2.7.5/x64:/c/cabal/bin:/c/ghcup/bin:/c/tools/ghc-9.2.3/bin:/c/Program Files/dotnet:/c/mysql/bin:/c/Program Files/R/R-4.2.1/bin/x64:/c/SeleniumWebDrivers/GeckoDriver:/c/Program Files (x86)/sbt/bin:/c/Program Files (x86)/GitHub CLI:/c/Program Files/Git/bin:/c/Program Files (x86)/pipx_bin:/c/npm/prefix:/c/hostedtoolcache/windows/go/1.17.12/x64/bin:/c/hostedtoolcache/windows/Python/3.7.9/x64/Scripts:/c/hostedtoolcache/windows/Python/3.7.9/x64:/c/hostedtoolcache/windows/Ruby/2.5.9/x64/bin:/c/tools/kotlinc/bin:/c/hostedtoolcache/windows/Java_Temurin-Hotspot_jdk/8.0.332-9/x64/bin:/c/Program Files (x86)/Microsoft SDKs/Azure/CLI2/wbin:/c/ProgramData/kind:/c/Program Files/Eclipse Foundation/jdk-8.0.302.8-hotspot/bin:/c/Windows/system32:/c/Windows:/c/Windows/System32/Wbem:/c/Windows/System32/WindowsPowerShell/v1.0:/c/Windows/System32/OpenSSH:/c/ProgramData/Chocolatey/bin:/c/Program Files/Docker:/c/Program Files/PowerShell/7:/c/Program Files/Microsoft/Web Platform Installer:/c/Program Files/dotnet:/c/Program Files/Microsoft SQL Server/130/Tools/Binn:/c/Program Files/Microsoft SQL Server/Client SDK/ODBC/170/Tools/Binn:/c/Program Files (x86)/Windows Kits/10/Windows Performance Toolkit:/c/Program Files (x86)/Microsoft SQL Server/110/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/120/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/130/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/140/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/150/DTS/Binn:/c/Program Files (x86)/Microsoft SQL Server/160/DTS/Binn:/c/Program Files/OpenSSL/bin:/c/Strawberry/c/bin:/c/Strawberry/perl/site/bin:/c/Strawberry/perl/bin:/c/ProgramData/chocolatey/lib/pulumi/tools/Pulumi/bin:/c/Program Files/TortoiseSVN/bin:/c/Program Files/CMake/bin:/c/ProgramData/chocolatey/lib/maven/apache-maven-3.8.6/bin:/c/Program Files/Microsoft Service Fabric/bin/Fabric/Fabric.Code:/c/Program Files/Microsoft SDKs/Service Fabric/Tools/ServiceFabricLocalClusterManager:/c/Program Files/nodejs:/c/Program Files/Git/cmd:/c/Program Files/Git/mingw64/bin:/c/Program Files/Git/usr/bin:/c/Program Files/GitHub CLI:/c/tools/php:/c/Program Files (x86)/sbt/bin:/c/SeleniumWebDrivers/ChromeDriver:/c/SeleniumWebDrivers/EdgeDriver:/c/Program Files/Amazon/AWSCLIV2:/c/Program Files/Amazon/SessionManagerPlugin/bin:/c/Program Files/Amazon/AWSSAMCLI/bin:/c/Program Files (x86)/Google/Cloud SDK/google-cloud-sdk/bin:/c/Program Files (x86)/Microsoft BizTalk Server:/c/Program Files/LLVM/bin:/c/Users/runneradmin/.dotnet/tools:/c/Users/runneradmin/.cargo/bin:/c/Users/runneradmin/AppData/Local/Microsoft/WindowsApps" 'D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe' --out-dir /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/coverage-llvmir/coverage-llvmir -L /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/coverage-llvmir/coverage-llvmir  ../coverage-llvmir/testprog.rs \
  -Cinstrument-coverage \
  --emit=llvm-ir
cat "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make/coverage-llvmir/coverage-llvmir"/testprog.ll | \
  "/d/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build/bin/FileCheck.exe" ../coverage-llvmir/filecheck.testprog.txt -check-prefixes=CHECK,WINDOWS -DDEFINE_INTERNAL='define internal' -DCOMDAT_IF_SUPPORTED=', comdat' -DINSTR_PROF_DATA='.lprfd$M' -DINSTR_PROF_NAME='.lprfn$M' -DINSTR_PROF_CNTS='.lprfc$M' -DINSTR_PROF_VALS='.lprfv$M' -DINSTR_PROF_VNODES='.lprfnd$M' -DINSTR_PROF_COVMAP='.lcovmap$M' -DINSTR_PROF_COVFUN='.lcovfun$M' -DINSTR_PROF_ORDERFILE='.lorderfile$M'
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/coverage-llvmir'
--- stderr -------------------------------
--- stderr -------------------------------
../coverage-llvmir/filecheck.testprog.txt:12:10: error: WINDOWS: expected string not found in input
WINDOWS: @__llvm_profile_runtime = external global i32
<stdin>:51:283: note: scanning from here
<stdin>:51:283: note: scanning from here
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [83 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 83, i32 0, i32 5 }, [83 x i8] c"\02P\000D:\\a\\rust\\rust\\src\\test\\run-make\\coverage-llvmir\1E../coverage-llvmir/testprog.rs" }, section ".lcovmap$M", align 8
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-pc-windows-msvc target=x86_64-pc-windows-msvc
<stdin>:52:1: note: possible intended match here
<stdin>:52:1: note: possible intended match here
@__llvm_profile_runtime = external hidden global i32

Input file: <stdin>
Input file: <stdin>
Check file: ../coverage-llvmir/filecheck.testprog.txt

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           46: @__covrec_FBB11442302CB608u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -310444624803219960, i32 9, i64 -6292917086266085143, i64 8925074348750210240, [9 x i8] c"\01\01\00\01\01\22:\00V" }>, section ".lcovfun$M", comdat, align 8 
           47: @__covrec_9938275C14C5EF8u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [45 x i8] }> <{ i64 690038610183610104, i32 45, i64 -6292917086266085143, i64 8925074348750210240, [45 x i8] c"\01\01\03\05\00\05\00\05\00\07\01\1C\01\04\13\05\04\14\012\03\01V\012\07\01V\01\06\00\01\0C\01/\00\01^\01\06\0B\02\01\00\02" }>, section ".lcovfun$M", comdat, align 8 
           48: @__covrec_C14476BABCDD2F24u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -4520357581568528604, i32 9, i64 2352890877362890314, i64 8925074348750210240, [9 x i8] c"\01\01\00\01\01\0C\01\05\02" }>, section ".lcovfun$M", comdat, align 8 
           49: @__covrec_13FC6B6E12250F79u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 1440144101346709369, i32 9, i64 -6292917086266085143, i64 8925074348750210240, [9 x i8] c"\01\01\00\01\01!:\00V" }>, section ".lcovfun$M", comdat, align 8 
           50: @__covrec_BE6D4F1A19FD0015u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [28 x i8] }> <{ i64 -4725033460518027243, i32 28, i64 -429397554593745454, i64 8925074348750210240, [28 x i8] c"\01\01\02\01\05\05\02\04\01\13\01\04\13\05\05\09\00\18\02\01\06\00\07\07\01\01\00\02" }>, section ".lcovfun$M", comdat, align 8 
           51: @__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [83 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 83, i32 0, i32 5 }, [83 x i8] c"\02P\000D:\\a\\rust\\rust\\src\\test\\run-make\\coverage-llvmir\1E../coverage-llvmir/testprog.rs" }, section ".lcovmap$M", align 8 
check:12'0                                                                                                                                                                                                                                                                                               X error: no match found
           52: @__llvm_profile_runtime = external hidden global i32 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1     ?                                                     possible intended match
           53: @__profc__RNvCsaweISYGdvTC_8testprog14will_be_called = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54: @__profd__RNvCsaweISYGdvTC_8testprog14will_be_called = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 5708379101922736890, i64 -7548357770902260283, i64 sub (i64 ptrtoint (ptr @__profc__RNvCsaweISYGdvTC_8testprog14will_be_called to i64), i64 ptrtoint (ptr @__profd__RNvCsaweISYGdvTC_8testprog14will_be_called to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: @__profc__RINvCsaweISYGdvTC_8testprog5printRReEB2_ = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56: @__profd__RINvCsaweISYGdvTC_8testprog5printRReEB2_ = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 -4520357581568528604, i64 2352890877362890314, i64 sub (i64 ptrtoint (ptr @__profc__RINvCsaweISYGdvTC_8testprog5printRReEB2_ to i64), i64 ptrtoint (ptr @__profd__RINvCsaweISYGdvTC_8testprog5printRReEB2_ to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57: @__profc__RINvCsaweISYGdvTC_8testprog9wrap_withNCNvB2_4main0ReEB2_ = private global [3 x i64] zeroinitializer, section ".lprfc$M", align 8 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
>>>>>>
make[1]: *** [Makefile:60: test_llvm_ir] Error 1



failures:
failures:
    [run-make] src/test\run-make\coverage-llvmir

test result: FAILED. 36 passed; 1 failed; 25 ignored; 0 measured; 0 filtered out; finished in 65.03s

 finished in 65.184 seconds
Build completed unsuccessfully in 1:16:27
make: *** [Makefile:70: ci-subset-1] Error 1
