plain
[00:14:06] running: "cmake" "/checkout/src/llvm" "-DCMAKE_SYSTEM_NAME=Windows" "-DCMAKE_RC_COMPILER=/usr/bin/i686-w64-mingw32-windres" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-pc-windows-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_BUILD_32_BITS=ON" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=i686-w64-mingw32-gcc" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=i686-w64-mingw32-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/i686-pc-windows-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
---
[00:14:16] -- Looking for setenv - not found
[00:14:16] -- Looking for _chsize_s
[00:14:16] -- Looking for _chsize_s - found
[00:14:16] -- Looking for _alloca
[00:14:16] -- Looking for _alloca - found
[00:14:16] -- Looking for __alloca
[00:14:16] -- Looking for __alloca - not found
[00:14:16] -- Looking for __chkstk
[00:14:16] -- Looking for __chkstk - found
[00:14:16] -- Looking for __chkstk_ms
[00:14:17] -- Looking for __chkstk_ms - found
[00:14:17] -- Looking for ___chkstk
[00:14:17] -- Looking for ___chkstk - not found
[00:14:17] -- Looking for ___chkstk_ms
[00:14:17] -- Looking for ___chkstk_ms - not found
[00:14:17] -- Looking for __ashldi3
[00:14:17] -- Looking for __ashldi3 - found
[00:14:17] -- Looking for __ashrdi3
[00:14:17] -- Looking for __ashrdi3 - found
[00:14:17] -- Looking for __divdi3
[00:14:18] -- Looking for __divdi3 - found
[00:14:18] -- Looking for __fixdfdi
[00:14:18] -- Looking for __fixdfdi - found
[00:14:18] -- Looking for __fixsfdi
[00:14:18] -- Looking for __fixsfdi - found
[00:14:18] -- Looking for __floatdidf
[00:14:18] -- Looking for __floatdidf - found
[00:14:18] -- Looking for __lshrdi3
[00:14:18] -- Looking for __lshrdi3 - found
[00:14:18] -- Looking for __moddi3
[00:14:19] -- Looking for __moddi3 - found
[00:14:19] -- Looking for __udivdi3
[00:14:19] -- Looking for __udivdi3 - found
[00:14:19] -- Looking for __umoddi3
[00:14:19] -- Looking for __umoddi3 - found
[00:14:19] -- Looking for __main
[00:14:19] -- Looking for __main - found
[00:14:19] -- Looking for __cmpdi2
[00:14:19] -- Looking for __cmpdi2 - found
---
[00:14:23] -- LLVMHello ignored -- Loadable modules not supported on this platform.
---
[00:14:24] -- BugpointPasses ignored -- Loadable modules not supported on this platform.
---
[00:15:49] [  2%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/AddressSanitizer.cpp.obj
[00:15:51] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.obj
[00:15:52] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DeltaAlgorithm.cpp.obj
[00:15:55] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/DAGDeltaAlgorithm.cpp.obj
[00:15:55] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BasicBlockUtils.cpp.obj
[00:15:59] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Error.cpp.obj
[00:16:02] [  3%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:60:13: error: 'mutex' in namespace 'std' does not name a type
[00:16:05]  static std::mutex ErrorHandlerMutex;
[00:16:05]              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:61:13: error: 'mutex' in namespace 'std' does not name a type
[00:16:05]  static std::mutex BadAllocErrorHandlerMutex;
[00:16:05]              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_fatal_error_handler(llvm::fatal_error_handler_t, void*)':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:19: error: 'mutex' is not a member of 'std'
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                    ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:19: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:29: error: template argument 1 is invalid
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:36: error: 'ErrorHandlerMutex' was not declared in this scope
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                     ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:67:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_fatal_error_handler()':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:19: error: 'mutex' is not a member of 'std'
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                    ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:19: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:29: error: template argument 1 is invalid
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:36: error: 'ErrorHandlerMutex' was not declared in this scope
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                     ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:76:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]    std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_fatal_error(const llvm::Twine&, bool)':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:21: error: 'mutex' is not a member of 'std'
[00:16:05]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                      ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:21: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:31: error: template argument 1 is invalid
[00:16:05]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:38: error: 'ErrorHandlerMutex' was not declared in this scope
[00:16:05]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                       ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:101:33: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]      std::lock_guard<std::mutex> Lock(ErrorHandlerMutex);
[00:16:05]                                  ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::install_bad_alloc_error_handler(llvm::fatal_error_handler_t, void*)':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:19: error: 'mutex' is not a member of 'std'
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                    ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:19: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:29: error: template argument 1 is invalid
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                     ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:132:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::remove_bad_alloc_error_handler()':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:19: error: 'mutex' is not a member of 'std'
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                    ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:19: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:29: error: template argument 1 is invalid
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                              ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:36: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                     ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:141:31: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]    std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp: In function 'void llvm::report_bad_alloc_error(const char*, bool)':
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:21: error: 'mutex' is not a member of 'std'
[00:16:05]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                      ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:21: error: 'mutex' is not a member of 'std'
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:31: error: template argument 1 is invalid
[00:16:05]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:38: error: 'BadAllocErrorHandlerMutex' was not declared in this scope
[00:16:05]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                       ^
[00:16:05] /checkout/src/llvm/lib/Support/ErrorHandling.cpp:154:33: warning: unused variable 'Lock' [-Wunused-variable]
[00:16:05]      std::lock_guard<std::mutex> Lock(BadAllocErrorHandlerMutex);
[00:16:05]                                  ^
[00:16:05] lib/Support/CMakeFiles/LLVMSupport.dir/build.make:813: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj' failed
[00:16:05] make[2]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/ErrorHandling.cpp.obj] Error 1
[00:16:05] CMakeFiles/Makefile2:528: recipe for target 'lib/Support/CMakeFiles/LLVMSupport.dir/all' failed
[00:16:05] make[1]: *** [lib/Support/CMakeFiles/LLVMSupport.dir/all] Error 2
[00:16:05] make[1]: *** Waiting for unfinished jobs....
[00:16:05] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BreakCriticalEdges.cpp.obj
[00:16:05] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/BoundsChecking.cpp.obj
[00:16:06] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BuildLibCalls.cpp.obj
[00:16:08] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenSchedule.cpp.obj
[00:16:13] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/DataFlowSanitizer.cpp.obj
[00:16:13] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenTarget.cpp.obj
[00:16:14] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/BypassSlowDivision.cpp.obj
[00:16:20] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelEmitter.cpp.obj
[00:16:22] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CallPromotionUtils.cpp.obj
[00:16:24] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherEmitter.cpp.obj
[00:16:27] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherGen.cpp.obj
[00:16:27] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/GCOVProfiling.cpp.obj
[00:16:28] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneFunction.cpp.obj
[00:16:34] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherOpt.cpp.obj
[00:16:35] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcher.cpp.obj
[00:16:39] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/MemorySanitizer.cpp.obj
[00:16:40] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CloneModule.cpp.obj
[00:16:40] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DFAPacketizerEmitter.cpp.obj
[00:16:41] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DisassemblerEmitter.cpp.obj
[00:16:46] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/FastISelEmitter.cpp.obj
[00:16:47] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CodeExtractor.cpp.obj
[00:16:48] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/FixedLenDecoderEmitter.cpp.obj
[00:16:53] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/IndirectCallPromotion.cpp.obj
[00:16:58] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/GlobalISelEmitter.cpp.obj
[00:16:58] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/CtorUtils.cpp.obj
[00:17:00] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InfoByHwMode.cpp.obj
[00:17:03] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/DemoteRegToStack.cpp.obj
[00:17:04] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/Instrumentation.cpp.obj
[00:17:05] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InstrInfoEmitter.cpp.obj
[00:17:08] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/InstrProfiling.cpp.obj
[00:17:10] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EntryExitInstrumenter.cpp.obj
[00:17:16] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/InstrDocsEmitter.cpp.obj
[00:17:17] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/EscapeEnumerator.cpp.obj
[00:17:20] [  3%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/PGOInstrumentation.cpp.obj
[00:17:22] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/IntrinsicEmitter.cpp.obj
[00:17:23] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/OptParserEmitter.cpp.obj
[00:17:23] [  3%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/Evaluator.cpp.obj
[00:17:27] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/PseudoLoweringEmitter.cpp.obj
[00:17:30] [  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterBankEmitter.cpp.obj
[00:17:32] [  4%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FlattenCFG.cpp.obj
[00:17:34] [  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/RegisterInfoEmitter.cpp.obj
[00:17:36] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SDNodeProperties.cpp.obj
[00:17:39] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/PGOMemOPSizeOpt.cpp.obj
[00:17:39] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SearchableTableEmitter.cpp.obj
[00:17:39] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FunctionComparator.cpp.obj
[00:17:45] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetEmitter.cpp.obj
[00:17:47] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/FunctionImportUtils.cpp.obj
[00:17:49] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/SubtargetFeatureInfo.cpp.obj
[00:17:49] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/SanitizerCoverage.cpp.obj
[00:17:53] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/TableGen.cpp.obj
[00:17:53] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/GlobalStatus.cpp.obj
[00:17:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/Types.cpp.obj
[00:17:56] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/ThreadSanitizer.cpp.obj
[00:17:56] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86DisassemblerTables.cpp.obj
[00:17:58] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InlineFunction.cpp.obj
[00:18:00] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/ImportedFunctionsInliningStatistics.cpp.obj
[00:18:01] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86EVEX2VEXTablesEmitter.cpp.obj
[00:18:05] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/InstructionNamer.cpp.obj
[00:18:07] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/EfficiencySanitizer.cpp.obj
[00:18:07] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86FoldTablesEmitter.cpp.obj
[00:18:09] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/IntegerDivision.cpp.obj
[00:18:14] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86ModRMFilters.cpp.obj
[00:18:14] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.obj
[00:18:15] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LCSSA.cpp.obj
[00:18:16] [  5%] Building CXX object lib/Transforms/Utils/CMakeFiles/LLVMTransformUtils.dir/LibCallsShrinkWrap.cpp.obj
[00:18:16] [  5%] Building CXX object lib/Transforms/Instrumentation/CMakeFiles/LLVMInstrumentation.dir/HWAddressSanitizer.cpp.obj
[00:18:19] [  5%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.obj
---
[00:19:33] Makefile:149: recipe for target 'all' failed
[00:19:33] make: *** [all] Error 2
[00:19:33] thread 'main' panicked at '
[00:19:33] command did not execute successfully, got: exit code: 2
[00:19:33]
[00:19:33] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.29/src/lib.rs:632:5
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:00a2154f:start=1523993145084671372,finish=1523993145104387349,duration=19715977
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:23db394a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:23db394a:start=1523993145114801480,finish=1523993145123102653,duration=8301173
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01627ef0
$ dmesg | grep -i kill
[   10.514748] init: failsafe main process (1095) killed by TERM signal
