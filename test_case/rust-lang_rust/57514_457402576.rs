plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:050684fa
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:25] 16.04: Pulling from library/ubuntu
[00:01:25] Digest: sha256:e4a134999bea4abb4a27bc437e6118fdddfb172e1b9d683129b74d254af51675
[00:01:25] Status: Downloaded newer image for ubuntu:16.04
[00:01:25]  ---> 7e87e2b3bf7a
[00:01:25] Step 2/8 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   python2.7-dev   libxml2-dev   libncurses-dev   libedit-dev   swig   doxygen   git   cmake   sudo   gdb   xz-utils
[00:01:25]  ---> abfcc36a520a
[00:01:25] Step 3/8 : COPY scripts/sccache.sh /scripts/
[00:01:25]  ---> Using cache
[00:01:25]  ---> d01fde94b8dc
---
[00:01:25]  ---> 281343a8e70a
[00:01:25] Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --enable-debug       --enable-lld       --enable-lldb       --enable-optimize
[00:01:25]  ---> Using cache
[00:01:25]  ---> d520972f72d6
[00:01:25] Step 8/8 : ENV SCRIPT   python2.7 ../x.py build &&   python2.7 ../x.py test src/test/run-make-fulldeps --test-args clang
[00:01:25]  ---> 41f306962997
[00:01:25] Successfully built 41f306962997
[00:01:25] Successfully tagged rust-ci:latest
[00:01:25] Built container sha256:41f306962997afb2b1160d353f148c6de6b370c94dfa973e2dbb1bd688dc63e5
---
[00:33:06] -- Looking for pthread_create in pthreads - not found
[00:33:06] -- Looking for pthread_create in pthread
[00:33:06] -- Looking for pthread_create in pthread - found
[00:33:06] -- Found Threads: TRUE  
[00:33:06] -- Found LibXml2: /usr/lib/x86_64-linux-gnu/libxml2.so (found version "2.9.3") 
[00:33:06] -- Looking for xar_open in xar - not found
[00:33:06] -- Looking for arc4random
[00:33:06] -- Looking for arc4random - not found
[00:33:06] -- Looking for backtrace
---
[00:33:11] -- Looking for sys/resource.h - found
[00:33:11] -- Clang version: 8.0.0
[00:33:11] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:33:11] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Failed
[00:33:11] -- Could NOT find Z3 (missing:  Z3_LIBRARIES Z3_INCLUDE_DIR) (Required is exact version "4.7.1")
[00:33:12] -- Found PythonLibs: /usr/lib/x86_64-linux-gnu/libpython2.7.so (found version "2.7.12") 
[00:33:12] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS - Success
[00:33:12] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS
[00:33:12] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS - Success
[00:33:12] -- Performing Test CXX_SUPPORTS_NO_STRICT_ALIASING
---
[00:33:12] -- LLDB version: 8.0.0
[00:33:12] -- Found Curses: /usr/lib/x86_64-linux-gnu/libcurses.so  
[00:33:12] -- Looking for __GLIBCXX__
[00:33:12] -- Looking for __GLIBCXX__ - found
[00:33:12] -- Performing Test LLDB_USING_LIBSTDCXX_4_9
[00:33:12] -- Performing Test LLDB_USING_LIBSTDCXX_4_9 - Success
[00:33:13] -- Looking for ppoll - found
[00:33:13] -- Looking for sigaction
[00:33:13] -- Looking for sigaction - found
[00:33:13] -- Looking for accept4
---
[00:33:49] [ 13%] Built target hmaptool
[00:33:49] Scanning dependencies of target readline
[00:33:49] [ 13%] Generating ../../../../share/scan-view/GetRadarVersion.scpt
[00:33:49] [ 13%] Building CXX object tools/clang/utils/TableGen/CMakeFiles/obj.clang-tblgen.dir/ClangCommentHTMLNamedCharacterReferenceEmitter.cpp.o
[00:33:50] [ 13%] Building CXX object tools/lldb/scripts/Python/modules/readline/CMakeFiles/readline.dir/readline.cpp.o
[00:33:50] [ 13%] Built target scan-view
[00:33:50] Scanning dependencies of target swig_wrapper
[00:33:50] [ 13%] Python script building LLDB Python wrapper
[00:33:50] [ 13%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/PDBSymbolTypeVTableShape.cpp.o
---
[00:38:55] [ 76%] Building CXX object tools/lldb/source/Plugins/Language/ObjC/CMakeFiles/lldbPluginObjCLanguage.dir/NSArray.cpp.o
[00:38:55] [ 76%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibCxxInitializerList.cpp.o
[00:38:55] [ 76%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/Property.cpp.o
[00:38:55] [ 76%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/ValistChecker.cpp.o
[00:38:55] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSArray.cpp:184:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:55]        ^
[00:38:55]        ^
[00:38:55] cc1plus: warning: unrecognized command line option '-Wno-nested-anon-types'
[00:38:55] cc1plus: warning: unrecognized command line option '-Wno-gnu-anonymous-struct'
[00:38:55] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:38:55] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:38:55] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibCxxList.cpp.o
[00:38:55] [ 77%] Building CXX object tools/lldb/source/Interpreter/CMakeFiles/lldbInterpreter.dir/ScriptInterpreter.cpp.o
[00:38:55] [ 77%] Building CXX object tools/clang/lib/StaticAnalyzer/Checkers/CMakeFiles/clangStaticAnalyzerCheckers.dir/VirtualCallChecker.cpp.o
[00:38:56] [ 77%] Linking CXX static library ../../../../lib/liblldbInterpreter.a
[00:38:56] [ 77%] Linking CXX static library ../../../../lib/liblldbInterpreter.a
[00:38:56] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibCxxMap.cpp.o
[00:38:56] [ 77%] Built target lldbInterpreter
[00:38:56] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSDictionary.cpp:292:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:56]        ^
[00:38:56]        ^
[00:38:56] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSDictionary.cpp:298:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:56]        ^
[00:38:56]        ^
[00:38:56] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSDictionary.cpp:312:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:56]        ^
[00:38:56]        ^
[00:38:56] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSDictionary.cpp:318:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:56]        ^
[00:38:56]        ^
[00:38:56] cc1plus: warning: unrecognized command line option '-Wno-nested-anon-types'
[00:38:56] cc1plus: warning: unrecognized command line option '-Wno-gnu-anonymous-struct'
[00:38:56] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:38:56] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:38:56] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/ObjC/CMakeFiles/lldbPluginObjCLanguage.dir/NSError.cpp.o
[00:38:56] [ 77%] Linking CXX static library ../../../../../lib/libclangStaticAnalyzerCheckers.a
[00:38:56] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibCxxQueue.cpp.o
[00:38:56] Scanning dependencies of target lldbPluginAppleObjCRuntime
---
[00:38:57] [ 77%] Building CXX object tools/lldb/source/Plugins/LanguageRuntime/ObjC/AppleObjCRuntime/CMakeFiles/lldbPluginAppleObjCRuntime.dir/AppleObjCRuntimeV2.cpp.o
[00:38:57] /checkout/src/tools/lldb/source/Plugins/Language/CPlusPlus/LibCxxVariant.cpp:114:2: warning: extra ';' [-Wpedantic]
[00:38:57]  };
[00:38:57]   ^
[00:38:57] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:38:57] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:38:57] Scanning dependencies of target lldbPluginLanguageRuntimeRust
[00:38:57] [ 77%] Building CXX object tools/lldb/source/Plugins/LanguageRuntime/Rust/CMakeFiles/lldbPluginLanguageRuntimeRust.dir/RustLanguageRuntime.cpp.o
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/ObjC/CMakeFiles/lldbPluginObjCLanguage.dir/NSSet.cpp.o
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibStdcpp.cpp.o
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibStdcpp.cpp.o
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/LanguageRuntime/ObjC/AppleObjCRuntime/CMakeFiles/lldbPluginAppleObjCRuntime.dir/AppleObjCTrampolineHandler.cpp.o
[00:38:58] [ 77%] Linking CXX static library ../../../../../../lib/liblldbPluginLanguageRuntimeRust.a
[00:38:58] [ 77%] Built target lldbPluginLanguageRuntimeRust
[00:38:58] Scanning dependencies of target lldbPluginObjectContainerBSDArchive
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/ObjectContainer/BSD-Archive/CMakeFiles/lldbPluginObjectContainerBSDArchive.dir/ObjectContainerBSDArchive.cpp.o
[00:38:58] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSSet.cpp:168:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:58]        ^
[00:38:58]        ^
[00:38:58] /checkout/src/tools/lldb/source/Plugins/Language/ObjC/NSSet.cpp:182:7: warning: ISO C++ prohibits anonymous structs [-Wpedantic]
[00:38:58]        ^
[00:38:58]        ^
[00:38:58] cc1plus: warning: unrecognized command line option '-Wno-nested-anon-types'
[00:38:58] cc1plus: warning: unrecognized command line option '-Wno-gnu-anonymous-struct'
[00:38:58] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:38:58] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/Language/CPlusPlus/CMakeFiles/lldbPluginCPlusPlusLanguage.dir/LibStdcppTuple.cpp.o
[00:38:58] [ 77%] Building CXX object tools/lldb/source/Plugins/LanguageRuntime/ObjC/AppleObjCRuntime/CMakeFiles/lldbPluginAppleObjCRuntime.dir/AppleObjCDeclVendor.cpp.o
[00:38:58] [ 77%] Linking CXX static library ../../../../../../lib/liblldbPluginObjectContainerBSDArchive.a
[00:38:59] [ 77%] Built target lldbPluginObjectContainerBSDArchive
---
[00:39:02] [ 78%] Building CXX object tools/lldb/source/Plugins/Process/Linux/CMakeFiles/lldbPluginProcessLinux.dir/ProcessorTrace.cpp.o
[00:39:02] [ 78%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/DynamicRegisterInfo.cpp.o
[00:39:02] [ 79%] Building CXX object tools/lldb/source/Plugins/Platform/MacOSX/CMakeFiles/lldbPluginPlatformMacOSX.dir/PlatformRemoteDarwinDevice.cpp.o
[00:39:02] [ 79%] Building CXX object tools/lldb/source/Plugins/Process/gdb-remote/CMakeFiles/lldbPluginProcessGDBRemote.dir/GDBRemoteCommunicationClient.cpp.o
[00:39:02] [ 79%] Building CXX object tools/lldb/source/Plugins/Process/Linux/CMakeFiles/lldbPluginProcessLinux.dir/SingleStepCheck.cpp.o
[00:39:02] [ 79%] Building CXX object tools/lldb/source/Plugins/Platform/MacOSX/CMakeFiles/lldbPluginPlatformMacOSX.dir/PlatformRemoteAppleBridge.cpp.o
[00:39:03] [ 79%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/GDBRemoteSignals.cpp.o
[00:39:03] [ 79%] Building CXX object tools/lldb/source/Plugins/Process/gdb-remote/CMakeFiles/lldbPluginProcessGDBRemote.dir/GDBRemoteCommunicationServer.cpp.o
[00:39:03] [ 79%] Linking CXX static library ../../../../../../lib/liblldbPluginProcessLinux.a
---
[00:39:05] [ 80%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/NativeRegisterContextRegisterInfo.cpp.o
[00:39:05] [ 80%] Building CXX object tools/lldb/source/Plugins/SymbolFile/NativePDB/CMakeFiles/lldbPluginSymbolFileNativePDB.dir/UdtRecordCompleter.cpp.o
[00:39:05] [ 80%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/NetBSDSignals.cpp.o
[00:39:05] [ 80%] Building CXX object tools/lldb/source/Plugins/Process/gdb-remote/CMakeFiles/lldbPluginProcessGDBRemote.dir/ProcessGDBRemoteLog.cpp.o
[00:39:05] /checkout/src/tools/lldb/source/Plugins/SymbolFile/DWARF/DWARFASTParserClang.cpp: In member function 'virtual lldb::TypeSP DWARFASTParserClang::ParseTypeFromDWARF(const lldb_private::SymbolContext&, const DWARFDIE&, lldb_private::Log*, bool*)':
[00:39:05] /checkout/src/tools/lldb/source/Plugins/SymbolFile/DWARF/DWARFASTParserClang.cpp:1669:17: warning: variable 'first_index' set but not used [-Wunused-but-set-variable]
[00:39:05]          int64_t first_index = 0;
[00:39:05] At global scope:
[00:39:05] At global scope:
[00:39:05] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:39:05] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:39:05] [ 80%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextDarwin_arm.cpp.o
[00:39:06] [ 80%] Linking CXX static library ../../../../../../lib/liblldbPluginSymbolFileNativePDB.a
[00:39:06] [ 80%] Building CXX object tools/lldb/source/Plugins/Process/gdb-remote/CMakeFiles/lldbPluginProcessGDBRemote.dir/ThreadGDBRemote.cpp.o
[00:39:06] [ 80%] Built target lldbPluginSymbolFileNativePDB
---
[00:39:06] [ 80%] Built target lldbPluginProcessGDBRemote
[00:39:06] [ 80%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDataExtractor.cpp.o
[00:39:06] Scanning dependencies of target lldbPluginSymbolFilePDB
[00:39:06] [ 81%] Building CXX object tools/lldb/source/Plugins/SymbolFile/PDB/CMakeFiles/lldbPluginSymbolFilePDB.dir/PDBASTParser.cpp.o
[00:39:06] /checkout/src/tools/lldb/source/Plugins/Process/Utility/RegisterContextDarwin_arm64.cpp: In member function 'virtual bool RegisterContextDarwin_arm64::ReadRegister(const lldb_private::RegisterInfo*, lldb_private::RegisterValue&)':
[00:39:06] /checkout/src/tools/lldb/source/Plugins/Process/Utility/RegisterContextDarwin_arm64.cpp:425:47: warning: array subscript is above array bounds [-Warray-bounds]
[00:39:06]                     endian::InlHostByteOrder());
[00:39:06] At global scope:
[00:39:06] At global scope:
[00:39:06] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:39:06] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:39:06] [ 81%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextDarwin_x86_64.cpp.o
[00:39:07] [ 81%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugAbbrev.cpp.o
[00:39:07] [ 81%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextDummy.cpp.o
[00:39:07] [ 81%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextFreeBSD_i386.cpp.o
---
[00:39:09] [ 81%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextLinux_mips.cpp.o
[00:39:09] [ 81%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDebugRanges.cpp.o
[00:39:09] [ 81%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDeclContext.cpp.o
[00:39:09] [ 82%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextLinux_s390x.cpp.o
[00:39:09] /checkout/src/tools/lldb/source/Symbol/ClangASTContext.cpp: In function 'llvm::Optional<lldb_private::SymbolFile::ArrayInfo> GetDynamicArrayInfo(lldb_private::ClangASTContext&, lldb_private::SymbolFile*, clang::QualType, const lldb_private::ExecutionContext*)':
[00:39:09] /checkout/src/tools/lldb/source/Symbol/ClangASTContext.cpp:5421:17: warning: unused variable 'dwarf_parser' [-Wunused-variable]
[00:39:09]        if (auto *dwarf_parser = ast.GetDWARFParser())
[00:39:09] At global scope:
[00:39:09] At global scope:
[00:39:09] cc1plus: warning: unrecognized command line option '-Wno-vla-extension'
[00:39:09] cc1plus: warning: unrecognized command line option '-Wno-deprecated-register'
[00:39:10] [ 82%] Building CXX object tools/lldb/source/Symbol/CMakeFiles/lldbSymbol.dir/ClangExternalASTSourceCallbacks.cpp.o
[00:39:10] [ 82%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDefines.cpp.o
[00:39:10] [ 82%] Building CXX object tools/lldb/source/Plugins/Process/Utility/CMakeFiles/lldbPluginProcessUtility.dir/RegisterContextLLDB.cpp.o
[00:39:10] [ 82%] Building CXX object tools/lldb/source/Plugins/SymbolFile/DWARF/CMakeFiles/lldbPluginSymbolFileDWARF.dir/DWARFDIE.cpp.o
---
[00:39:38] [ 86%] Building CXX object tools/lldb/source/Plugins/UnwindAssembly/x86/CMakeFiles/lldbPluginUnwindAssemblyX86.dir/x86AssemblyInspectionEngine.cpp.o
[00:39:38] [ 86%] Linking CXX static library ../../../../../../lib/liblldbPluginUnwindAssemblyInstEmulation.a
[00:39:38] [ 86%] Built target lldbPluginUnwindAssemblyInstEmulation
[00:39:38] [ 86%] Building CXX object tools/lldb/source/Plugins/SystemRuntime/MacOSX/CMakeFiles/lldbPluginSystemRuntimeMacOSX.dir/AppleGetQueuesHandler.cpp.o
[00:39:38] Scanning dependencies of target lldbIntelMPX
[00:39:38] [ 86%] Building CXX object tools/lldb/tools/intel-features/intel-mpx/CMakeFiles/lldbIntelMPX.dir/cli-wrapper-mpxtable.cpp.o
[00:39:38] Scanning dependencies of target LLVMExegesisAArch64
[00:39:38] [ 86%] Building CXX object tools/llvm-exegesis/lib/AArch64/CMakeFiles/LLVMExegesisAArch64.dir/Target.cpp.o
[00:39:38] [ 86%] Built target lldbPluginUnwindAssemblyX86
[00:39:38] Scanning dependencies of target LLVMExegesisPowerPC
[00:39:38] Scanning dependencies of target LLVMExegesisPowerPC
[00:39:38] [ 86%] Building CXX object tools/llvm-exegesis/lib/PowerPC/CMakeFiles/LLVMExegesisPowerPC.dir/Target.cpp.o
[00:39:38] [ 86%] Linking CXX static library ../../../../../lib/liblldbIntelMPX.a
[00:39:38] [ 86%] Built target lldbIntelMPX
[00:39:38] [ 86%] Building CXX object tools/lli/ChildTarget/CMakeFiles/lli-child-target.dir/ChildTarget.cpp.o
[00:39:38] [ 86%] Building CXX object tools/lldb/source/Plugins/SystemRuntime/MacOSX/CMakeFiles/lldbPluginSystemRuntimeMacOSX.dir/AppleGetThreadItemInfoHandler.cpp.o
[00:39:39] [ 86%] Linking CXX static library ../../../../lib/libLLVMExegesisAArch64.a
[00:39:39] [ 86%] Linking CXX static library ../../../../lib/libLLVMExegesisPowerPC.a
---
[00:41:03] [ 98%] Building CXX object tools/lldb/tools/lldb-mi/CMakeFiles/lldb-mi.dir/MICmdCmdStack.cpp.o
[00:41:04] [ 98%] Building CXX object tools/lldb/tools/lldb-mi/CMakeFiles/lldb-mi.dir/MICmdCmdSupportInfo.cpp.o
[00:41:04] [ 98%] Building CXX object tools/lldb/tools/lldb-mi/CMakeFiles/lldb-mi.dir/MICmdCmdSupportList.cpp.o
[00:41:04] [ 98%] Building CXX object tools/lldb/tools/lldb-mi/CMakeFiles/lldb-mi.dir/MICmdCmdSymbol.cpp.o
[00:41:04] Scanning dependencies of target lldbIntelFeatures
[00:41:04] [ 98%] Building CXX object tools/lldb/tools/intel-features/CMakeFiles/lldbIntelFeatures.dir/cli-wrapper.cpp.o
[00:41:04] Scanning dependencies of target llvm-strip
[00:41:04] [ 98%] Generating ../../bin/llvm-strip
[00:41:04] [ 98%] Built target llvm-strip
[00:41:04] Scanning dependencies of target llvm-readelf
---
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/lldb
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/lldb-mi
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/lldb-vscode
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/lldb-server
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/liblldbIntelFeatures.so.8svn
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/liblldbIntelFeatures.so
[00:41:11] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/liblldbIntelFeatures.so.8svn
[00:41:11] -- Up-to-date: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/liblldbIntelFeatures.so
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/liblldbIntelMPX.a
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-size
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cvtres
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-cfi-verify
[00:41:11] -- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/obj2yaml
---
[01:37:28] status: exit code: 2
[01:37:28] command: "make"
[01:37:28] stdout:
[01:37:28] ------------------------------------------
[01:37:28] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang  -Zcross-lang-lto=on -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang/librustlib-xlto.a -Copt-level=2 -Ccodegen-units=1 ./rustlib.rs
[01:37:28] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang -flto=thin -fuse-ld=lld -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang -lrustlib-xlto -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto-clang/cross-lang-lto-clang/cmain ./cmain.c -O3
[01:37:28] Makefile:11: recipe for target 'cpp-executable' failed
[01:37:28] ------------------------------------------
[01:37:28] stderr:
[01:37:28] ------------------------------------------
[01:37:28] ------------------------------------------
[01:37:28] warning: ignoring --out-dir flag due to -o flag
[01:37:28] 
[01:37:28] clang-8: error: invalid linker name in argument '-fuse-ld=lld'
[01:37:28] make: *** [cpp-executable] Error 1
[01:37:28] ------------------------------------------
[01:37:28] 
[01:37:28] thread '[run-make] run-make-fulldeps/cross-lang-lto-clang' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3249:9
[01:37:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:37:28] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 193 filtered out
[01:37:28] 
[01:37:28] 
[01:37:28] 
[01:37:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--lldb-version" "lldb version 8.0.0\n  rust-enabled\n" "--lldb-python-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/python2.7/site-packages" "--run-clang-based-tests-with" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/clang" "clang" "--quiet" "--llvm-version" "8.0.0svn\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm/include -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O3   -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:37:28] 
[01:37:28] 
[01:37:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-make-fulldeps --test-args clang
[01:37:28] Build completed unsuccessfully in 0:01:32
---
travis_time:end:064f46c8:start=1548372469875863092,finish=1548372469895237310,duration=19374218
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b5fb96f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2fed77d4
travis_time:start:2fed77d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:085d4cab
$ dmesg | grep -i kill
