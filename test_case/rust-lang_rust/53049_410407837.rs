plain
[00:08:10] [RUSTC-TIMING] proc_macro test:false 8.872
[00:08:10]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:29] [RUSTC-TIMING] syntax_ext test:false 19.233
[00:13:11] [RUSTC-TIMING] rustc test:false 300.798
[00:13:11]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:11]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:12] [RUSTC-TIMING] rustc_metadata_utils test:false 0.758
[00:13:12]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:13:42] [RUSTC-TIMING] rustc_allocator test:false 30.579
---
[00:18:35] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:18:35] -- Performing Test C_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Success
[00:18:35] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG
[00:18:35] -- Performing Test CXX_SUPPORTS_COVERED_SWITCH_DEFAULT_FLAG - Success
[00:18:35] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG
[00:18:35] -- Performing Test CXX_SUPPORTS_CLASS_MEMACCESS_FLAG - Failed
[00:18:35] -- Performing Test CXX_WONT_WARN_ON_FINAL_NONVIRTUALDTOR - Success
[00:18:35] -- Performing Test C_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG
[00:18:36] -- Performing Test C_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG - Success
[00:18:36] -- Performing Test CXX_SUPPORTS_DELETE_NON_VIRTUAL_DTOR_FLAG
---
[00:23:12] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeBuiltinSymbol.cpp.o
[00:23:13] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
[00:23:13] [ 15%] Linking CXX executable ../../bin/llvm-cxxfilt
[00:23:13] [ 15%] Built target llvm-cxxfilt
[00:23:13] Scanning dependencies of target llvm-undname
[00:23:13] [ 15%] Building CXX object tools/llvm-undname/CMakeFiles/llvm-undname.dir/llvm-undname.cpp.o
[00:23:14] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumSymbol.cpp.o
[00:23:15] [ 15%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumTypes.cpp.o
[00:23:15] [ 15%] Linking CXX executable ../../bin/llvm-undname
[00:23:15] [ 15%] Built target llvm-undname
---
[00:43:59] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86RetpolineThunks.cpp.o
[00:44:02] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SelectionDAGInfo.cpp.o
[00:44:03] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86ShuffleDecodeConstantPool.cpp.o
[00:44:05] [ 69%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMCallLowering.cpp.o
[00:44:05] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86SpeculativeLoadHardening.cpp.o
[00:44:08] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetMachine.cpp.o
[00:44:11] [ 69%] Building CXX object lib/Target/ARM/CMakeFiles/LLVMARMCodeGen.dir/ARMCodeGenPrepare.cpp.o
[00:44:11] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetObjectFile.cpp.o
[00:44:15] [ 69%] Building CXX object lib/Target/X86/CMakeFiles/LLVMX86CodeGen.dir/X86TargetTransformInfo.cpp.o
---
[00:53:04] [ 90%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCChecker.cpp.o
[00:53:05] [ 90%] Linking CXX static library ../../../libLLVMHexagonInfo.a
[00:53:05] [ 90%] Built target LLVMHexagonInfo
[00:53:06] Scanning dependencies of target LLVMWebAssemblyCodeGen
[00:53:06] [ 91%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyAddMissingPrototypes.cpp.o
[00:53:07] [ 91%] Built target LLVMHexagonDisassembler
[00:53:07] [ 91%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyArgumentMove.cpp.o
[00:53:09] [ 91%] Building CXX object lib/Target/Hexagon/MCTargetDesc/CMakeFiles/LLVMHexagonDesc.dir/HexagonMCCodeEmitter.cpp.o
[00:53:10] [ 91%] Building CXX object lib/Target/WebAssembly/CMakeFiles/LLVMWebAssemblyCodeGen.dir/WebAssemblyAsmPrinter.cpp.o
---
[01:03:27]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[01:03:43] error: failed to run custom build command for `rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)`
[01:03:43] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_lsan-fe91098473ffc0d9/build-script-build` (exit code: 101)
[01:03:43] --- stdout
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformFuchsia.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface_internal.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_always_instrument.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_never_instrument.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_function_call_trie.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_AArch64.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_segmented_array.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/xray/weak_symbols.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_linux.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_malloc_mac.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/.clang-format
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_flags.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_mac.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_preinit.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/CMakeLists.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_interceptors.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_linux.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_mac.cc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/lsan/weak_symbols.txt
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negvdi2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_thread_fence.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/subdf3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfti.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divdi3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixtfdi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qadd.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/restFP.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qsub.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatditf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/saveFP.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixunstfdi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/divtc3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatunditf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qmul.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/DD.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qdiv.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/multc3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negdf2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/extenddftf2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulosi4.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ashldi3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/absvsi2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatsidf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdidf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fmin_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_abi2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/sfdiv_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath_dlib_asm.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfaddsub.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivmodsi4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/memcpy_forward_vp4cp4n2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fabs_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivmoddi4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_legacy.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivdi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/common_entry_exit_abi1.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/memcpy_likely_aligned.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/divsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/umoddi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfsqrt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath2_ldlib_asm.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/sfsqrt_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/moddi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/modsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fastmath2_dlib_asm.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfmul.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfdiv.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/udivsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fmax_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/fma_opt.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/divdi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dfminmax.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/umodsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/hexagon/dffma.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/adddf3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfti.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvti3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/extendhfsf2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/moddi3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divmoddi4.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdisf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negvti2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/muldi3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_lib.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfsi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/comparesf2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/negdi2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdixf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunstfsi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfdi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/ctzti2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/muldf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_drsub.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/extendsfdf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uidivmod.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssidfvfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negdf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunssfsivfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcpy.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_frsub.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divdf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapdi2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_fcmp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/truncdfsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_div0.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtdf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_ldivmod.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uldivmod.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_synchronize.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivmodsi4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nesf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_dcmp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memset.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ledf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapsi2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gesf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/lesf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nedf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unordsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsisfvfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memmove.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/comparesf2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subsf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/softfloat-alias.list
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divmodsi4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/chkstk.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcmp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negsf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/modsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/mulsf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzsi2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subdf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_idivmod.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzdi2.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch16.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gedf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/adddf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixsfsivfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltdf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqdf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsidfvfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch32.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switchu8.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_4.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync-ops.h
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixdfsivfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/umodsi3.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unorddf2vfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssisfvfp.S
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/floatundidf.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/powixf2.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvdi3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/divtf3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfsi.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/multf3.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_flag_clear_explicit.c
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_trunc_impl.inc
[01:03:43] cargo:rerun-if-changed=/checkout/src/librustc_lsan/../libcompiler_builtins/compiler-rt/lib/builtins/lshrti3.c
---
[01:03:46] -- Looking for fopen in c
[01:03:46] -- Looking for fopen in c - found
[01:03:46] -- Looking for __gcc_personality_v0 in gcc_s
[01:03:46] -- Looking for __gcc_personality_v0 in gcc_s - found
[01:03:46] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[01:03:46] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_G_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[01:03:46] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[01:03:46] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[01:03:46] -- Looking for __func__
[01:03:46] -- Looking for __func__ - found
[01:03:46] -- Looking for dlopen in dl - found
[01:03:46] -- Looking for shm_open in rt
[01:03:46] -- Looking for shm_open in rt - found
[01:03:46] -- Looking for pow in m
[01:03:46] -- Looking for pow in m
[01:03:46] -- Looking for pow in m - found
[01:03:46] -- Looking for pthread_create in pthread - found
[01:03:46] -- Looking for pthread_create in pthread - found
[01:03:46] -- Looking for __cxa_throw in c++
[01:03:46] -- Looking for __cxa_throw in c++ - not found
[01:03:46] -- Looking for __cxa_throw in stdc++
[01:03:46] -- Looking for __cxa_throw in stdc++ - found
[01:03:46] -- Looking for __i386__
[01:03:46] -- Looking for __i386__ - found
[01:03:46] -- Compiler-RT supported architectures: x86_64;i386
[01:03:46] -- Looking for rpc/xdr.h
[01:03:46] -- Looking for rpc/xdr.h - not found
[01:03:46] -- Looking for tirpc/rpc/xdr.h
[01:03:46] -- Looking for tirpc/rpc/xdr.h - not found
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[01:03:46] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[01:03:46] -- Performing Test HAS_THREAD_LOCAL - Success
[01:03:46] -- Configuring done
[01:03:46] -- Generating done
[01:03:46] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[01:03:46] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/lsan/build
[01:03:46] running: "cmake" "--build" "." "--target" "clang_rt.lsan-x86_64" "--config" "Release" "--"
[01:03:46] Scanning dependencies of target RTLSanCommon.x86_64
[01:03:46] [ 10%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common.cc.o
[01:03:46] [ 10%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_linux.cc.o
[01:03:46] [ 10%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_mac.cc.o
[01:03:46] [ 10%] Built target RTLSanCommon.x86_64
[01:03:46] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:46] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:46] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:46] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:46] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:46] [ 20%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:46] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:46] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:46] [ 40%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:46] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:46] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:46] [ 50%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:46] --- stderr
[01:03:46] --- stderr
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1042:17: error: use of undeclared identifier 'mmsghdr'
[01:03:46] CHECK_TYPE_SIZE(mmsghdr);
[01:03:46]                 ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: use of undeclared identifier 'mmsghdr'
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:46]                       ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:1: error: expected expression
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:46] ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:46]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:46]                                  ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1043:23: error: unknown type name 'mmsghdr'
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_hdr);
[01:03:46]                       ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: use of undeclared identifier 'mmsghdr'
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:46]                       ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:1: error: expected expression
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:46] ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.h:1514:34: note: expanded from macro 'CHECK_SIZE_AND_OFFSET'
[01:03:46]                  sizeof(((CLASS *) NULL)->MEMBER));                \
[01:03:46]                                  ^
[01:03:46] /checkout/src/libcompiler_builtins/compiler-rt/lib/sanitizer_common/sanitizer_platform_limits_posix.cc:1044:23: error: unknown type name 'mmsghdr'
[01:03:46] CHECK_SIZE_AND_OFFSET(mmsghdr, msg_len);
[01:03:46] 7 errors generated.
[01:03:46] 7 errors generated.
[01:03:46] gmake[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o] Error 1
[01:03:46] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:46] gmake[1]: *** [lib/lsan/CMakeFiles/clang_rt.lsan-x86_64.dir/rule] Error 2
[01:03:46] gmake: *** [clang_rt.lsan-x86_64] Error 2
[01:03:46] command did not execute successfully, got: exit code: 2
[01:03:46] 
[01:03:46] 
[01:03:46] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.31/src/lib.rs:643:5
[01:03:46] 
[01:03:46] warning: build failed, waiting for other jobs to finish...
[01:03:46] error: failed to run custom build command for `rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)`
[01:03:46] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_tsan-22fe0cfc0ba96036/build-script-build` (exit code: 101)
[01:03:46] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_tsan-22fe0cfc0ba96036/build-script-build` (exit code: 101)
[01:03:46] --- stdout
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformFuchsia.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface_internal.h
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_always_instrument.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.cc
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_never_instrument.txt
[01:03:46] cargo:rerun-if-changed=/checkout/src/librustc_tsan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.cc
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
[01:03:49] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/tsan/build
[01:03:49] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/tsan/build
[01:03:49] running: "cmake" "--build" "." "--target" "clang_rt.tsan-x86_64" "--config" "Release" "--"
[01:03:49] Scanning dependencies of target RTUbsan.x86_64
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_monitor.cc.o
[01:03:49] [  7%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[01:03:49] [  7%] Built target RTUbsan.x86_64
[01:03:49] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:49] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:49] [ 15%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:49] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:49] [ 23%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:49] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:49] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:49] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:49] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[01:03:49] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:49] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
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
[01:03:49] gmake[3]: *** Waiting for unfinished jobs....
[01:03:49] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:49] gmake[1]: *** [lib/tsan/CMakeFiles/clang_rt.tsan-x86_64.dir/rule] Error 2
[01:03:49] gmake: *** [clang_rt.tsan-x86_64] Error 2
[01:03:49] command did not execute successfully, got: exit code: 2
[01:03:49] 
[01:03:49] 
[01:03:49] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.31/src/lib.rs:643:5
[01:03:49] 
[01:03:49] warning: build failed, waiting for other jobs to finish...
[01:03:50] error: failed to run custom build command for `rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)`
[01:03:50] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-f15bc57f5f7bfb26/build-script-build` (exit code: 101)
[01:03:50] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_msan-f15bc57f5f7bfb26/build-script-build` (exit code: 101)
[01:03:50] --- stdout
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_syscalls.awk
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/utils/generate_netbsd_ioctls.awk
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/merge_data_flow.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/collect_data_flow.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDataFlowTrace.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/dataflow/DataFlow.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone_preinit.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_monitor.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtems.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping_myriad.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_local.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformFuchsia.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_mapping.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_dynamic_shadow.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profiling_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_allocator.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_flags.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_basic_logging.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/allocator_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/profile_collector_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/segmented_array_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/function_call_trie_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_recursion_guard.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_profile_collector.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[01:03:50] cargo:rerun-if-changed=/checkout/src/librustc_msan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
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
[01:03:52] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[01:03:52] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/msan/build
[01:03:52] running: "cmake" "--build" "." "--target" "clang_rt.msan-x86_64" "--config" "Release" "--"
[01:03:52] Scanning dependencies of target RTUbsan.x86_64
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_monitor.cc.o
[01:03:52] [  9%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[01:03:52] [  9%] Built target RTUbsan.x86_64
[01:03:52] Scanning dependencies of target RTSanitizerCommonCoverage.x86_64
[01:03:52] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sancov_flags.cc.o
[01:03:52] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[01:03:52] Scanning dependencies of target RTSanitizerCommon.x86_64
[01:03:52] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[01:03:52] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[01:03:52] [ 18%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonCoverage.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[01:03:52] [ 27%] Built target RTSanitizerCommonCoverage.x86_64
[01:03:52] Scanning dependencies of target RTSanitizerCommonSymbolizer.x86_64
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_allocator_report.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_mac_libcdep.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_posix_libcdep.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stoptheworld_linux_libcdep.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stackdepot.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace.cc.o
[01:03:52] [ 27%] Built target RTSanitizerCommonLibc.x86_64
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[01:03:52] Scanning dependencies of target RTInterception.x86_64
[01:03:52] [ 27%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_libcdep.cc.o
[01:03:52] [ 27%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_linux.cc.o
[01:03:52] [ 27%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_mac.cc.o
[01:03:52] [ 27%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_win.cc.o
[01:03:52] [ 27%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_type_test.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_printer.cc.o
[01:03:52] [ 36%] Built target RTInterception.x86_64
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_stacktrace_sparc.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libbacktrace.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_libcdep.cc.o
[01:03:52] [ 36%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_mac.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_markup.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[01:03:52] [ 45%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_posix_libcdep.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_report.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_symbolizer_win.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_linux_libcdep.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.x86_64.dir/sanitizer_unwind_win.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_openbsd.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[01:03:52] [ 54%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[01:03:52] [ 54%] Built target RTSanitizerCommonSymbolizer.x86_64
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_openbsd.cc.o
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[01:03:52] [ 63%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_printf.cc.o
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
[01:03:52] gmake[3]: *** Waiting for unfinished jobs....
[01:03:52] gmake[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[01:03:52] gmake[1]: *** [lib/msan/CMakeFiles/clang_rt.msan-x86_64.dir/rule] Error 2
[01:03:52] gmake: *** [clang_rt.msan-x86_64] Error 2
[01:03:52] command did not execute successfully, got: exit code: 2
[01:03:52] 
[01:03:52] 
[01:03:52] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.31/src/lib.rs:643:5
[01:03:52] 
[01:03:52] warning: build failed, waiting for other jobs to finish...
[01:04:02] [RUSTC-TIMING] core test:false 46.687
[01:04:02] error: build failed
[01:04:02] error: build failed
[01:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[01:04:02] expected success, got: exit code: 101
[01:04:02] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1118:9
[01:04:02] travis_fold:end:stage1-std

[01:04:02] travis_time:end:stage1-std:start=1533342641628278716,finish=1533342688734989214,duration=47106710498

---
travis_time:end:3e819108:start=1533342689556009525,finish=1533342689563763743,duration=7754218
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:114614d3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:057330d2
travis_time:start:057330d2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00985d5e
$ dmesg | grep -i kill
