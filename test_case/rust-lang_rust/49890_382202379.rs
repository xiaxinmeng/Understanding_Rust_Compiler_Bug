plain
Resolving deltas: 100% (617003/617003), completed with 4910 local objects.
---
[00:01:01] configure: rust.quiet-tests     := True
---
[00:01:01] configure: build.configure-args := ['--enable-quiet-tests', '--enable-sccache', ' ...
---
[00:11:26] running: "cmake" "/checkout/src/llvm" "-DCMAKE_SYSTEM_NAME=Windows" "-DCMAKE_RC_COMPILER=/usr/bin/i686-w64-mingw32-windres" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-pc-windows-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_BUILD_32_BITS=ON" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=i686-w64-mingw32-gcc" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=i686-w64-mingw32-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/i686-pc-windows-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
---
[00:11:36] -- Looking for setenv - not found
[00:11:36] -- Looking for _chsize_s
[00:11:36] -- Looking for _chsize_s - found
[00:11:36] -- Looking for _alloca
[00:11:36] -- Looking for _alloca - found
[00:11:36] -- Looking for __alloca
[00:11:36] -- Looking for __alloca - not found
[00:11:36] -- Looking for __chkstk
[00:11:36] -- Looking for __chkstk - found
[00:11:36] -- Looking for __chkstk_ms
[00:11:36] -- Looking for __chkstk_ms - found
[00:11:36] -- Looking for ___chkstk
[00:11:37] -- Looking for ___chkstk - not found
[00:11:37] -- Looking for ___chkstk_ms
[00:11:37] -- Looking for ___chkstk_ms - not found
[00:11:37] -- Looking for __ashldi3
[00:11:37] -- Looking for __ashldi3 - found
[00:11:37] -- Looking for __ashrdi3
[00:11:37] -- Looking for __ashrdi3 - found
[00:11:37] -- Looking for __divdi3
[00:11:37] -- Looking for __divdi3 - found
[00:11:37] -- Looking for __fixdfdi
[00:11:37] -- Looking for __fixdfdi - found
[00:11:37] -- Looking for __fixsfdi
[00:11:38] -- Looking for __fixsfdi - found
[00:11:38] -- Looking for __floatdidf
[00:11:38] -- Looking for __floatdidf - found
[00:11:38] -- Looking for __lshrdi3
[00:11:38] -- Looking for __lshrdi3 - found
[00:11:38] -- Looking for __moddi3
[00:11:38] -- Looking for __moddi3 - found
[00:11:38] -- Looking for __udivdi3
[00:11:38] -- Looking for __udivdi3 - found
[00:11:38] -- Looking for __umoddi3
[00:11:38] -- Looking for __umoddi3 - found
[00:11:38] -- Looking for __main
[00:11:39] -- Looking for __main - found
[00:11:39] -- Looking for __cmpdi2
[00:11:39] -- Looking for __cmpdi2 - found
---
[00:11:42] -- LLVMHello ignored -- Loadable modules not supported on this platform.
---
[00:11:43] -- BugpointPasses ignored -- Loadable modules not supported on this platform.
---
[00:11:51] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/AddressSanitizer.cpp.obj
[00:11:51] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/GlobalISelEmitter.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DataExtractor.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BypassSlowDivision.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Debug.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/BoundsChecking.cpp.obj
[00:11:51] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InfoByHwMode.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CallPromotionUtils.cpp.obj
[00:11:51] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DebugCounter.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/DataFlowSanitizer.cpp.obj
[00:11:52] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InstrInfoEmitter.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DeltaAlgorithm.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneFunction.cpp.obj
[00:11:52] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InstrDocsEmitter.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneModule.cpp.obj
[00:11:52] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/IntrinsicEmitter.cpp.obj
[00:11:52] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CodeExtractor.cpp.obj
[00:11:53] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/OptParserEmitter.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/GCOVProfiling.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CtorUtils.cpp.obj
[00:11:53] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/MemorySanitizer.cpp.obj
[00:11:53] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterBankEmitter.cpp.obj
[00:11:53] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/DemoteRegToStack.cpp.obj
[00:11:54] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterInfoEmitter.cpp.obj
[00:11:54] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/IndirectCallPromotion.cpp.obj
[00:11:54] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EntryExitInstrumenter.cpp.obj
[00:11:54] [  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SDNodeProperties.cpp.obj
[00:11:54] [  4%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/Instrumentation.cpp.obj
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:60:13: error: 'mutex' in namespace 'std' does not name a type
[00:11:54]  static std::mutex ErrorHandlerMutex;
[00:11:54]              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:61:13: error: 'mutex' in namespace 'std' does not name a type
[00:11:54]  static std::mutex BadAllocErrorHandlerMutex;
[00:11:54]              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_fatal_error_handler(llvm::fatal_error_handler_t, void*)':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:19: error: 'mutex' is not a member of 'std'
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                    ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:19: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:29: error: template argument 1 is invalid
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:36: error: 'ErrorHandlerMutex' was not declared in this scope
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                     ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:19: error: 'mutex' is not a member of 'std'
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                    ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:19: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:29: error: template argument 1 is invalid
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:36: error: 'ErrorHandlerMutex' was not declared in this scope
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                     ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_fatal_error(const llvm::Twine&, bool)':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:21: error: 'mutex' is not a member of 'std'
[00:11:54]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                      ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:21: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:31: error: template argument 1 is invalid
[00:11:54]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:38: error: 'ErrorHandlerMutex' was not declared in this scope
[00:11:54]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                       ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:33: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:11:54]                                  ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_bad_alloc_error_handler(llvm::fatal_error_handler_t, void*)':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:19: error: 'mutex' is not a member of 'std'
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                    ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:19: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:29: error: template argument 1 is invalid
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                     ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:19: error: 'mutex' is not a member of 'std'
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                    ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:19: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:29: error: template argument 1 is invalid
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                              ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                     ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:21: error: 'mutex' is not a member of 'std'
[00:11:54]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                      ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:21: error: 'mutex' is not a member of 'std'
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:31: error: template argument 1 is invalid
[00:11:54]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:38: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:11:54]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                       ^
[00:11:54] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:33: warning: unused variable 'Lock' [-Wunused-variable]
[00:11:54]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:11:54]                                  ^
[00:11:54] make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj] Error 1
[00:11:54] lib/Support/CMakeFiles/LLVMSupport.dir/build.make:813: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj' failed
[00:11:54] make[1]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
[00:11:54] CMakeFiles/Makefile2:528: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/all' failed
[00:11:54] make[1]: *** Waiting for unfinished jobs....
[00:11:54] [  4%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/InstrProfiling.cpp.obj
[00:11:54] [  4%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EscapeEnumerator.cpp.obj
[00:11:54] [  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SearchableTableEmitter.cpp.obj
[00:11:55] [  4%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/PGOInstrumentation.cpp.obj
[00:11:55] [  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetEmitter.cpp.obj
[00:11:55] [  4%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/PGOMemOPSizeOpt.cpp.obj
[00:11:55] [  4%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Evaluator.cpp.obj
[00:11:55] [  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetFeatureInfo.cpp.obj
[00:11:55] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FlattenCFG.cpp.obj
[00:11:55] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/SanitizerCoverage.cpp.obj
[00:11:55] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/TableGen.cpp.obj
[00:11:55] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/ThreadSanitizer.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/Types.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/EfficiencySanitizer.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FunctionComparator.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86DisassemblerTables.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86EVEX2VEXTablesEmitter.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FunctionImportUtils.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GlobalStatus.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/HWAddressSanitizer.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86FoldTablesEmitter.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.obj
[00:11:56] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InlineFunction.cpp.obj
[00:11:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.obj
---
[00:12:01] make: *** [all] Error 2
[00:12:01] Makefile:149: recipe for target 'all' failed
[00:12:01] thread 'main' panicked at '
[00:12:01] command did not execute successfully, got: exit code: 2
[00:12:01]
[00:12:01] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.29/src/lib.rs:632:5
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:07028b0c:start=1524006606705850156,finish=1524006606713666421,duration=7816265
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:01b0c8f8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:01b0c8f8:start=1524006606719534108,finish=1524006606726354534,duration=6820426
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0200cdc2
$ dmesg | grep -i kill
[    9.980607] init: failsafe main process (1093) killed by TERM signal
