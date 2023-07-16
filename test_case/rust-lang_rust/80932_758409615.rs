plain
##[endgroup]
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force -B try refs/remotes/origin/try
Updating files:  15% (3966/26440)
Updating files:  16% (4231/26440)
Updating files:  17% (4495/26440)
Updating files:  18% (4760/26440)
---
Updating files:  98% (26101/26440)
Updating files:  99% (26176/26440)
Updating files: 100% (26440/26440)
Updating files: 100% (26440/26440), done.
Branch 'try' set up to track remote branch 'try' from 'origin'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'8dec2931cf0d3d13621cb85c4f74390261878cee'
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
file:.git/config submodule.src/doc/edition-guide.active=true
---
[2267/2702] Building RC object tools\llvm-lto\CMakeFiles\llvm-lto.dir\__\__\resources\windows_version_resource.rc.res
[2268/2702] Building CXX object tools\lto\CMakeFiles\LTO.dir\lto.cpp.obj
[2269/2702] Building CXX object tools\llvm-ar\CMakeFiles\llvm-ar.dir\llvm-ar.cpp.obj
[2270/2702] Building CXX object tools\llvm-config\CMakeFiles\llvm-config.dir\llvm-config.cpp.obj
[2271/2702] Building CXX object lib\Testing\Support\CMakeFiles\LLVMTestingSupport.dir\Annotations.cpp.obj
[2273/2702] Linking CXX executable bin\llvm-config.exe
[2274/2702] Linking CXX executable bin\llvm-profdata.exe
[2274/2702] Linking CXX executable bin\llvm-profdata.exe
[2275/2702] Building CXX object lib\Testing\Support\CMakeFiles\LLVMTestingSupport.dir\SupportHelpers.cpp.obj
[2277/2702] Building CXX object tools\bugpoint\CMakeFiles\bugpoint.dir\ExecutionDriver.cpp.obj
[2278/2702] Generating ../../bin/llvm-lib.exe
[2279/2702] Generating ../../bin/llvm-ranlib.exe
[2280/2702] Generating ../../bin/llvm-dlltool.exe
[2280/2702] Generating ../../bin/llvm-dlltool.exe
[2281/2702] Building CXX object lib\Testing\Support\CMakeFiles\LLVMTestingSupport.dir\Error.cpp.obj
[2283/2702] Building CXX object tools\bugpoint\CMakeFiles\bugpoint.dir\CrashDebugger.cpp.obj
[2284/2702] Building CXX object tools\bugpoint\CMakeFiles\bugpoint.dir\ExtractFunction.cpp.obj
[2285/2702] Building CXX object tools\bugpoint\CMakeFiles\bugpoint.dir\ToolRunner.cpp.obj
[2286/2702] Building RC object tools\bugpoint\CMakeFiles\bugpoint.dir\__\__\resources\windows_version_resource.rc.res
---
[2680/2702] Linking CXX executable bin\sanstats.exe
[2681/2702] Linking CXX executable bin\sancov.exe
[2682/2702] Linking CXX executable bin\verify-uselistorder.exe
[2683/2702] Linking CXX executable bin\yaml2obj.exe
[2684/2702] Building CXX object unittests\Support\DynamicLibrary\CMakeFiles\DynamicLibraryLib.dir\ExportedFuncs.cpp.obj
[2685/2702] Linking CXX static library lib\DynamicLibraryLib.lib
[2686/2702] Building CXX object unittests\Support\DynamicLibrary\CMakeFiles\PipSqueak.dir\PipSqueak.cpp.obj
[2687/2702] Linking CXX shared library unittests\Support\DynamicLibrary\PipSqueak.dll
   Creating library unittests\Support\DynamicLibrary\PipSqueak.lib and object unittests\Support\DynamicLibrary\PipSqueak.exp

[2688/2702] Building RC object utils\KillTheDoctor\CMakeFiles\KillTheDoctor.dir\__\__\resources\windows_version_resource.rc.res
[2689/2702] Copying llvm-locstats into D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/build/./bin
[2690/2702] Building CXX object unittests\Support\DynamicLibrary\CMakeFiles\SecondLib.dir\PipSqueak.cpp.obj
[2691/2702] Linking CXX shared library unittests\Support\DynamicLibrary\SecondLib.dll
   Creating library unittests\Support\DynamicLibrary\SecondLib.lib and object unittests\Support\DynamicLibrary\SecondLib.exp
[2692/2702] Linking CXX shared library bin\LLVM-C.dll
   Creating library lib\LLVM-C.lib and object lib\LLVM-C.exp

[2693/2702] Linking CXX executable bin\opt.exe
[2693/2702] Linking CXX executable bin\opt.exe
[2694/2702] Building CXX object utils\unittest\UnitTestMain\CMakeFiles\gtest_main.dir\TestMain.cpp.obj
[2695/2702] Building CXX object utils\KillTheDoctor\CMakeFiles\KillTheDoctor.dir\KillTheDoctor.cpp.obj
[2696/2702] Linking CXX executable bin\KillTheDoctor.exe
[2697/2702] Building CXX object utils\unittest\CMakeFiles\gtest.dir\googlemock\src\gmock-all.cc.obj
[2698/2702] Building CXX object utils\unittest\CMakeFiles\gtest.dir\googletest\src\gtest-all.cc.obj
[2699/2702] Linking CXX static library lib\gtest.lib
[2700/2702] Linking CXX static library lib\gtest_main.lib
[2701/2702] Linking CXX static library lib\LLVMTestingSupport.lib
-- Install configuration: "Release"
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ADT
-- Installing: D:/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/include/llvm/ADT/AllocatorList.h
---
Dist rust-src-nightly
 finished in 7.595 seconds
Could not determine the LLVM submodule commit hash. Assuming that an LLVM rebuild is not necessary.
To force LLVM to rebuild, remove the file `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\llvm-finished-building`
saw files 'aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray
D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMXRay.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWindowsManifest.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTestingSupport.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTableGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSymbolize.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDebugInfoPDB.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMOrcJIT.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMOrcError.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMJITLink.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMObjectYAML.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMIRParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMCA.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMLTO.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPasses.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMCoroutines.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMObjCARCOpts.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMExtensions.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMLineEditor.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMLibDriver.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMInterpreter.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest_main.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\gtest.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMFuzzMutate.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMCJIT.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMExecutionEngine.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRuntimeDyld.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDWARFLinker.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDlltoolDriver.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMOption.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDebugInfoGSYM.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMCoverage.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAVRDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAVRCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAVRAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAVRDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAVRInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMX86Disassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMX86AsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMX86CodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMX86Desc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMX86Info.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWebAssemblyDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWebAssemblyCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWebAssemblyDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWebAssemblyAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMWebAssemblyInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSystemZDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSystemZCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSystemZAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSystemZDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSystemZInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSparcDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSparcCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSparcAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSparcDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSparcInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVUtils.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRISCVInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPowerPCDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPowerPCCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPowerPCAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPowerPCDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMPowerPCInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMNVPTXCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMNVPTXDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMNVPTXInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMipsDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMipsCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMipsAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMipsDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMipsInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMSP430Disassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMSP430CodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMSP430AsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMSP430Desc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMSP430Info.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMHexagonDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMHexagonCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMipo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMInstrumentation.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMVectorize.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMLinker.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMIRReader.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMFrontendOpenMP.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMHexagonAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMHexagonDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMHexagonInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMAsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMDesc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMUtils.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMARMInfo.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64Disassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMCDisassembler.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64CodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMCFGuard.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMGlobalISel.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSelectionDAG.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAsmPrinter.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDebugInfoDWARF.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMCodeGen.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTarget.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMScalarOpts.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMInstCombine.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAggressiveInstCombine.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTransformUtils.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMBitWriter.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAnalysis.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMProfileData.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMObject.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMTextAPI.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMBitReader.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMCore.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMRemarks.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMBitstreamReader.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64AsmParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMCParser.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64Desc.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMMC.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDebugInfoCodeView.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDebugInfoMSF.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMBinaryFormat.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64Utils.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMAArch64Info.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMSupport.lib D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\lib\LLVMDemangle.lib
'
thread 'main' panicked at 'Error: File "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" not found!', src\bootstrap\lib.rs:1293:17
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\/library\std\src\panicking.rs:493
   1: std::panicking::begin_panic_fmt
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\/library\std\src\panicking.rs:435
   2: bootstrap::Build::install
             at .\src\bootstrap\lib.rs:1293
   3: bootstrap::dist::maybe_install_llvm
             at .\src\bootstrap\dist.rs:1844
   4: bootstrap::dist::{{impl}}::run
             at .\src\bootstrap\dist.rs:1985
   5: bootstrap::builder::Builder::ensure<bootstrap::dist::RustDev>
             at .\src\bootstrap\builder.rs:1484
   6: bootstrap::dist::{{impl}}::make_run
             at .\src\bootstrap\dist.rs:1944
   7: bootstrap::builder::StepDescription::maybe_run
             at .\src\bootstrap\builder.rs:179
   8: bootstrap::builder::StepDescription::run
             at .\src\bootstrap\builder.rs:200
   9: bootstrap::builder::Builder::run_step_descriptions
             at .\src\bootstrap\builder.rs:570
  10: bootstrap::builder::Builder::execute_cli
             at .\src\bootstrap\builder.rs:561
  11: bootstrap::Build::build
             at .\src\bootstrap\lib.rs:510
  12: bootstrap::main
             at .\src\bootstrap\bin\main.rs:30
  13: core::ops::function::FnOnce::call_once<fn(),tuple<>>
             at /rustc/05b6023675d77979637b04a350c85903fbf59257\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
