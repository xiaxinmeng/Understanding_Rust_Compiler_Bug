plain
[00:32:02] warning: 3 warnings generated.
[00:33:18] error: failed to run custom build command for `rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)`
[00:33:18] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build/rustc_asan-b19085b8093e9c00/build-script-build` (exit code: 101)
[00:33:18] --- stdout
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/CODE_OWNERS.TXT
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/CREDITS.TXT
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/scripts/unbalanced_allocs.py
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeak.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilPosix.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOPosix.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInternal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemPosix.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsymWin.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtraCounters.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerRandom.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIO.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerLoop.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilDarwin.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctions.def
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/FuzzerUnittest.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/tests/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerFlags.def
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmem.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerInterface.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilFuchsia.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemFuchsia.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerTracePC.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDictionary.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerIOWindows.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsWeakAlias.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/README.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilWindows.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/standalone/StandaloneFuzzTargetMain.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDefs.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCommand.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMain.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerExtFunctionsDlsym.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerValueBitMap.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerClangCounters.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerShmemWindows.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/afl/afl_driver.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMerge.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCorpus.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerOptions.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerMutate.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtilLinux.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerUtil.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerCrossOver.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/build.sh
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerDriver.cpp
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/fuzzer/FuzzerSHA1.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_win.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_checks.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_interface.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan.syms.extra
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_value.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dynamic_runtime_thunk.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_signals_standalone.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init_standalone.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_dll_thunk.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_win_weak_interception.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_platform.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag_standalone.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_init.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_diag.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_handlers_cxx.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_type_hash_itanium.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/weak_symbols.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan/ubsan_flags.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_symbolize.py
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/scripts/asan_device_setup
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_internal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/.clang-format
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_blacklist.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fake_stack.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_suppressions.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stack.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interface_internal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_memory_profile.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_mac.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_linux.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_init_version.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_debugging.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_linux.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_interface_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_benchmarks_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_main.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test_helpers.mm
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mem_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_racy_double_free_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_asm_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_config.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_mac_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.ignore
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_str_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test_utils.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_fake_stack_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_globals_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_exceptions_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_internal_interface_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_noinst_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/tests/asan_oob_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_fuchsia.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_shadow_setup.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals_win.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_malloc_win.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_new_delete.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/README.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_preinit.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_rtl.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan.syms.extra
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_errors.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_report.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mapping.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_allocator.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_posix.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_lock.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_premap_shadow.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors_memintrinsics.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_activation_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_mac.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_flags.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_interceptors.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_scariness_score.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_descriptions.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_weak_interception.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dynamic_runtime_thunk.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_poisoning.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_win_dll_thunk.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_stats.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/weak_symbols.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_globals.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/asan/asan_thread.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi_blacklist.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/cfi/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/cfi/cfi.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMergeFile.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingRuntime.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPort.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformOther.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingInternal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformDarwin.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingFile.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/WindowsMMap.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingBuffer.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingWriter.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfData.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfiling.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingNameVar.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/GCDAProfiling.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingPlatformLinux.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan.syms.extra
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/ubsan_minimal/ubsan_minimal_handlers.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/safestack/.clang-format
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/safestack/safestack.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/safestack/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/.clang-format
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interface_internal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_flags.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_interceptors.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_allocator.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_blacklist.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.syms.extra
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_linux.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_report.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_poisoning.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_thread.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/hwasan/hwasan_new_delete.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_arm.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_log_records.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_inmemory_log.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_powerpc64_asm.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_log_interface.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_inmemory_log.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips64.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips64.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging_impl.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_x86_64.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_defs.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_mips.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_arm.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/xray_unit_test_main.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/fdr_logging_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/tests/unit/buffer_queue_test.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/tests/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_init.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_mips.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_tsc.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_flags.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_fdr_logging.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_AArch64.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_interface_internal.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_always_instrument.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_never_instrument.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_powerpc64.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_x86_64.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_trampoline_AArch64.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_buffer_queue.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/xray_utils.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/xray/weak_symbols.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_linux.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_malloc_mac.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/.clang-format
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_flags.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common_mac.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_preinit.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_interceptors.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_allocator.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_linux.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_common.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_thread.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/lsan_mac.cc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/lsan/weak_symbols.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negvdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_thread_fence.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/subdf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfti.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divdi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixtfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qadd.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/restFP.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qsub.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatditf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/saveFP.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/fixunstfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/divtc3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/floatunditf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qmul.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/DD.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/gcc_qdiv.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ppc/multc3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negdf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/extenddftf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/mulosi4.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ashldi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/absvsi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatsidf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdidf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/adddf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfti.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/extendhfsf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/moddi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divmoddi4.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdisf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negvti2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/muldi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_lib.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfsi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/comparesf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatdixf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunstfsi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixtfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ctzti2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/muldf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_drsub.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/extendsfdf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uidivmod.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssidfvfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negdf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunssfsivfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcpy.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_frsub.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divdf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapdi2.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_fcmp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/truncdfsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_div0.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gtdf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_max_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_ldivmod.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_uldivmod.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_synchronize.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivmodsi4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nesf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_dcmp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memset.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ledf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/bswapsi2.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gesf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/lesf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/nedf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unordsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsisfvfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divsi3.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memmove.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/comparesf2.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subsf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/softfloat-alias.list
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/divmodsi4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_min_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_sub_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_xor_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_nand_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_memcmp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/negsf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/modsi3.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/mulsf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzsi2.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/subdf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umax_8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_idivmod.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/clzdi2.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch16.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/gedf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/adddf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/udivsi3.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixsfsivfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_add_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/ltdf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/eqdf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatsidfvfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switch32.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_and_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/switchu8.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/addsf3vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_umin_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync_fetch_and_or_4.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/sync-ops.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/fixdfsivfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/umodsi3.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/unorddf2vfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/arm/floatunssisfvfp.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatundidf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/powixf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvdi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divtf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfsi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/multf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_flag_clear_explicit.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_trunc_impl.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/lshrti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/addtf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ashlti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunsxfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ctzdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ffsdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ashrti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/umodti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatuntixf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixdfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixxfti.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/cmpdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixsfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/truncsfhf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_trunc.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/truncdfhf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/clzdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/absvdi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/mulodi4.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/aarch64/chkstk.S
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/subsf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/comparetf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/CMakeLists.txt
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/addvti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divsf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/emutls.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/eprintf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/addsf3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/absvti2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/int_types.h
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_extend_impl.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floattitf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/paritydi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fp_fixint_impl.inc
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negti2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/extendsfdf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/lshrdi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negsf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/popcountsi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floattidf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ctzsi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunssfsi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic_flag_test_and_set.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatundixf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatunsitf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divsc3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ashrdi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/enable_execute_stack.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/negvsi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunsxfti.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/modti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatditf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/floatuntitf.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/bswapsi2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/gcc_personality_v0.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunsdfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/subvsi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunssfdi.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/addvdi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/umodsi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/os_version_check.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/fixunsdfti.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/truncdfsf2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/divti3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/ffsti2.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/atomic.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/multi3.c
[00:33:18] cargo:rerun-if-changed=/checkout/src/librustc_asan/../libcompiler_builtins/compiler-rt/lib/builtins/mulvsi3.c
---
[00:33:20] -- Looking for fopen in c
[00:33:20] -- Looking for fopen in c - found
[00:33:20] -- Looking for __gcc_personality_v0 in gcc_s
[00:33:20] -- Looking for __gcc_personality_v0 in gcc_s - found
[00:33:20] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_NODEFAULTLIBS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FPIC_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FPIE_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_BUILTIN_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_EXCEPTIONS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FOMIT_FRAME_POINTER_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FUNWIND_TABLES_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_STACK_PROTECTOR_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_SANITIZE_SAFE_STACK_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_HIDDEN_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FRTTI_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_RTTI_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FFREESTANDING_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_FUNCTION_SECTIONS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_STD_CXX11_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC
[00:33:20] -- Performing Test COMPILER_RT_HAS_FTLS_MODEL_INITIAL_EXEC - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FNO_LTO_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_MSSE3_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_MSSE4_2_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_SYSROOT_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_MCRC_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_FVISIBILITY_INLINES_HIDDEN_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_GR_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_GR_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_GS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_GS_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_MT_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_MT_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_Oy_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_Oy_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_GLINE_TABLES_ONLY_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_G_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_G_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_Zi_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_Zi_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WALL_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WALL_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WERROR_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WFRAME_LARGER_THAN_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WGLOBAL_CONSTRUCTORS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WC99_EXTENSIONS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WGNU_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WNON_VIRTUAL_DTOR_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WVARIADIC_MACROS_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WUNUSED_PARAMETER_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WCOVERED_SWITCH_DEFAULT_FLAG - Success
[00:33:20] -- Performing Test COMPILER_RT_HAS_W4_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_W4_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WX_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WX_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4146_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4291_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4221_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4391_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4722_FLAG - Failed
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG
[00:33:20] -- Performing Test COMPILER_RT_HAS_WD4800_FLAG - Failed
[00:33:20] -- Looking for __func__
[00:33:20] -- Looking for __func__ - found
[00:33:20] -- Looking for dlopen in dl - found
[00:33:20] -- Looking for shm_open in rt
[00:33:20] -- Looking for shm_open in rt - found
[00:33:20] -- Looking for pow in m
[00:33:20] -- Looking for pow in m
[00:33:20] -- Looking for pow in m - found
[00:33:20] -- Looking for pthread_create in pthread - found
[00:33:20] -- Looking for pthread_create in pthread - found
[00:33:20] -- Looking for __cxa_throw in stdc++
[00:33:20] -- Looking for __cxa_throw in stdc++ - found
[00:33:20] -- Looking for __i386__
[00:33:20] -- Looking for __i386__ - found
[00:33:20] -- Compiler-RT supported architectures: x86_64;i386
[00:33:20] -- Looking for rpc/xdr.h
[00:33:20] -- Looking for rpc/xdr.h - not found
[00:33:20] -- Looking for tirpc/rpc/xdr.h
[00:33:20] -- Looking for tirpc/rpc/xdr.h - not found
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_ATOMICS - Success
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_FCNTL_LCK - Success
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME
[00:33:20] -- Performing Test COMPILER_RT_TARGET_HAS_UNAME - Success
[00:33:20] -- Performing Test HAS_THREAD_LOCAL - Success
[00:33:20] -- Configuring done
[00:33:20] -- Generating done
[00:33:20] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build
[00:33:20] -- Build files have been written to: /checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build
[00:33:20] running: "cmake" "--build" "." "--target" "asan" "--config" "Release" "--"
[00:33:20] Scanning dependencies of target RTAsan_preinit.x86_64
[00:33:20] [  0%] Building CXX object lib/asan/CMakeFiles/RTAsan_preinit.x86_64.dir/asan_preinit.cc.o
[00:33:20] [  0%] Built target RTAsan_preinit.x86_64
[00:33:20] Scanning dependencies of target RTSanitizerCommon.x86_64
[00:33:20] [  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cc.o
[00:33:20] [  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector1.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_deadlock_detector2.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_errno.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_file.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flags.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_flag_parser.cc.o
[00:33:20] [  2%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_fuchsia.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libc.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_libignore.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_s390.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_mac.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_persistent_allocator.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_linux.cc.o
[00:33:20] [  5%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_netbsd.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_posix.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_platform_limits_solaris.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_posix.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_printf.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_common.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_freebsd.cc.o
[00:33:20] [  7%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_linux.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_mac.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_procmaps_solaris.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_solaris.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stackdepot.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stacktrace.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stacktrace_printer.cc.o
[00:33:20] [ 10%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_stoptheworld_mac.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_suppressions.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_symbolizer.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_symbolizer_fuchsia.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_symbolizer_libbacktrace.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_symbolizer_mac.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_symbolizer_win.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_tls_get_addr.cc.o
[00:33:20] [ 12%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_thread_registry.cc.o
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_win.cc.o
[00:33:20] [ 15%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_x86_64.S.o
[00:33:20] [ 15%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_linux_mips64.S.o
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_termination.cc.o
[00:33:20] [ 15%] Built target RTSanitizerCommon.x86_64
[00:33:20] Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cc.o
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cc.o
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sancov_flags.cc.o
[00:33:20] [ 15%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_coverage_fuchsia.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_coverage_libcdep_new.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_coverage_win_sections.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_linux_libcdep.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_mac_libcdep.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_posix_libcdep.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stacktrace_libcdep.cc.o
[00:33:20] [ 17%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_stoptheworld_linux_libcdep.cc.o
[00:33:20] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_symbolizer_libcdep.cc.o
[00:33:20] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_symbolizer_posix_libcdep.cc.o
[00:33:20] [ 20%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_unwind_linux_libcdep.cc.o
[00:33:20] [ 20%] Built target RTSanitizerCommonLibc.x86_64
[00:33:20] Scanning dependencies of target RTInterception.x86_64
[00:33:20] [ 20%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_linux.cc.o
[00:33:20] [ 20%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_mac.cc.o
[00:33:20] [ 20%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_win.cc.o
[00:33:20] [ 20%] Building CXX object lib/interception/CMakeFiles/RTInterception.x86_64.dir/interception_type_test.cc.o
[00:33:20] [ 20%] Built target RTInterception.x86_64
[00:33:20] Scanning dependencies of target RTLSanCommon.x86_64
[00:33:20] [ 20%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common.cc.o
[00:33:20] [ 20%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_linux.cc.o
[00:33:20] [ 20%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.x86_64.dir/lsan_common_mac.cc.o
[00:33:20] [ 20%] Built target RTLSanCommon.x86_64
[00:33:20] Scanning dependencies of target RTUbsan.x86_64
[00:33:20] [ 20%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_diag.cc.o
[00:33:20] [ 20%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_init.cc.o
[00:33:20] [ 20%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_flags.cc.o
[00:33:20] [ 20%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_handlers.cc.o
[00:33:20] [ 20%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.x86_64.dir/ubsan_value.cc.o
[00:33:20] [ 20%] Built target RTUbsan.x86_64
[00:33:20] Scanning dependencies of target RTAsan.x86_64
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_allocator.cc.o
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_activation.cc.o
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_debugging.cc.o
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_descriptions.cc.o
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_errors.cc.o
[00:33:20] [ 20%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_fake_stack.cc.o
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_flags.cc.o
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_fuchsia.cc.o
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_globals.cc.o
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_globals_win.cc.o
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_interceptors.cc.o
[00:33:20] Scanning dependencies of target RTAsan_cxx.i386
[00:33:20] [ 23%] Building CXX object lib/asan/CMakeFiles/RTAsan_cxx.i386.dir/asan_new_delete.cc.o
[00:33:20] [ 23%] Built target RTAsan_cxx.i386
[00:33:20] Scanning dependencies of target RTUbsan_cxx.i386
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.i386.dir/ubsan_handlers_cxx.cc.o
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.i386.dir/ubsan_type_hash.cc.o
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.i386.dir/ubsan_type_hash_itanium.cc.o
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.i386.dir/ubsan_type_hash_win.cc.o
[00:33:20] [ 25%] Built target RTUbsan_cxx.i386
[00:33:20] Scanning dependencies of target RTAsan_cxx.x86_64
[00:33:20] [ 25%] Building CXX object lib/asan/CMakeFiles/RTAsan_cxx.x86_64.dir/asan_new_delete.cc.o
[00:33:20] [ 25%] Built target RTAsan_cxx.x86_64
[00:33:20] Scanning dependencies of target RTUbsan_cxx.x86_64
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.x86_64.dir/ubsan_handlers_cxx.cc.o
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.x86_64.dir/ubsan_type_hash.cc.o
[00:33:20] [ 25%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.x86_64.dir/ubsan_type_hash_itanium.cc.o
[00:33:20] [ 28%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan_cxx.x86_64.dir/ubsan_type_hash_win.cc.o
[00:33:20] [ 28%] Built target RTUbsan_cxx.x86_64
[00:33:20] Scanning dependencies of target RTAsan_preinit.i386
[00:33:20] [ 28%] Building CXX object lib/asan/CMakeFiles/RTAsan_preinit.i386.dir/asan_preinit.cc.o
[00:33:20] [ 28%] Built target RTAsan_preinit.i386
[00:33:20] Scanning dependencies of target RTSanitizerCommon.i386
[00:33:20] [ 28%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_allocator.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_common.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_deadlock_detector1.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_deadlock_detector2.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_errno.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_file.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_flags.cc.o
[00:33:20] [ 30%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_flag_parser.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_fuchsia.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_libc.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_libignore.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_linux.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_linux_s390.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_mac.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_persistent_allocator.cc.o
[00:33:20] [ 33%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_platform_limits_linux.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_platform_limits_netbsd.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_platform_limits_posix.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_platform_limits_solaris.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_posix.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_printf.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_procmaps_common.cc.o
[00:33:20] [ 35%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_interceptors_memintrinsics.cc.o
[00:33:20] [ 35%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_procmaps_freebsd.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_procmaps_linux.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_procmaps_mac.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_procmaps_solaris.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_solaris.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_stackdepot.cc.o
[00:33:20] Scanning dependencies of target RTSanitizerCommonLibc.i386
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_common_libcdep.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_stacktrace.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_allocator_checks.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sancov_flags.cc.o
[00:33:20] [ 38%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_stacktrace_printer.cc.o
[00:33:20] [ 41%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_coverage_fuchsia.cc.o
[00:33:20] [ 41%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_coverage_libcdep_new.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_stoptheworld_mac.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_suppressions.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_coverage_win_sections.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_linux_libcdep.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_symbolizer.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_symbolizer_fuchsia.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_mac_libcdep.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_symbolizer_libbacktrace.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_posix_libcdep.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_symbolizer_mac.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_symbolizer_win.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_tls_get_addr.cc.o
[00:33:20] [ 43%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_stacktrace_libcdep.cc.o
[00:33:20] [ 46%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_thread_registry.cc.o
[00:33:20] [ 48%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_stoptheworld_linux_libcdep.cc.o
[00:33:20] [ 48%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_win.cc.o
[00:33:20] [ 48%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_linux_x86_64.S.o
[00:33:20] [ 48%] Building ASM object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_linux_mips64.S.o
[00:33:20] [ 48%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_symbolizer_libcdep.cc.o
[00:33:20] [ 48%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.i386.dir/sanitizer_termination.cc.o
[00:33:20] [ 48%] Built target RTSanitizerCommon.i386
[00:33:20] Scanning dependencies of target RTInterception.i386
[00:33:20] [ 51%] Building CXX object lib/interception/CMakeFiles/RTInterception.i386.dir/interception_linux.cc.o
[00:33:20] [ 51%] Building CXX object lib/interception/CMakeFiles/RTInterception.i386.dir/interception_mac.cc.o
[00:33:20] [ 51%] Building CXX object lib/interception/CMakeFiles/RTInterception.i386.dir/interception_win.cc.o
[00:33:20] [ 51%] Building CXX object lib/interception/CMakeFiles/RTInterception.i386.dir/interception_type_test.cc.o
[00:33:20] [ 51%] Built target RTInterception.i386
[00:33:20] Scanning dependencies of target RTLSanCommon.i386
[00:33:20] [ 53%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.i386.dir/lsan_common.cc.o
[00:33:20] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_symbolizer_posix_libcdep.cc.o
[00:33:20] [ 53%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_linux.cc.o
[00:33:20] [ 53%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.i386.dir/sanitizer_unwind_linux_libcdep.cc.o
[00:33:20] [ 53%] Built target RTSanitizerCommonLibc.i386
[00:33:20] Scanning dependencies of target RTUbsan.i386
[00:33:20] [ 53%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.i386.dir/ubsan_diag.cc.o
[00:33:20] [ 56%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_mac.cc.o
[00:33:20] [ 56%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_malloc_linux.cc.o
[00:33:20] [ 56%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.i386.dir/lsan_common_linux.cc.o
[00:33:20] [ 56%] Building CXX object lib/lsan/CMakeFiles/RTLSanCommon.i386.dir/lsan_common_mac.cc.o
[00:33:20] [ 56%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.i386.dir/ubsan_init.cc.o
[00:33:20] [ 56%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_malloc_mac.cc.o
[00:33:20] [ 56%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_malloc_win.cc.o
[00:33:20] [ 56%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_memory_profile.cc.o
[00:33:20] [ 58%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.i386.dir/ubsan_flags.cc.o
[00:33:20] [ 58%] Built target RTLSanCommon.i386
[00:33:20] [ 58%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.i386.dir/ubsan_handlers.cc.o
[00:33:20] [ 58%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.i386.dir/ubsan_value.cc.o
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_poisoning.cc.o
[00:33:20] Scanning dependencies of target RTAsan.i386
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_allocator.cc.o
[00:33:20] [ 58%] Built target RTUbsan.i386
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_activation.cc.o
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_posix.cc.o
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_debugging.cc.o
[00:33:20] [ 58%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_premap_shadow.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_report.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_descriptions.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_rtl.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_errors.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_fake_stack.cc.o
[00:33:20] [ 61%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_shadow_setup.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_flags.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_stack.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_stats.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_fuchsia.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_globals.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_suppressions.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_globals_win.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_interceptors.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_thread.cc.o
[00:33:20] [ 64%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_interceptors_memintrinsics.cc.o
[00:33:20] [ 66%] Building CXX object lib/asan/CMakeFiles/RTAsan.x86_64.dir/asan_win.cc.o
[00:33:20] [ 66%] Built target RTAsan.x86_64
[00:33:20] [ 66%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_linux.cc.o
[00:33:20] Scanning dependencies of target RTAsan_dynamic.i386
[00:33:20] [ 66%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_allocator.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_mac.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_malloc_linux.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_malloc_mac.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_malloc_win.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_memory_profile.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_poisoning.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_posix.cc.o
[00:33:20] [ 69%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_activation.cc.o
[00:33:20] [ 71%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_premap_shadow.cc.o
[00:33:20] [ 71%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_report.cc.o
[00:33:20] [ 71%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_debugging.cc.o
[00:33:20] [ 71%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_descriptions.cc.o
[00:33:20] [ 71%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_rtl.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_errors.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_shadow_setup.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_fake_stack.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_stack.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_stats.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_suppressions.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_thread.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_flags.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_fuchsia.cc.o
[00:33:20] [ 74%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_globals.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan.i386.dir/asan_win.cc.o
[00:33:20] Scanning dependencies of target clang_rt.asan-preinit-i386
[00:33:20] [ 76%] Linking CXX static library ../linux/libclang_rt.asan-preinit-i386.a
[00:33:20] [ 76%] Built target clang_rt.asan-preinit-i386
[00:33:20] Scanning dependencies of target RTAsan_dynamic.x86_64
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_allocator.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_globals_win.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_interceptors.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_activation.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_debugging.cc.o
[00:33:20] [ 76%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_descriptions.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_errors.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_fake_stack.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_flags.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_fuchsia.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_globals.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_globals_win.cc.o
[00:33:20] [ 79%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_interceptors.cc.o
[00:33:20] [ 79%] Built target RTAsan.i386
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_interceptors_memintrinsics.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_linux.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_mac.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_malloc_linux.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_malloc_mac.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_malloc_win.cc.o
[00:33:20] [ 82%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_memory_profile.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_poisoning.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_posix.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_premap_shadow.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_report.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_rtl.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_shadow_setup.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_stack.cc.o
[00:33:20] [ 84%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_stats.cc.o
[00:33:20] [ 87%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_suppressions.cc.o
[00:33:20] [ 87%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_thread.cc.o
[00:33:20] [ 87%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_win.cc.o
[00:33:20] [ 87%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.x86_64.dir/asan_new_delete.cc.o
[00:33:20] [ 87%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_interceptors_memintrinsics.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_linux.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_mac.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_malloc_linux.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_malloc_mac.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_malloc_win.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_memory_profile.cc.o
[00:33:20] [ 89%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_poisoning.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_posix.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_premap_shadow.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_report.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_rtl.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_shadow_setup.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_stack.cc.o
[00:33:20] [ 92%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_stats.cc.o
[00:33:20] Scanning dependencies of target asan_blacklist
[00:33:20] [ 92%] Copying asan_blacklist.txt...
[00:33:20] [ 92%] Built target asan_blacklist
[00:33:20] Scanning dependencies of target clang_rt.asan-preinit-x86_64
[00:33:20] [ 92%] Linking CXX static library ../linux/libclang_rt.asan-preinit-x86_64.a
[00:33:20] [ 92%] Built target clang_rt.asan-preinit-x86_64
[00:33:20] Scanning dependencies of target clang_rt.asan-x86_64
[00:33:20] [ 92%] Linking CXX static library ../linux/libclang_rt.asan-x86_64.a
[00:33:20] [ 94%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_suppressions.cc.o
[00:33:20] [ 94%] Built target clang_rt.asan-x86_64
[00:33:20] Scanning dependencies of target clang_rt.asan_cxx-i386
[00:33:20] [ 94%] Linking CXX static library ../linux/libclang_rt.asan_cxx-i386.a
[00:33:20] [ 94%] Built target clang_rt.asan_cxx-i386
[00:33:20] Scanning dependencies of target clang_rt.asan_cxx-x86_64
[00:33:20] [ 94%] Linking CXX static library ../linux/libclang_rt.asan_cxx-x86_64.a
[00:33:20] [ 94%] Built target clang_rt.asan_cxx-x86_64
[00:33:20] Scanning dependencies of target clang_rt.asan-i386
[00:33:20] [ 94%] Linking CXX static library ../linux/libclang_rt.asan-i386.a
[00:33:20] [ 94%] Built target clang_rt.asan-i386
[00:33:20] [ 94%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_thread.cc.o
[00:33:20] [ 94%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_win.cc.o
[00:33:20] [ 94%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.i386.dir/asan_new_delete.cc.o
[00:33:20] Scanning dependencies of target clang_rt.asan-x86_64-symbols
[00:33:20] [ 97%] Generating exported symbols for clang_rt.asan-x86_64
[00:33:20] [ 97%] Built target RTAsan_dynamic.i386
[00:33:20] [ 97%] Generating version list for clang_rt.asan-dynamic-x86_64
[00:33:20] [ 97%] Built target clang_rt.asan-x86_64-symbols
[00:33:20] Scanning dependencies of target clang_rt.asan_cxx-x86_64-symbols
[00:33:20] [ 97%] Generating exported symbols for clang_rt.asan_cxx-x86_64
[00:33:20] [100%] Generating version list for clang_rt.asan-dynamic-i386
[00:33:20] [100%] Built target clang_rt.asan_cxx-x86_64-symbols
[00:33:20] Scanning dependencies of target RTAsan_dynamic_version_script_dummy.i386
[00:33:20] [100%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic_version_script_dummy.i386.dir/dummy.cc.o
[00:33:20] [100%] Built target RTAsan_dynamic_version_script_dummy.i386
[00:33:20] Scanning dependencies of target clang_rt.asan-dynamic-i386
[00:33:20] [100%] Linking CXX shared library ../linux/libclang_rt.asan-i386.so
[00:33:20] Scanning dependencies of target RTAsan_dynamic_version_script_dummy.x86_64
[00:33:20] [100%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic_version_script_dummy.x86_64.dir/dummy.cc.o
[00:33:20] [100%] Built target RTAsan_dynamic_version_script_dummy.x86_64
[00:33:20] [100%] Built target RTAsan_dynamic.x86_64
[00:33:20] --- stderr
[00:33:20] --- stderr
[00:33:20] /rustroot/bin/ld:/checkout/obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers:1: syntax error in VERSION script
[00:33:20] clang-6.0: error: linker command failed with exit code 1 (use -v to see invocation)
[00:33:20] gmake[3]: *** [lib/linux/libclang_rt.asan-i386.so] Error 1
[00:33:20] gmake[2]: *** [lib/asan/CMakeFiles/clang_rt.asan-dynamic-i386.dir/all] Error 2
[00:33:20] gmake[2]: *** Waiting for unfinished jobs....
[00:33:20] gmake[1]: *** [lib/asan/CMakeFiles/asan.dir/rule] Error 2
[00:33:20] gmake: *** [asan] Error 2
[00:33:20] command did not execute successfully, got: exit code: 2
[00:33:20] 
[00:33:20] 
[00:33:20] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[00:33:20] 
[00:33:20] warning: build failed, waiting for other jobs to finish...
[00:33:22] [RUSTC-TIMING] core test:false 99.439
[00:33:22] error: build failed
[00:33:22] error: build failed
[00:33:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:33:22] expected success, got: exit code: 101
[00:33:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:33:22] travis_fold:end:stage1-std

[00:33:22] travis_time:end:stage1-std:start=1526902582646913520,finish=1526902682538527835,duration=99891614315

---
travis_fold:start:after_failure.4
travis_time:start:00c62a20
$ dmesg | grep -i kill
[   10.497597] init: failsafe main process (1092) killed by TERM signal
[   41.805557] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.4

Done. Your build exited with 1.
