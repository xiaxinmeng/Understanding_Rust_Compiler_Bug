plain
Updating files:  98% (34885/35596)
Updating files:  99% (35241/35596)
Updating files: 100% (35596/35596)
Updating files: 100% (35596/35596), done.
Switched to a new branch 'try'
branch 'try' set up to track 'origin/try'.
[command]/usr/local/bin/git log -1 --format='%H'
'e65ef47fe4ef940cdcd1523b0baa5254ca001f49'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Success
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX
-- Performing Test LLVM_HAS_NOGLOBAL_CTOR_MUTEX - Failed
-- Looking for __x86_64__
-- Looking for __x86_64__ - found
-- LLVMHello ignored -- Loadable modules not supported on this platform.
-- Targeting AArch64
-- Targeting ARM
-- Targeting BPF
---
running: "cmake" "--build" "." "--target" "install" "--config" "Release" "--" "-j" "3"
Scanning dependencies of target LLVMSupportBlake3
[  0%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/Demangle.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3.c.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_dispatch.c.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterEmitter.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterEmitter.cpp.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_portable.c.o
[  1%] Building C object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_neon.c.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterInst.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterInst.cpp.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse2_x86-64_unix.S.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_sse41_x86-64_unix.S.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx2_x86-64_unix.S.o
[  1%] Building ASM object lib/Support/BLAKE3/CMakeFiles/LLVMSupportBlake3.dir/blake3_avx512_x86-64_unix.S.o
[  1%] Built target LLVMSupportBlake3
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CallingConvEmitter.cpp.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/DLangDemangle.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeEmitterGen.cpp.o
---
[  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DirectiveEmitter.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamError.cpp.o
[  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DisassemblerEmitter.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamReader.cpp.o
[  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DXILEmitter.cpp.o
[  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/ExegesisEmitter.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BinaryStreamWriter.cpp.o
[  2%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/FastISelEmitter.cpp.o
[  2%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/BlockFrequency.cpp.o
---
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86FoldTablesEmitter.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ExtensibleRTTI.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86MnemonicTables.cpp.o
[  4%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/FileUtilities.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
---
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Twine.cpp.o
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/TypeSize.cpp.o
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Unicode.cpp.o
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeCaseFold.cpp.o
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepoint.cpp.o
[  6%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/UnicodeNameToCodepointGenerated.cpp.o
[  7%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/VirtualFileSystem.cpp.o
[  7%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/WithColor.cpp.o
[  7%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/X86TargetParser.cpp.o
[  7%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/YAMLParser.cpp.o
---
[ 10%] Linking CXX executable ../../bin/FileCheck
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 10%] Building CXX object utils/not/CMakeFiles/not.dir/not.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 10%] Built target FileCheck
[ 10%] Built target FileCheck
[ 10%] Building CXX object utils/UnicodeData/CMakeFiles/UnicodeNameMappingGenerator.dir/UnicodeNameMappingGenerator.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSubsectionVisitor.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSymbolRVASubsection.cpp.o
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSymbolRVASubsection.cpp.o
[ 10%] Linking CXX executable ../../bin/UnicodeNameMappingGenerator
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 10%] Built target UnicodeNameMappingGenerator
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/DebugSymbolsSubsection.cpp.o
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/EnumTables.cpp.o
[ 10%] Linking CXX executable ../../bin/yaml-bench
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 10%] Built target yaml-bench
[ 10%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/GlobalTypeTableBuilder.cpp.o
[ 10%] Building CXX object tools/llvm-config/CMakeFiles/llvm-config.dir/llvm-config.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/LazyRandomTypeCollection.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/LazyRandomTypeCollection.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/Line.cpp.o
[ 11%] Building CXX object tools/llvm-dlang-demangle-fuzzer/CMakeFiles/llvm-dlang-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/MergingTypeTableBuilder.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-config
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Built target llvm-config
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordName.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-dlang-demangle-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
[ 11%] Building CXX object tools/llvm-itanium-demangle-fuzzer/CMakeFiles/llvm-itanium-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/RecordSerialization.cpp.o
[ 11%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-itanium-demangle-fuzzer/CMakeFiles/llvm-itanium-demangle-fuzzer.dir/llvm-itanium-demangle-fuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/llvm-microsoft-demangle-fuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-microsoft-demangle-fuzzer/CMakeFiles/llvm-microsoft-demangle-fuzzer.dir/llvm-microsoft-demangle-fuzzer.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-itanium-demangle-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SimpleTypeSerializer.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/StringsAndChecksums.cpp.o
[ 11%] Built target llvm-microsoft-demangle-fuzzer
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolDumper.cpp.o
[ 11%] Building CXX object tools/llvm-rust-demangle-fuzzer/CMakeFiles/llvm-rust-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
---
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/SymbolRecordMapping.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-rust-demangle-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 11%] Built target llvm-rust-demangle-fuzzer
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Built target llvm-special-case-list-fuzzer
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeDumpVisitor.cpp.o
[ 11%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/DummyYAMLNumericParserFuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/DummyYAMLNumericParserFuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-numeric-parser-fuzzer/CMakeFiles/llvm-yaml-numeric-parser-fuzzer.dir/yaml-numeric-parser-fuzzer.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndex.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-undname
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
[ 11%] Linking CXX executable ../../bin/llvm-yaml-numeric-parser-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordHelpers.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-parser-fuzzer/CMakeFiles/llvm-yaml-parser-fuzzer.dir/DummyYAMLParserFuzzer.cpp.o
[ 11%] Building CXX object tools/split-file/CMakeFiles/split-file.dir/split-file.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-parser-fuzzer/CMakeFiles/llvm-yaml-parser-fuzzer.dir/yaml-parser-fuzzer.cpp.o
[ 11%] Building CXX object tools/llvm-yaml-parser-fuzzer/CMakeFiles/llvm-yaml-parser-fuzzer.dir/yaml-parser-fuzzer.cpp.o
[ 11%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeRecordMapping.cpp.o
[ 11%] Linking CXX executable ../../bin/split-file
[ 11%] Linking CXX executable ../../bin/llvm-yaml-parser-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: ld: warning: building for macOS 10.7.0 is deprecated
building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Built target llvm-yaml-parser-fuzzer
[ 11%] Built target split-file
[ 11%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Architecture.cpp.o
[ 11%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/CodeExpander.cpp.o
---
[ 11%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagOperands.cpp.o
[ 11%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Symbol.cpp.o
[ 11%] Building CXX object lib/TextAPI/CMakeFiles/LLVMTextAPI.dir/Target.cpp.o
[ 11%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicate.cpp.o
[ 11%] Building CXX object lib/WindowsDriver/CMakeFiles/LLVMWindowsDriver.dir/MSVCPaths.cpp.o
[ 11%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchDagPredicateDependencyEdge.cpp.o
[ 11%] Building CXX object utils/TableGen/GlobalISel/CMakeFiles/LLVMTableGenGlobalISel.dir/GIMatchTree.cpp.o
[ 11%] Linking CXX static library ../libLLVMWindowsDriver.a
[ 11%] Built target LLVMWindowsDriver
---
[ 11%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/ELFObjectWriter.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 11%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmBackend.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 11%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfo.cpp.o
[ 11%] Building Attributes.inc...
[ 11%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoCOFF.cpp.o
[ 11%] Building IntrinsicImpl.inc...
---
[ 11%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmInfoXCOFF.cpp.o
[ 11%] Building IntrinsicsBPF.h...
[ 11%] Building Opts.inc...
[ 11%] Built target CvtResTableGen
[ 11%] Building IntrinsicsDirectX.h...
[ 12%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmMacro.cpp.o
[ 12%] Building IntrinsicsMips.h...
[ 12%] Building Opts.inc...
[ 12%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCAsmStreamer.cpp.o
---
[ 13%] Building IntrinsicsWebAssembly.h...
[ 13%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCDXContainerStreamer.cpp.o
[ 13%] Building IntrinsicsX86.h...
[ 13%] Building IntrinsicsXCore.h...
[ 13%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCDXContainerWriter.cpp.o
[ 13%] Building IntrinsicsVE.h...
[ 13%] Built target LipoOptsTableGen
[ 13%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCELFObjectTargetWriter.cpp.o
[ 13%] Built target intrinsics_gen
---
[ 15%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSectionXCOFF.cpp.o
[ 15%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkParser.cpp.o
[ 15%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCStreamer.cpp.o
[ 15%] Building CXX object lib/Remarks/CMakeFiles/LLVMRemarks.dir/YAMLRemarkSerializer.cpp.o
[ 15%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSPIRVStreamer.cpp.o
[ 15%] Linking CXX static library ../libLLVMRemarks.a
[ 15%] Built target LLVMRemarks
[ 15%] Building ARMGenAsmMatcher.inc...
[ 15%] Building CXX object lib/MC/CMakeFiles/LLVMMC.dir/MCSubtargetInfo.cpp.o
---
[ 22%] Building M68kGenAsmMatcher.inc...
[ 22%] Linking CXX executable ../../bin/llvm-cxxfilt
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 22%] Building CXX object tools/llvm-mt/CMakeFiles/llvm-mt.dir/llvm-mt.cpp.o
[ 22%] Building M68kGenDisassemblerTable.inc...
[ 22%] Linking CXX executable ../../bin/llvm-mt
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 22%] Building X86GenDisassemblerTables.inc...
[ 22%] Built target M68kCommonTableGen
[ 22%] Building X86GenEVEX2VEXTables.inc...
[ 22%] Building X86GenExegesis.inc...
---
[ 22%] Built target OtoolOptsTableGen
[ 22%] Building X86GenInstrInfo.inc...
[ 22%] Building WindresOpts.inc...
[ 22%] Built target WindresOptsTableGen
[ 22%] Building X86GenMnemonicTables.inc...
[ 22%] Linking CXX shared library ../../lib/libRemarks.dylib
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 22%] Building X86GenRegisterBank.inc...
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AbstractCallSite.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/AsmWriter.cpp.o
[ 22%] Building CXX object lib/IR/CMakeFiles/LLVMCore.dir/Assumptions.cpp.o
---
[ 29%] Building CXX object tools/llvm-jitlink/llvm-jitlink-executor/CMakeFiles/llvm-jitlink-executor.dir/llvm-jitlink-executor.cpp.o
[ 29%] Linking CXX executable ../../bin/llvm-cxxmap
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 29%] Building CXX object tools/llvm-diff/lib/CMakeFiles/LLVMDiff.dir/DifferenceEngine.cpp.o
[ 29%] Building CXX object tools/llvm-diff/lib/CMakeFiles/LLVMDiff.dir/DiffLog.cpp.o
[ 29%] Linking CXX executable ../../../bin/llvm-jitlink-executor
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 29%] Building BitcodeStripOpts.inc...
[ 29%] Built target BitcodeStripOptsTableGen
[ 30%] Building CXX object lib/Bitcode/Reader/CMakeFiles/LLVMBitReader.dir/BitcodeAnalyzer.cpp.o
[ 30%] Building CXX object lib/AsmParser/CMakeFiles/LLVMAsmParser.dir/LLLexer.cpp.o
---
[ 32%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kELFObjectWriter.cpp.o
[ 32%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCELFStreamer.cpp.o
[ 32%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kInstPrinter.cpp.o
[ 32%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCExpr.cpp.o
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86MnemonicTables.cpp.o
[ 32%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRMCTargetDesc.cpp.o
[ 32%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCCodeEmitter.cpp.o
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86ELFObjectWriter.cpp.o
[ 32%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRTargetStreamer.cpp.o
[ 32%] Building CXX object lib/Target/AVR/MCTargetDesc/CMakeFiles/LLVMAVRDesc.dir/AVRTargetStreamer.cpp.o
/Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/MCTargetDesc/M68kMCCodeEmitter.cpp:223:12: warning: unused variable 'Opcode' [-Wunused-variable]
  unsigned Opcode = MI.getOpcode();
[ 32%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCTargetDesc.cpp.o
[ 32%] Building CXX object lib/Target/M68k/MCTargetDesc/CMakeFiles/LLVMM68kDesc.dir/M68kMCTargetDesc.cpp.o
/Users/runner/work/rust/rust/src/llvm-project/llvm/lib/Target/M68k/MCTargetDesc/M68kMCCodeEmitter.cpp:40:22: warning: private field 'MCII' is not used [-Wunused-private-field]
  const MCInstrInfo &MCII;
2 warnings generated.
[ 32%] Linking CXX static library ../../../libLLVMAVRDesc.a
[ 32%] Built target LLVMAVRDesc
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86WinCOFFObjectWriter.cpp.o
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86WinCOFFObjectWriter.cpp.o
[ 32%] Building CXX object tools/llvm-bcanalyzer/CMakeFiles/llvm-bcanalyzer.dir/llvm-bcanalyzer.cpp.o
[ 32%] Linking CXX static library ../../../libLLVMM68kDesc.a
[ 32%] Built target LLVMM68kDesc
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86WinCOFFStreamer.cpp.o
[ 32%] Linking CXX executable ../../bin/llvm-bcanalyzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 32%] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86WinCOFFTargetStreamer.cpp.o
[ 32%] Building CXX object tools/llvm-dis/CMakeFiles/llvm-dis.dir/llvm-dis.cpp.o
[ 32%] Building StripOpts.inc...
[ 32%] Built target StripOptsTableGen
[ 32%] Built target StripOptsTableGen
[ 32%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/Archive.cpp.o
[ 32%] Linking CXX executable ../../bin/llvm-dis
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 32%] Built target LLVMX86Desc
[ 32%] Building CXX object lib/Object/CMakeFiles/LLVMObject.dir/ArchiveWriter.cpp.o
[ 32%] Building CXX object lib/IRReader/CMakeFiles/LLVMIRReader.dir/IRReader.cpp.o
[ 32%] Built target llvm-dis
---
[ 33%] Building CXX object tools/llvm-cxxdump/CMakeFiles/llvm-cxxdump.dir/llvm-cxxdump.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRRecords.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Building CXX object tools/llvm-cxxdump/CMakeFiles/llvm-cxxdump.dir/Error.cpp.o
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceExpander.cpp.o
[ 33%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/llvm-diff.cpp.o
[ 33%] Linking CXX executable ../../bin/llvm-cxxdump
[ 33%] Linking CXX executable ../../bin/llvm-cxxdump
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FDRTraceWriter.cpp.o
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/FileHeaderReader.cpp.o
[ 33%] Building CXX object tools/llvm-opt-report/CMakeFiles/llvm-opt-report.dir/OptReport.cpp.o
[ 33%] Built target llvm-diff
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/llvm-rc.cpp.o
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/llvm-rc.cpp.o
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/InstrumentationMap.cpp.o
[ 33%] Linking CXX executable ../../bin/llvm-opt-report
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceFileWriter.cpp.o
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceFileWriter.cpp.o
[ 33%] Building CXX object tools/llvm-remark-size-diff/CMakeFiles/llvm-remark-size-diff.dir/RemarkSizeDiff.cpp.o
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptCppFilter.cpp.o
[ 33%] Linking CXX executable ../../bin/llvm-remark-size-diff
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Built target llvm-remark-size-diff
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptParser.cpp.o
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordInitializer.cpp.o
[ 33%] Building CXX object tools/llvm-size/CMakeFiles/llvm-size.dir/llvm-size.cpp.o
[ 33%] Building CXX object tools/llvm-size/CMakeFiles/llvm-size.dir/llvm-size.cpp.o
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptStmt.cpp.o
[ 33%] Building CXX object lib/XRay/CMakeFiles/LLVMXRay.dir/RecordPrinter.cpp.o
[ 33%] Building CXX object tools/llvm-rc/CMakeFiles/llvm-rc.dir/ResourceScriptToken.cpp.o
[ 33%] Linking CXX executable ../../bin/llvm-size
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Linking CXX executable ../../bin/llvm-rc
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[ 33%] Built target llvm-size
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 33%] Linking CXX static library ../libLLVMXRay.a
[ 33%] Built target LLVMXRay
[ 33%] Built target llvm-rc
[ 33%] Building CXX object tools/llvm-tapi-diff/CMakeFiles/llvm-tapi-diff.dir/llvm-tapi-diff.cpp.o
[ 33%] Building CXX object tools/llvm-tapi-diff/CMakeFiles/llvm-tapi-diff.dir/llvm-tapi-diff.cpp.o
[ 33%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/GenericError.cpp.o
[ 34%] Linking CXX executable ../../bin/llvm-strings
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 34%] Built target llvm-strings
[ 34%] Building CXX object tools/llvm-tapi-diff/CMakeFiles/llvm-tapi-diff.dir/DiffEngine.cpp.o
[ 34%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAbbreviationDeclaration.cpp.o
[ 34%] Linking CXX executable ../../bin/llvm-tapi-diff
[ 34%] Linking CXX executable ../../bin/llvm-tapi-diff
[ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDB.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 34%] Built target llvm-tapi-diff
[ 34%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBContext.cpp.o
[ 34%] Building CXX object lib/InterfaceStub/CMakeFiles/LLVMInterfaceStub.dir/ELFObjHandler.cpp.o
[ 34%] Building CXX object lib/DebugInfo/DWARF/CMakeFiles/LLVMDebugInfoDWARF.dir/DWARFAcceleratorTable.cpp.o
---
[ 36%] Building CXX object lib/ObjCopy/CMakeFiles/LLVMObjCopy.dir/XCOFF/XCOFFObjcopy.cpp.o
[ 36%] Linking CXX static library ../../libLLVMDebugInfoDWARF.a
[ 36%] Built target LLVMDebugInfoDWARF
[ 36%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeManaged.cpp.o
[ 36%] Building CXX object lib/ObjCopy/CMakeFiles/LLVMObjCopy.dir/XCOFF/XCOFFReader.cpp.o
[ 36%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypePointer.cpp.o
[ 36%] Building CXX object lib/ObjCopy/CMakeFiles/LLVMObjCopy.dir/XCOFF/XCOFFWriter.cpp.o
[ 36%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/ArchiveYAML.cpp.o
[ 36%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeTypedef.cpp.o
---
[ 37%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/LookupResult.cpp.o
[ 37%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleList.cpp.o
[ 37%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ObjectFileTransformer.cpp.o
[ 37%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStream.cpp.o
[ 37%] Building CXX object lib/ObjectYAML/CMakeFiles/LLVMObjectYAML.dir/DXContainerYAML.cpp.o
[ 38%] Building CXX object lib/DebugInfo/GSYM/CMakeFiles/LLVMDebugInfoGSYM.dir/ExtractRanges.cpp.o
[ 38%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStreamBuilder.cpp.o
[ 38%] Linking CXX static library ../../libLLVMDebugInfoGSYM.a
[ 38%] Built target LLVMDebugInfoGSYM
[ 38%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/EnumTables.cpp.o
---
[ 40%] Building CXX object tools/llvm-objcopy/CMakeFiles/llvm-objcopy.dir/llvm-objcopy-driver.cpp.o
[ 40%] Linking CXX executable ../../bin/llvm-objcopy
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 40%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeInlineSiteSymbol.cpp.o
[ 40%] Built target llvm-objcopy
[ 40%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/MachO_arm64.cpp.o
[ 40%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeLineNumber.cpp.o
---
[ 41%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELFLinkGraphBuilder.cpp.o
[ 41%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/dwarf2yaml.cpp.o
[ 41%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeTypeArray.cpp.o
[ 41%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_aarch64.cpp.o
[ 41%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/dxcontainer2yaml.cpp.o
[ 41%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/elf2yaml.cpp.o
[ 41%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_riscv.cpp.o
[ 41%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeTypeEnum.cpp.o
[ 41%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/ELF_x86_64.cpp.o
---
[ 42%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NamedStreamMap.cpp.o
[ 42%] Linking CXX executable ../../bin/obj2yaml
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 42%] Building CXX object lib/ExecutionEngine/JITLink/CMakeFiles/LLVMJITLink.dir/x86_64.cpp.o
[ 42%] Built target obj2yaml
[ 42%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBFile.cpp.o
[ 42%] Linking CXX static library ../../libLLVMJITLink.a
---
[ 43%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PDBStringTableBuilder.cpp.o
[ 44%] Linking CXX executable ../../bin/yaml2obj
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/PublicsStream.cpp.o
[ 44%] Built target yaml2obj
[ 44%] Building CXX object tools/llvm-ifs/CMakeFiles/llvm-ifs.dir/llvm-ifs.cpp.o
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GSIStreamBuilder.cpp.o
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GSIStreamBuilder.cpp.o
[ 44%] Generating ../../bin/llvm-install-name-tool
[ 44%] Linking CXX executable ../../bin/llvm-ifs
[ 44%] Built target llvm-install-name-tool
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
[ 44%] Generating ../../bin/llvm-bitcode-strip
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 44%] Built target llvm-bitcode-strip
[ 44%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/SymbolCache.cpp.o
[ 44%] Generating ../../bin/llvm-strip
[ 44%] Built target llvm-strip
---
[ 46%] Linking CXX executable ../../bin/llvm-readobj
[ 46%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/PrettyExternalSymbolDumper.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 46%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/PrettyFunctionDumper.cpp.o
[ 46%] Built target llvm-readobj
[ 46%] Building CXX object lib/ProfileData/CMakeFiles/LLVMProfileData.dir/MemProf.cpp.o
[ 46%] Building CXX object tools/llvm-pdbutil/CMakeFiles/llvm-pdbutil.dir/PrettyTypeDumper.cpp.o
---
[ 46%] Linking CXX executable ../../bin/llvm-pdbutil
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
[ 46%] Linking CXX static library ../libLLVMProfileData.a
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 46%] Building CXX object tools/llvm-debuginfod/CMakeFiles/llvm-debuginfod.dir/llvm-debuginfod.cpp.o
[ 46%] Linking CXX static library ../../../lib/libLLVMCFIVerify.a
[ 46%] Built target LLVMCFIVerify
[ 46%] Building CXX object tools/llvm-debuginfod-find/CMakeFiles/llvm-debuginfod-find.dir/llvm-debuginfod-find.cpp.o
[ 46%] Building CXX object tools/llvm-debuginfod-find/CMakeFiles/llvm-debuginfod-find.dir/llvm-debuginfod-find.cpp.o
[ 46%] Linking CXX executable ../../bin/llvm-debuginfod
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 46%] Generating ../../bin/llvm-readelf
[ 46%] Built target llvm-readelf
[ 46%] Building CXX object tools/llvm-symbolizer/CMakeFiles/llvm-symbolizer.dir/llvm-symbolizer.cpp.o
[ 46%] Built target llvm-debuginfod-find
[ 46%] Built target llvm-debuginfod-find
[ 46%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/func-id-helper.cpp.o
[ 46%] Built target llvm-debuginfod
[ 46%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/llvm-xray.cpp.o
[ 46%] Building CXX object tools/sanstats/CMakeFiles/sanstats.dir/sanstats.cpp.o
[ 46%] Linking CXX executable ../../bin/llvm-symbolizer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 46%] Linking CXX executable ../../bin/sanstats
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 46%] Built target llvm-symbolizer
[ 46%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-converter.cpp.o
[ 46%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/AliasAnalysis.cpp.o
[ 46%] Built target sanstats
---
[ 47%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/AssumptionCache.cpp.o
[ 47%] Linking CXX executable ../../bin/llvm-xray
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 47%] Built target llvm-xray
[ 47%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/BlockFrequencyInfo.cpp.o
[ 47%] Built target llvm-profdata
[ 47%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/BlockFrequencyInfoImpl.cpp.o
---
[ 48%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CmpInstAnalysis.cpp.o
[ 48%] Linking CXX executable ../../bin/llvm-cov
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 48%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CodeMetrics.cpp.o
[ 48%] Built target llvm-cov
[ 48%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/ConstantFolding.cpp.o
[ 48%] Building CXX object lib/Analysis/CMakeFiles/LLVMAnalysis.dir/CycleAnalysis.cpp.o
---
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BuildLibCalls.cpp.o
[ 52%] Linking CXX executable ../../bin/llvm-sim
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BypassSlowDivision.cpp.o
[ 52%] Built target llvm-sim
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CallPromotionUtils.cpp.o
[ 52%] Linking CXX executable ../../bin/llvm-stress
[ 52%] Linking CXX executable ../../bin/llvm-stress
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CallGraphUpdater.cpp.o
[ 52%] Built target llvm-stress
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CanonicalizeAliases.cpp.o
[ 52%] Building CXX object tools/verify-uselistorder/CMakeFiles/verify-uselistorder.dir/verify-uselistorder.cpp.o
[ 52%] Building CXX object tools/verify-uselistorder/CMakeFiles/verify-uselistorder.dir/verify-uselistorder.cpp.o
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CanonicalizeFreezeInLoops.cpp.o
[ 52%] Linking CXX executable ../../bin/llvm-tli-checker
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneFunction.cpp.o
[ 52%] Linking CXX executable ../../bin/verify-uselistorder
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneModule.cpp.o
[ 52%] Built target verify-uselistorder
[ 52%] Building CXX object lib/DWP/CMakeFiles/LLVMDWP.dir/DWPError.cpp.o
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CodeExtractor.cpp.o
---
[ 52%] Linking CXX executable ../../bin/llvm-as
[ 52%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/ExecutionEngineBindings.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 52%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/GDBRegistrationListener.cpp.o
[ 52%] Built target llvm-as
[ 52%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/DemoteRegToStack.cpp.o
[ 53%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/SectionMemoryManager.cpp.o
[ 53%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/SectionMemoryManager.cpp.o
[ 53%] Building CXX object tools/llvm-cat/CMakeFiles/llvm-cat.dir/llvm-cat.cpp.o
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EntryExitInstrumenter.cpp.o
[ 53%] Building CXX object lib/ExecutionEngine/CMakeFiles/LLVMExecutionEngine.dir/TargetSelect.cpp.o
[ 53%] Linking CXX executable ../../bin/llvm-cat
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 53%] Built target LLVMExecutionEngine
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EscapeEnumerator.cpp.o
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Evaluator.cpp.o
[ 53%] Built target llvm-cat
---
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FunctionImportUtils.cpp.o
[ 53%] Linking CXX executable ../../bin/llvm-modextract
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GuardUtils.cpp.o
[ 53%] Built target llvm-modextract
[ 53%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/HelloWorld.cpp.o
[ 53%] Building CXX object lib/ExecutionEngine/MCJIT/CMakeFiles/LLVMMCJIT.dir/MCJIT.cpp.o
---
[ 58%] Linking CXX executable ../../bin/llvm-split
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
[ 58%] Built target LLVMInstrumentation
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 58%] Building CXX object lib/Transforms/ObjCARC/CMakeFiles/LLVMObjCARCOpts.dir/ObjCARCContract.cpp.o
[ 58%] Built target llvm-split
[ 58%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/AlignmentFromAssumptions.cpp.o
[ 58%] Building CXX object lib/Transforms/ObjCARC/CMakeFiles/LLVMObjCARCOpts.dir/DependencyAnalysis.cpp.o
---
[ 62%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/SpeculativeExecution.cpp.o
[ 62%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/StraightLineStrengthReduce.cpp.o
[ 62%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/StructurizeCFG.cpp.o
[ 62%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/TailRecursionElimination.cpp.o
[ 62%] Building CXX object lib/Transforms/Scalar/CMakeFiles/LLVMScalarOpts.dir/TLSVariableHoist.cpp.o
[ 62%] Linking CXX static library ../../libLLVMScalarOpts.a
[ 62%] Built target LLVMScalarOpts
[ 62%] Building CXX object lib/FuzzMutate/CMakeFiles/LLVMFuzzMutate.dir/IRMutator.cpp.o
[ 62%] Building CXX object lib/Frontend/OpenMP/CMakeFiles/LLVMFrontendOpenMP.dir/OMP.cpp.o
---
[ 66%] Building CXX object lib/Transforms/Coroutines/CMakeFiles/LLVMCoroutines.dir/CoroCleanup.cpp.o
[ 66%] Linking CXX executable ../../bin/llvm-extract
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 66%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/JMCInstrumenter.cpp.o
[ 66%] Building CXX object lib/Transforms/Coroutines/CMakeFiles/LLVMCoroutines.dir/CoroConditionalWrapper.cpp.o
[ 66%] Built target llvm-extract
[ 66%] Building CXX object tools/llvm-link/CMakeFiles/llvm-link.dir/llvm-link.cpp.o
[ 66%] Building CXX object lib/Transforms/Coroutines/CMakeFiles/LLVMCoroutines.dir/CoroEarly.cpp.o
[ 66%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LazyMachineBlockFrequencyInfo.cpp.o
[ 66%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LazyMachineBlockFrequencyInfo.cpp.o
[ 66%] Linking CXX executable ../../bin/llvm-link
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 66%] Built target llvm-link
[ 66%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LexicalScopes.cpp.o
[ 66%] Building CXX object lib/Transforms/Coroutines/CMakeFiles/LLVMCoroutines.dir/CoroFrame.cpp.o
[ 66%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/LiveDebugVariables.cpp.o
---
[ 70%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MultiHazardRecognizer.cpp.o
[ 70%] Linking CXX executable ../../../bin/lli-child-target
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 70%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/PatchableFunction.cpp.o
[ 70%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MBFIWrapper.cpp.o
[ 70%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRPrinter.cpp.o
[ 70%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MIRPrintingPass.cpp.o
---
[ 88%] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kISelDAGToDAG.cpp.o
[ 88%] Linking CXX executable ../../bin/llvm-dwarfdump
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 88%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Analysis.cpp.o
[ 88%] Building CXX object lib/Target/M68k/CMakeFiles/LLVMM68kCodeGen.dir/M68kMachineFunction.cpp.o
[ 88%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Assembler.cpp.o
[ 88%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86IndirectThunks.cpp.o
---
[ 88%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/SnippetRepetitor.cpp.o
[ 88%] Linking CXX executable ../../bin/llvm-jitlink
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 88%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/llvm-objdump.cpp.o
[ 88%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86MCInstLower.cpp.o
[ 88%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/Target.cpp.o
[ 88%] Building CXX object tools/llvm-exegesis/lib/CMakeFiles/LLVMExegesis.dir/UopsBenchmarkRunner.cpp.o
---
[ 88%] Building CXX object tools/llvm-profgen/CMakeFiles/llvm-profgen.dir/ProfiledBinary.cpp.o
[ 88%] Linking CXX executable ../../bin/llvm-objdump
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 88%] Built target llvm-objdump
[ 88%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ReturnThunks.cpp.o
[ 88%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SelectionDAGInfo.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ShuffleDecodeConstantPool.cpp.o
[ 89%] Building CXX object tools/llvm-rtdyld/CMakeFiles/llvm-rtdyld.dir/llvm-rtdyld.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeLoadHardening.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeLoadHardening.cpp.o
[ 89%] Linking CXX executable ../../bin/llvm-profgen
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeExecutionSideEffectSuppression.cpp.o
[ 89%] Linking CXX executable ../../bin/llvm-rtdyld
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86Subtarget.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetMachine.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetObjectFile.cpp.o
[ 89%] Building CXX object tools/sancov/CMakeFiles/sancov.dir/sancov.cpp.o
[ 89%] Building CXX object tools/sancov/CMakeFiles/sancov.dir/sancov.cpp.o
[ 89%] Copying llvm-locstats into /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/./bin
[ 89%] Built target llvm-locstats
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetTransformInfo.cpp.o
[ 89%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/GISel/AArch64CallLowering.cpp.o
[ 89%] Linking CXX executable ../../bin/sancov
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 89%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/A15SDOptimizer.cpp.o
[ 89%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86VZeroUpper.cpp.o
[ 89%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/GISel/AArch64GlobalISelUtils.cpp.o
[ 89%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMAsmPrinter.cpp.o
---
[ 90%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/GISel/AArch64RegisterBankInfo.cpp.o
[ 90%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMFastISel.cpp.o
[ 90%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64A57FPLoadBalancing.cpp.o
[ 90%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsDelaySlotFiller.cpp.o
[ 90%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMFixCortexA57AES1742098Pass.cpp.o
[ 90%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsExpandPseudo.cpp.o
[ 90%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64AsmPrinter.cpp.o
[ 90%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMFrameLowering.cpp.o
[ 90%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFastISel.cpp.o
---
[ 94%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
[ 94%] Linking CXX executable ../../bin/lli
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 94%] Building CXX object tools/llvm-cfi-verify/CMakeFiles/llvm-cfi-verify.dir/llvm-cfi-verify.cpp.o
[ 94%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/Thumb1InstrInfo.cpp.o
[ 94%] Linking CXX executable ../../bin/llvm-cfi-verify
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 94%] Building CXX object tools/llvm-exegesis/lib/X86/CMakeFiles/LLVMExegesisX86.dir/Target.cpp.o
[ 94%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
[ 94%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ThumbRegisterInfo.cpp.o
[ 94%] Building CXX object tools/llvm-exegesis/lib/X86/CMakeFiles/LLVMExegesisX86.dir/X86Counter.cpp.o
---
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/CodeRegion.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-mc
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Built target llvm-mc
[ 96%] Building CXX object tools/llvm-ml/CMakeFiles/llvm-ml.dir/Disassembler.cpp.o
[ 96%] Building CXX object tools/llvm-nm/CMakeFiles/llvm-nm.dir/llvm-nm.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/PipelinePrinter.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/PipelinePrinter.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ml
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/BottleneckAnalysis.cpp.o
[ 96%] Generating ../../bin/llvm-otool
[ 96%] Built target llvm-otool
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/DispatchStatistics.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/DispatchStatistics.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/InstructionInfoView.cpp.o
[ 96%] Building CXX object tools/llvm-opt-fuzzer/CMakeFiles/llvm-opt-fuzzer.dir/DummyOptFuzzer.cpp.o
[ 96%] Building CXX object tools/llvm-opt-fuzzer/CMakeFiles/llvm-opt-fuzzer.dir/llvm-opt-fuzzer.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-nm
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/InstructionView.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/DeltaManager.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/RegisterFileStatistics.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/ResourcePressureView.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/ResourcePressureView.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/RetireControlUnitStatistics.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/ReducerWorkItem.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/SchedulerStatistics.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-opt-fuzzer
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Built target llvm-opt-fuzzer
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/TestRunner.cpp.o
[ 96%] Building CXX object tools/llvm-mca/CMakeFiles/llvm-mca.dir/Views/TimelineView.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/AnalysisWrappers.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/AnalysisWrappers.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-mca
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Delta.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/BreakpointPrinter.cpp.o
[ 96%] Building CXX object tools/lto/CMakeFiles/LTO.dir/LTODisassembler.cpp.o
[ 96%] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.o
[ 96%] Building CXX object tools/lto/CMakeFiles/LTO.dir/lto.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/GraphPrinters.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/NewPMDriver.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/Utils.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceAliases.cpp.o
[ 96%] Linking CXX shared library ../../lib/libLTO.dylib
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/PrintSCC.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceArguments.cpp.o
[ 96%] Building CXX object tools/opt/CMakeFiles/opt.dir/opt.cpp.o
[ 96%] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.o
[ 96%] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceAttributes.cpp.o
[ 96%] Building CXX object tools/llvm-ar/CMakeFiles/llvm-ar.dir/llvm-ar-driver.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-ar
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceBasicBlocks.cpp.o
[ 96%] Linking CXX executable ../../bin/opt
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/llvm-lto/CMakeFiles/llvm-lto.dir/llvm-lto.cpp.o
[ 96%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/BugDriver.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctionBodies.cpp.o
[ 96%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/CrashDebugger.cpp.o
[ 96%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/CrashDebugger.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceFunctions.cpp.o
[ 96%] Linking CXX executable ../../bin/llvm-lto
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 96%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExecutionDriver.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalObjects.cpp.o
[ 96%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/ExtractFunction.cpp.o
[ 96%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceGlobalValues.cpp.o
---
[ 97%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/CFBundle.cpp.o
[ 97%] Linking CXX executable ../../bin/bugpoint
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 97%] Built target bugpoint
[ 97%] Building CXX object tools/llc/CMakeFiles/llc.dir/llc.cpp.o
[ 97%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceSpecialGlobals.cpp.o
[ 97%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DwarfLinkerForBinary.cpp.o
[ 97%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/DwarfLinkerForBinary.cpp.o
[ 97%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceOperands.cpp.o
[ 97%] Linking CXX executable ../../bin/llc
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 97%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachODebugMapParser.cpp.o
[ 97%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceOperandsSkip.cpp.o
[ 97%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/attributes.c.o
[ 97%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/MachOUtils.cpp.o
---
[ 98%] Building C object tools/llvm-c-test/CMakeFiles/llvm-c-test.dir/targets.c.o
[ 98%] Linking CXX executable ../../bin/llvm-c-test
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceOperandsToArgs.cpp.o
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructionsMIR.cpp.o
[ 99%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/Reproducer.cpp.o
[ 99%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/SymbolMap.cpp.o
[ 99%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/SymbolMap.cpp.o
[ 99%] Building CXX object tools/llvm-dwarfutil/CMakeFiles/llvm-dwarfutil.dir/llvm-dwarfutil.cpp.o
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceInstructionFlagsMIR.cpp.o
[ 99%] Building CXX object tools/dsymutil/CMakeFiles/dsymutil.dir/dsymutil-driver.cpp.o
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 99%] Built target dsymutil
[ 99%] Building CXX object tools/llvm-dwarfutil/CMakeFiles/llvm-dwarfutil.dir/DebugInfoLinker.cpp.o
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceIRReferences.cpp.o
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceVirtualRegisters.cpp.o
[ 99%] Linking CXX executable ../../bin/llvm-dwarfutil
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 99%] Built target llvm-dwarfutil
[ 99%] Building CXX object tools/llvm-exegesis/CMakeFiles/llvm-exegesis.dir/llvm-exegesis.cpp.o
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceRegisterMasks.cpp.o
[ 99%] Linking CXX executable ../../bin/llvm-dwp
[ 99%] Linking CXX executable ../../bin/llvm-dwp
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 99%] Building CXX object tools/llvm-exegesis/lib/AArch64/CMakeFiles/LLVMExegesisAArch64.dir/Target.cpp.o
[ 99%] Linking CXX executable ../../bin/llvm-exegesis
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[ 99%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceRegisterDefs.cpp.o
[100%] Building CXX object tools/llvm-gsymutil/CMakeFiles/llvm-gsymutil.dir/llvm-gsymutil.cpp.o
[100%] Linking CXX static library ../../../../lib/libLLVMExegesisAArch64.a
[100%] Built target LLVMExegesisAArch64
[100%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceRegisterUses.cpp.o
[100%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/ReduceRegisterUses.cpp.o
[100%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/deltas/SimplifyInstructions.cpp.o
[100%] Linking CXX executable ../../bin/llvm-gsymutil
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[100%] Building CXX object tools/llvm-isel-fuzzer/CMakeFiles/llvm-isel-fuzzer.dir/llvm-isel-fuzzer.cpp.o
[100%] Built target llvm-gsymutil
[100%] Building CXX object tools/llvm-libtool-darwin/CMakeFiles/llvm-libtool-darwin.dir/llvm-libtool-darwin.cpp.o
[100%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[100%] Building CXX object tools/llvm-reduce/CMakeFiles/llvm-reduce.dir/llvm-reduce.cpp.o
[100%] Linking CXX executable ../../bin/llvm-libtool-darwin
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[100%] Building CXX object tools/llvm-lipo/CMakeFiles/llvm-lipo.dir/llvm-lipo.cpp.o
[100%] Built target llvm-isel-fuzzer
[100%] Building CXX object tools/llvm-lto2/CMakeFiles/llvm-lto2.dir/llvm-lto2.cpp.o
[100%] Linking CXX executable ../../bin/llvm-lipo
[100%] Linking CXX executable ../../bin/llvm-lipo
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[100%] Generating ../../bin/llvm-ranlib
[100%] Built target llvm-ranlib
[100%] Generating ../../bin/llvm-lib
[100%] Built target llvm-lib
[100%] Built target llvm-lib
[100%] Generating ../../bin/llvm-dlltool
[100%] Built target llvm-dlltool
[100%] Linking CXX executable ../../bin/llvm-reduce
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
[100%] Linking CXX executable ../../bin/llvm-lto2
clang-14: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: building for macOS 10.7.0 is deprecated
ld: warning: dylib (/usr/local/lib/libzstd.1.5.2.dylib) was built for newer macOS version (11.0) than being linked (10.7)
Install the project...
-- Install configuration: "Release"
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ADT
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Frontend/OpenMP/OMPKinds.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Frontend/OpenMP/OMPGridValues.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Frontend/OpenMP/OMP.td
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData/MIBEntryDef.inc
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData/InstrProfReader.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData/ProfileCommon.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData/SampleProfWriter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ProfileData/MemProfData.inc
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/AsmParser/LLToken.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/Type.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/TrackingMDRef.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicsDirectX.td
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IRPrintingPasses.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/SymbolTableListTraits.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/MatrixBuilder.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/LegacyPassManager.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/VE.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/ARC.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/AArch64.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/LoongArch.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/Sparc.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/M68k.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/AMDGPU.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/BinaryFormat/ELFRelocs/PowerPC64.def
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/CodeViewYAMLDebugSections.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/XCOFFYAML.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/CodeViewYAMLTypeHashing.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/YAML.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/DXContainerYAML.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/ArchiveYAML.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/CodeViewYAMLSymbols.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/MinidumpYAML.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjectYAML/yaml2obj.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/CodeGen/MachineInstr.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/CodeGen/RegisterBank.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/CodeGen/Analysis.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/CodeGen/ISDOpcodes.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/CodeGen/CFIFixup.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Debuginfod/DIFetcher.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Debuginfod/Debuginfod.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Debuginfod/HTTPServer.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Debuginfod/HTTPClient.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/ELF_aarch64.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/MachO_x86_64.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/JITLinkMemoryManager.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/JITLinkDylib.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/DWARFRecordSectionSplitter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/ELF.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/MachO_arm64.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/ELF_riscv.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ExecutionEngine/JITLink/MemoryFlags.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/MakeGuardsExplicit.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/LoopLoadElimination.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/DeadStoreElimination.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/LoopDistribute.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/TLSVariableHoist.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/LowerAtomicPass.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Scalar/LICM.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Vectorize
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Vectorize/LoadStoreVectorizer.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Transforms/Vectorize/SLPVectorizer.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/BinaryStream.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/Timer.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CFGDiff.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CodeGen.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CSKYTargetParser.def
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/Unicode.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/TargetParser.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/Printable.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/AtomicOrdering.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/AtomicOrdering.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/thread.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/RecyclingAllocator.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/AMDHSAKernelDescriptor.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/SpecialCaseList.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/DXILOperationCommon.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/SuffixTree.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/Capacity.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/Allocator.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/LowLevelTypeImpl.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/LowLevelTypeImpl.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/FormatVariadicDetails.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CheckedArithmetic.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CSKYAttributeParser.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/JSON.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/DOTGraphTraits.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/MSVCErrorWorkarounds.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/CSKYTargetParser.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/AArch64TargetParser.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/KnownBits.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/DJB.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/SMTAPI.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ToolDrivers/llvm-dlltool
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ToolDrivers/llvm-dlltool/DlltoolDriver.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Pass.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSPIRVObjectWriter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCExpr.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/TargetRegistry.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCLinkerOptimizationHint.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCXCOFFObjectWriter.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSectionMachO.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/SectionKind.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/SubtargetFeature.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCMachObjectWriter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCDecoderOps.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCDwarf.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSPIRVStreamer.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCCodeEmitter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCAsmInfoWasm.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCXCOFFStreamer.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCPseudoProbe.h
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSectionELF.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCFixup.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCLabel.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCFixupKindInfo.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCDXContainerWriter.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSection.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCAsmLayout.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/MC/MCSymbolGOFF.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/InterfaceStub
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/MachO
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/MachO/MachOObjcopy.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/MachO/MachOConfig.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/ConfigManager.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/wasm
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/wasm/WasmConfig.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/wasm/WasmObjcopy.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/XCOFF
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/XCOFF/XCOFFObjcopy.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/XCOFF/XCOFFConfig.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/ObjCopy.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/MultiFormatConfig.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/ELF
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/ELF/ELFObjcopy.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/ObjCopy/ELF/ELFConfig.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/InitializePasses.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/InitializePasses.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/LinkAllPasses.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IRReader
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IRReader/IRReader.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/WindowsDriver
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/WindowsDriver/MSVCPaths.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/WindowsDriver/MSVCSetupApi.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/TableGen/Main.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/TableGen/Error.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/TableGen/SetTheory.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/TableGen/SearchableTable.td
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicImpl.inc
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicsHexagon.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicEnums.inc
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicsAMDGPU.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/IR/IntrinsicsDirectX.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/include/llvm/Support/VCSRevision.h
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/lib/cmake/llvm/LLVMConfigExtensions.cmake
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/lib/libLLVMDemangle.a
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/lib/libLLVMSupport.a
---
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/FileCheck
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/llvm-PerfectShuffle
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/count
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/not
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/UnicodeNameMappingGenerator
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/lib/libLTO.dylib
-- Installing: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/bin/llvm-ar
CMake Warning (dev) at /usr/local/Cellar/cmake/3.23.2/share/cmake/Modules/GNUInstallDirs.cmake:241 (message):
-- Creating llvm-ranlib
---
[RUSTC-TIMING] rustc_interface test:false 48.421
   Compiling rustc_driver v0.0.0 (/Users/runner/work/rust/rust/compiler/rustc_driver)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-Wl,-exported_symbols_list,/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/rustcR4zaIu/list" "-m64" "-arch" "x86_64" "/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/rustcR4zaIu/symbols.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.0.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.1.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.10.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.11.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.12.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.13.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.14.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.15.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.2.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.3.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.4.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.5.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.6.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.7.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.8.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.rustc_driver.978c8767-cgu.9.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/rustc_driver-0324347d7c661795.ufqdh23z9g90af6.rcgu.rmeta" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/release/deps" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/psm-2e3b7efa763fe02b/out" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/build/rustc_llvm-7be5b74ca5717ccb/out" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/lib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_error_codes-e86273ae87c8fcc7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_save_analysis-3236cc1a114089f1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librls_data-548a45ad540ffbc9.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librls_span-5af0252bff573780.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_log-a484881324e0e55b.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_interface-2e99fb8812f6861f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_codegen_llvm-b170cde13d379043.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_llvm-78ef19df4cbb5312.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_privacy-9128df1a820ddb6f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_monomorphize-364d273fea76dffc.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_mir_transform-6d0b36643278f31e.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_ast_lowering-9b68508ed68f30b1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_builtin_macros-b696330f35641f81.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_typeck-0a49a1a66ce9f8de.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_ty_utils-74be62762d30f423.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_resolve-c4280598b524b6df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_passes-b353016f136c6d54.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_mir_build-948362a9828d949b.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_borrowck-62bf59aacc2dc479.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_traits-11e2d31ba81e3d59.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libchalk_engine-7c2f44b4318a7f62.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libchalk_solve-b991aad4058bfd35.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtracing_tree-38884f78465a9ff4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtracing_subscriber-315f7fc15d7a4365.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libthread_local-c915a1c9c4a3a8f3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libsharded_slab-35d0c2865dd533ef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libansi_term-4e4f4832f0167b12.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libmatchers-bd1398018600651d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libregex_automata-49bcf72af0d68ce7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtracing_log-98a175d20efadb1d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpetgraph-676635062a78c3ad.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libfixedbitset-344d7d4d38f6abed.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_const_eval-ceee6ac0bdb328ed.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_mir_dataflow-ca5a87bdd83dfb2a.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_query_impl-ee0bfa54bd3acb29.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_codegen_ssa-9bad883a737f2aa8.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libthorin-6bec57c27b9b61cf.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libgimli-b9ab1ac55513e2ef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libfallible_iterator-2ab1b4b21e4c3313.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_symbol_mangling-ef624afc87b5adae.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_demangle-d8938e27e1b6cce2.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpunycode-686836d74dd598d1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_incremental-569637c782e5bb21.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpathdiff-f128e9e9b7a86127.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libobject-b0441d758968b59f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libflate2-39299d2c1e80e4ee.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libminiz_oxide-a4e2785760f1faa0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libadler-e275b06fed309ee3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcrc32fast-85e369cda0db44a7.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libregex-a3fd5e0338dab51f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libaho_corasick-b765b706502d0b34.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libmemchr-61a1ba80c5e5a2ba.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libregex_syntax-15d196262f1fbe69.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcc-ba1272ab9bdec785.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_plugin_impl-c5172d6d43cd71df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_metadata-425a62c3c9b2fe50.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_hir_pretty-5c09db8710095780.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libsnap-7ca9fdf5c32911fe.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_expand-d4ead318fae47e10.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcrossbeam_channel-1be0297eb7239385.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcrossbeam_utils-d7f738fc8ee7ee86.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_ast_passes-5bf8f7733f6134d0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libitertools-314fc77e3ca00ca4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_parse-4d88099ecceac72f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libproc_macro-c52bd3e2ad3fee96.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblibloading-d2c172f4cbc49e82.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_lint-2fd612329b81ab0d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunicode_security-116f0a426af0a557.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunicode_script-28643bd19d78e419.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunicode_normalization-5f7f7cd0f029915e.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtinyvec-1cfd15c60060f722.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_trait_selection-61057117ceb93afe.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_transmute-697e32f2579b8eb5.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_parse_format-51deafef4c7dbd05.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_infer-0267d3cae732db11.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_middle-e9bbf24013c21948.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libchalk_ir-6ea05312be4d7ff0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librand_xoshiro-ef388572db7fd397.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_type_ir-2ae95690b11ff913.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_apfloat-e0475334ab3223b1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libgsgdt-ce333a343f59f128.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libeither-33babc3eee1e2d05.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpolonius_engine-e1bdbea55fe599f0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libdatafrog-58d7a437a68d7017.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_attr-673bdaefbd94db71.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_ast_pretty-263cabc45aa48106.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_query_system-cbe0a7bf21f846c8.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_session-4b985e354ffb74dd.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libnum_cpus-a32ea7e731ec3318.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libgetopts-9dd71d502e6dea58.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_fs_util-fd1f28b1b2a5672b.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_feature-4e96d5898ac116d0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_errors-f3f4bb7218e97557.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtermize-97d03fa564fd17ff.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libatty-063d88ec04e0bf2e.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libannotate_snippets-a33ec8aac5312f4f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtermcolor-0828c07a7a8c08a3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_lint_defs-9bee71691e24818b.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_hir-11f1828fb8ad8166.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libodht-f8646e3eb5754333.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_target-6df0192b72e2388f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libserde_json-d8f971a48f3545b9.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libryu-c7da23295424feef.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libitoa-850f689d086d5bfa.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libserde-71bd4673740b4e35.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_error_messages-5c29be6e73cca3b8.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libfluent_bundle-3fbe3aa7ca36b3ed.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libfluent_langneg-b936c91172680ace.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libintl_pluralrules-18385ceac20957ba.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libself_cell-69f134df5ddf2640.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libintl_memoizer-a09f3c8cd0f2267b.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtype_map-caab1e66386d4369.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_langid-28f959a9b77610b0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_langid_macros-99eba00d23249bf9.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_langid_impl-6f60968150f19000.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtinystr-be9882c3504254f4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libfluent_syntax-3dfc5709bc2ed5b3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libthiserror-c3ffb3d538ca3864.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_ast-6b4f0eb6e0e46502.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_lexer-35e5f0aa7cadba30.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_emoji_char-6d5cdb5f42be9676.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_ucd_version-4f5a61e8f7e43951.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_common-92839c0dc84555a9.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_char_property-aefb9d7f48c55b62.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunic_char_range-2082afca0522cda6.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunicode_xid-113c1597c32905af.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_span-1ae84ab4623b4f9a.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libscoped_tls-3dae267bba88c6e8.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libsha2-4b21b39a1c3622b4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libsha1-20241e0c8212455f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcpufeatures-01b654600ee905df.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libmd5-ebda67136302084c.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libdigest-3a3e7eba1565fbae.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libblock_buffer-4392204a91475261.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcrypto_common-e3d9a3611c27f22e.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libgeneric_array-3e1ad13db111dca1.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtypenum-0eeab100662e42c3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libunicode_width-b6bedf0f366f3f83.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_arena-d0fc3e38236b3bb0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_data_structures-c563946f82d3657d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libstacker-56e4e8447fed7155.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpsm-8548afb510996b4f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libmemmap2-00ece993b892a5d2.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtempfile-e5ca187475fdaad0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librand-00e721da804e5d59.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librand_chacha-2d66bfcc57bc9cd9.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libppv_lite86-d127b004a6eeae01.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librand_core-ac427fdaed2ef8ff.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libremove_dir_all-08a769f918ac1b68.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libmeasureme-dc5d3ba1d52a1770.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libparking_lot-8880ac5dee2cb44d.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libparking_lot_core-f1cc01eb058e7bab.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblock_api-ac80298387984f21.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libscopeguard-6d1e9656911335d4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libinstant-3b9076478803cc82.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libena-f0d464b217da04d4.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblog-a36c90ab5a6fba53.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libstable_deref_trait-c77c4e43a2cff100.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_graphviz-256024e6a3a47524.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libjobserver-8becbecc8a1b0f7f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_hash-75720f896e2e6c59.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_index-8b423584b544435f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_serialize-dcc223cd5a2b16c3.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libindexmap-e837ed88701b1f9f.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libhashbrown-72d9a153a8fcbe7c.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libahash-3ae843d9820538a0.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libgetrandom-d298b6cb5dd0d33c.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblibc-32c49185c4266032.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libonce_cell-1a2d4088d02f3784.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libsmallvec-8dcbcffe8e78d682.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libarrayvec-f4ea7dc049d77c08.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcfg_if-6f9118be64647d7a.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libbitflags-406952cbce57dc87.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtracing-d78241bc3bc1fe69.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libpin_project_lite-fbcd52073ca4c42c.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libtracing_core-b1d87e1db6e0b496.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/liblazy_static-3622f0af1c9961ff.rlib" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/libcfg_if-b46da79c1442cae4.rlib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib" "-lstd-9c74d5b073066f5b" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-06e3704b523ea45b.rlib" "-lm" "-lz" "-llibzstd.1.5.2.dylib" "-lc++" "-liconv" "-lSystem" "-lresolv" "-lc" "-lm" "-liconv" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-rustc/x86_64-apple-darwin/release/deps/librustc_driver-0324347d7c661795.dylib" "-Wl,-dead_strip" "-dynamiclib" "-Wl,-dylib" "-Wl,-install_name,@rpath/librustc_driver-0324347d7c661795.dylib" "-nodefaultlibs" "-Wl,-rpath,@loader_path/../lib"
  = note: ld: library not found for -llibzstd.1.5.2.dylib
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

[RUSTC-TIMING] rustc_driver test:false 27.274
error: could not compile `rustc_driver` due to previous error
Build completed unsuccessfully in 1:43:29
