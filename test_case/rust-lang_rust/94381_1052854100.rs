plain
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Feb 27 00:17:17 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
Scanning dependencies of target clang_rt.builtins-i386
/tmp/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:567:1: warning: destructor priorities from 0 to 100 are reserved for the implementation
 static void llvm_writeout_and_clear(void) {
 ^
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/macho_ehframe_registration.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvdi2.c.o
[  1%] Built target count
Scanning dependencies of target clang_rt.builtins-x86_64
Scanning dependencies of target clang_rt.builtins-x86_64
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/xray_basic_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:100:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic push
 ^
 ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:104:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic ignored "-Wglobal-constructors"
 ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:108:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic pop
 ^
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvdi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:13:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
Scanning dependencies of target RTXrayFDR.x86_64
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingInternal.c.o
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvsi2.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvsi2.c.o
Scanning dependencies of target RTSanitizerCommon.x86_64
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvsi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.h:20,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cpp:14:
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/all] Error 2
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfiling.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvti2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
Scanning dependencies of target obj.llvm-tblgen
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/adddf3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.cpp:14:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingValue.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_init.cpp:18:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/adddf3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/adddf3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingInternal.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingValue.c.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling_flags.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvdi3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingBuffer.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvsi3.c.o
---
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashlti3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashrti3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvsi3.c.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingFile.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/all] Error 2
[  3%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/run_program_wrapper.cpp.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzdi2.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingFile.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzsi2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzsi2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/apple_versioning.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzti2.c.o
[  4%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterEmitter.cpp.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashldi3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/cmpdi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o] Error 1
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/cmpti2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMerge.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/comparedf2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/comparedf2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrdi3.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMerge.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/all] Error 2
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/comparesf2.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMergeFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addtf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clear_cache.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/comparetf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/cpu_model.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/divtc3.c:20:26: warning: conflicting types for built-in function '__divtc3'
 COMPILER_RT_ABI Lcomplex __divtc3(long double __a, long double __b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendhftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendsftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendsftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfsi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/fp_mode.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfti.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfsi.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfti.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashrdi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatsitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatsitf.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/divdi3.S.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdidf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunditf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunsitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunsitf.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatuntitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multc3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
/tmp/llvm-project/compiler-rt/lib/builtins/multc3.c:18:38: warning: conflicting types for built-in function '__multc3'
 COMPILER_RT_ABI long double _Complex __multc3(long double a, long double b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multf3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/lshrdi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/moddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/subtf3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/muldi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfhf2.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfhf2.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/udivdi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfsf2.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/umoddi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/divxc3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/i386/fp_mode.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/x86_64/floatdidf.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfdi.c.o
---
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[ 10%] Built target obj.llvm-tblgen
make: *** [all] Error 2
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Feb 27 00:19:11 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o
Scanning dependencies of target RTXrayFDR.x86_64
[  1%] Linking C executable ../../bin/count
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Copying compiler-rt's profile/InstrProfData.inc...
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o
Scanning dependencies of target clang_rt.builtins-i386
[  1%] Built target compiler-rt-headers
[  1%] Built target compiler-rt-headers
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/xray_basic_flags.cpp.o] Error 1
 #    pragma clang diagnostic push
 ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:104:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic ignored "-Wglobal-constructors"
 #    pragma clang diagnostic ignored "-Wglobal-constructors"
 ^
make[1]: /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:108:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic pop
 ^
*** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvdi2.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfiling.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building CXX object projects/compiler-rt/lib/fuzzer/CMakeFiles/RTfuzzer_interceptors.i386.dir/FuzzerInterceptors.cpp.o
[  1%] Built target count
[  1%] Built target count
Scanning dependencies of target clang_rt.builtins-x86_64
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:13:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cpp:14:
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/extensible_rtti.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/all] Error 2
Scanning dependencies of target RTSanitizerCommon.x86_64
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvsi2.c.o
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_allocator_checks.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.h:20,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
   ^
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/ItaniumDemangle.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingInternal.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
   ^
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/macho_ehframe_registration.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.cpp:14:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/adddf3.c.o
[  3%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cpp.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvti2.c.o
Scanning dependencies of target obj.llvm-tblgen
Scanning dependencies of target obj.llvm-tblgen
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingValue.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/adddf3.c.o
[  3%] Built target RTfuzzer_interceptors.i386
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.cpp:13:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  3%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangle.cpp.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfiling.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingBuffer.c.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingBuffer.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addsf3.c.o
[  3%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_common.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingInternal.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingValue.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvdi3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvdi3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvdi3.c.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingFile.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_init.cpp:18:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingBuffer.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvsi3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvsi3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvsi3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/all] Error 2
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/apple_versioning.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashlti3.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/apple_versioning.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/apple_versioning.c.o
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashrti3.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashldi3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMerge.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrdi3.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzdi2.c.o
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMergeFile.c.o
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMergeFile.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrti3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzsi2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/bswapdi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o] Error 1
make[1]: [  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzti2.c.o
*** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/all] Error 2
[  6%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/RustDemangle.cpp.o
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMerge.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addtf3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/umodsi3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/comparetf2.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtc3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/umodti3.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/divtc3.c:20:26: warning: conflicting types for built-in function '__divtc3'
 COMPILER_RT_ABI Lcomplex __divtc3(long double __a, long double __b,
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/emutls.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtf3.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/enable_execute_stack.c.o
---
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatsitf.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floattitf.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/fp_mode.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunditf.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashldi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunsitf.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashrdi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatuntitf.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/divdi3.S.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdidf.S.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
/tmp/llvm-project/compiler-rt/lib/builtins/multc3.c:18:38: warning: conflicting types for built-in function '__multc3'
 COMPILER_RT_ABI long double _Complex __multc3(long double a, long double b,
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multf3.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/lshrdi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/subtf3.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/moddi3.S.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/muldi3.S.o
[ 10%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfdf2.c.o
[ 10%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/udivdi3.S.o
[ 12%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfhf2.c.o
[ 12%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/umoddi3.S.o
[ 12%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/divxc3.c.o
[ 12%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/cpu_model.c.o
[ 12%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfdi.c.o
[ 12%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfti.c.o
---
[ 12%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[ 12%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[ 12%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[ 12%] Built target obj.llvm-tblgen
make: *** [all] Error 2
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Feb 27 00:21:06 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o
[  1%] Linking C executable ../../bin/count
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
Scanning dependencies of target clang_rt.builtins-i386
Scanning dependencies of target RTXrayFDR.x86_64
/tmp/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:567:1: warning: destructor priorities from 0 to 100 are reserved for the implementation
 static void llvm_writeout_and_clear(void) {
---
 ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:108:0: warning: ignoring #pragma clang diagnostic [-Wunknown-pragmas]
 #    pragma clang diagnostic pop
 ^
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/xray_basic_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvdi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:13:
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:13:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/extensible_rtti.cpp.o
[  1%] Built target count
Scanning dependencies of target clang_rt.builtins-x86_64
Scanning dependencies of target clang_rt.builtins-x86_64
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfiling.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvsi2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_fdr_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
Scanning dependencies of target RTSanitizerCommon.x86_64
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvdi2.c.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/xray_fdr_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayFDR.x86_64.dir/all] Error 2
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfiling.c.o
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling_flags.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.h:20,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvsi2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/adddf3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/macho_ehframe_registration.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.cpp:14:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
Scanning dependencies of target obj.llvm-tblgen
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingInternal.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling_flags.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvti2.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingValue.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingInternal.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingBuffer.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addsf3.c.o
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingValue.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingValue.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_init.cpp:18:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvdi3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingBuffer.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvsi3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvdi3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/all] Error 2
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/apple_versioning.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvsi3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingFile.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashlti3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashrti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/apple_versioning.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashldi3.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMerge.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o] Error 1
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzdi2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzdi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/all] Error 2
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrdi3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzsi2.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMerge.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/truncsfhf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ucmpdi2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/fp_mode.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ucmpti2.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/udivdi3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashrdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/divdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdidf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/lshrdi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/udivmodsi4.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/moddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/udivmodti4.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/muldi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/udivdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/umoddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/divxc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/udivti3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/umoddi3.c.o
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addtf3.c.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherOpt.cpp.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/comparetf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtc3.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/divtc3.c:20:26: warning: conflicting types for built-in function '__divtc3'
 COMPILER_RT_ABI Lcomplex __divtc3(long double __a, long double __b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendhftf2.c.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcher.cpp.o
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunditf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunsitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatuntitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multc3.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/multc3.c:18:38: warning: conflicting types for built-in function '__multc3'
 COMPILER_RT_ABI long double _Complex __multc3(long double a, long double b,
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DFAPacketizerEmitter.cpp.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/subtf3.c.o
---
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[ 10%] Built target obj.llvm-tblgen
make: *** [all] Error 2
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Feb 27 00:22:41 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
[  1%] Copying compiler-rt's xray/xray_records.h...
Scanning dependencies of target RTSanitizerCommonLibc.x86_64
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o
[  1%] Linking C executable ../../bin/count
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
Scanning dependencies of target clang_rt.builtins-i386
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o
/tmp/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:567:1: warning: destructor priorities from 0 to 100 are reserved for the implementation
 static void llvm_writeout_and_clear(void) {
 static void llvm_writeout_and_clear(void) {
 ^
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/xray_basic_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/all] Error 2
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvdi2.c.o
make[1]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfiling.c.o
Scanning dependencies of target clang_rt.builtins-x86_64
[  1%] Copying compiler-rt's profile/InstrProfData.inc...
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
 #    pragma clang diagnostic pop
 ^
[  1%] Built target count
[  1%] Built target compiler-rt-headers
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfiling.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common_libcdep.cpp:13:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvdi2.c.o
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/log_error_to_stderr.cpp.o
Scanning dependencies of target RTSanitizerCommon.x86_64
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvsi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.h:20,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
   ^
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/sanitizer_common_libcdep.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.x86_64.dir/all] Error 2
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/macho_ehframe_registration.cpp.o
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvsi2.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingInternal.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
Scanning dependencies of target obj.llvm-tblgen
In file included from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.h:16,
                 from /tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_allocator.cpp:14:
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingValue.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvti2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_init.cpp:18:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/adddf3.c.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/adddf3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingBuffer.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingValue.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
make[2]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/sanitizer_allocator.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.x86_64.dir/all] Error 2
[  1%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addsf3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingBuffer.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvdi3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvdi3.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvsi3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
   ^
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/all] Error 2
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvsi3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling_flags.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/apple_versioning.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/apple_versioning.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashldi3.c.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMerge.c.o
[  3%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMerge.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashlti3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/ashrti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o] Error 1
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/all] Error 2
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMerge.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMergeFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrdi3.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingMergeFile.c.o
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/umodsi3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/fp_mode.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/umodti3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/emutls.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/enable_execute_stack.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashrdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/divdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdidf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/gcc_personality_v0.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/clear_cache.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addtf3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/lshrdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/moddi3.S.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CodeGenRegisters.cpp.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/muldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtc3.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/udivdi3.S.o
/tmp/llvm-project/compiler-rt/lib/builtins/divtc3.c:20:26: warning: conflicting types for built-in function '__divtc3'
 COMPILER_RT_ABI Lcomplex __divtc3(long double __a, long double __b,
                          ^
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/umoddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/divxc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendhftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendsftf2.c.o
---
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherEmitter.cpp.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multc3.c.o
[  9%] Built target clang_rt.builtins-i386
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherGen.cpp.o
/tmp/llvm-project/compiler-rt/lib/builtins/multc3.c:18:38: warning: conflicting types for built-in function '__multc3'
 COMPILER_RT_ABI long double _Complex __multc3(long double a, long double b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/subtf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/trunctfdf2.c.o
---
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[ 10%] Built target obj.llvm-tblgen
make: *** [all] Error 2
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Sun Feb 27 00:24:18 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingInternal.c.o
Scanning dependencies of target clang_rt.builtins-i386
Scanning dependencies of target RTXray.x86_64
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flag_parser.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_basic_flags.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvdi2.c.o
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/extensible_rtti.cpp.o
Scanning dependencies of target clang_rt.builtins-x86_64
Scanning dependencies of target clang_rt.builtins-x86_64
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/xray_basic_flags.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayBASIC.x86_64.dir/all] Error 2
make[1]: *** Waiting for unfinished jobs....
[  1%] Built target count
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building CXX object projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvdi2.c.o
---
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvsi2.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingBuffer.c.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/absvti2.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/macho_ehframe_registration.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/absvti2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_flags.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:18,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_init.cpp:18:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/adddf3.c.o
Scanning dependencies of target obj.llvm-tblgen
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_buffer_queue.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_function_call_trie.h:17,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.h:20,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profile_collector.cpp:14:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
   ^
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_init.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  1%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmMatcherEmitter.cpp.o
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/adddf3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingFile.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  1%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/addvti3.c.o
[  1%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingMergeFile.c.o
[  3%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/addvti3.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/xray_buffer_queue.cpp.o] Error 1
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXray.x86_64.dir/all] Error 2
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingWriter.c.o
[  4%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-x86_64.dir/InstrProfilingPlatformDarwin.c.o
[  4%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/apple_versioning.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
---
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingPlatformDarwin.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashlti3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapdi2.c.o
In file included from /tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_atomic.h:16:0,
                 from /tmp/llvm-project/compiler-rt/lib/xray/xray_profiling.cpp:17:
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h: In function 'constexpr __sanitizer::uptr __sanitizer::RoundUpTo(__sanitizer::uptr, __sanitizer::uptr)':
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:298:42: error: call to non-constexpr function 'void __sanitizer::RawWrite(const char*)'
       for (const char* m : msgs) RawWrite(m);  \
                                          ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_internal_defs.h:303:25: note: in expansion of macro 'RAW_CHECK_MSG'
 #define RAW_CHECK(expr) RAW_CHECK_MSG(expr, #expr "\n", )
                         ^
/tmp/llvm-project/compiler-rt/lib/xray/../sanitizer_common/sanitizer_common.h:430:3: note: in expansion of macro 'RAW_CHECK'
   RAW_CHECK(IsPowerOfTwo(boundary));
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingPlatformFuchsia.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrdi3.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/bswapsi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profile_collector.cpp.o] Error 1
make[2]: *** Waiting for unfinished jobs....
[  6%] Building C object projects/compiler-rt/lib/profile/CMakeFiles/clang_rt.profile-i386.dir/InstrProfilingPlatformLinux.c.o
cc1: warning: command line option '-nostdinc++' is valid for C++/ObjC++ but not for C
[  6%] Building CXX object projects/compiler-rt/lib/orc/CMakeFiles/RTOrc.x86_64.dir/elfnix_platform.cpp.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/ashrti3.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clzdi2.c.o
---
[  6%] Building CXX object lib/Demangle/CMakeFiles/LLVMDemangle.dir/MicrosoftDemangleNodes.cpp.o
[  6%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterInst.cpp.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/cmpti2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/clzsi2.c.o
make[2]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/xray_profiling.cpp.o] Error 1
make[1]: *** [projects/compiler-rt/lib/xray/CMakeFiles/RTXrayPROFILING.x86_64.dir/all] Error 2
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/comparedf2.c.o
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/clzti2.c.o
[  6%] Linking CXX static library ../../../../lib/clang/14.0.0/lib/linux/libclang_rt.profile-x86_64.a
[  6%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/comparesf2.c.o
---
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/comparetf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/clear_cache.c.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DAGISelMatcherOpt.cpp.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/cpu_model.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/divtc3.c:20:26: warning: conflicting types for built-in function '__divtc3'
 COMPILER_RT_ABI Lcomplex __divtc3(long double __a, long double __b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/divtf3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/fp_mode.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extenddftf2.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendhftf2.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/ashrdi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/extendsftf2.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/divdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdidf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfsi.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatdisf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfti.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixtfti.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundidf.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/floatundisf.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfsi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/fixunstfti.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/lshrdi3.S.o
[  9%] Built target RTOrc.x86_64
[  9%] Built target RTOrc.x86_64
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/moddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatsitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatsitf.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/muldi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floattitf.c.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/udivdi3.S.o
[  9%] Building ASM object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/i386/umoddi3.S.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/divxc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatunsitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfdi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatuntitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/floatuntitf.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixxfti.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multc3.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixunsxfdi.c.o
/tmp/llvm-project/compiler-rt/lib/builtins/multc3.c:18:38: warning: conflicting types for built-in function '__multc3'
 COMPILER_RT_ABI long double _Complex __multc3(long double a, long double b,
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-i386.dir/fixunsxfsi.c.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/multf3.c.o
[  9%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/DFAEmitter.cpp.o
[  9%] Building C object projects/compiler-rt/lib/builtins/CMakeFiles/clang_rt.builtins-x86_64.dir/powitf2.c.o
---
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/X86RecognizableInstr.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyDisassemblerEmitter.cpp.o
[ 10%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[ 10%] Built target obj.llvm-tblgen
make: *** [all] Error 2
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
##[error]Process completed with exit code 1.
Post job cleanup.
