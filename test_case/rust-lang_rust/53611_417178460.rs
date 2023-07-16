plain
[00:22:06] -- Targeting NVPTX
[00:22:06] -- Targeting Hexagon
[00:22:06] -- Targeting WebAssembly
[00:22:07] -- Targeting RISCV
[00:22:08] CMake Warning (dev) at utils/benchmark/CMakeLists.txt:3 (project):
[00:22:08]   Policy CMP0048 is not set: project() command manages VERSION variables.
[00:22:08]   command to set the policy and suppress this warning.
[00:22:08] 
[00:22:08] 
[00:22:08]   The following variable(s) would be set to empty:
[00:22:08]     PROJECT_VERSION
[00:22:08]     PROJECT_VERSION_MAJOR
[00:22:08]     PROJECT_VERSION_MINOR
[00:22:08]     PROJECT_VERSION_PATCH
[00:22:08]     PROJECT_VERSION_PATCH
[00:22:08] This warning is for project developers.  Use -Wno-dev to suppress it.
[00:22:08] 
[00:22:08] -- Found Git: /usr/local/bin/git (found version "2.16.1") 
[00:22:08] -- git Version: v0.0.0
[00:22:08] -- Version: 0.0.0
[00:22:08] -- Performing Test HAVE_CXX_FLAG_STD_CXX11
[00:22:08] -- Performing Test HAVE_CXX_FLAG_STD_CXX11 - Success
[00:22:08] -- Performing Test HAVE_CXX_FLAG_WALL
[00:22:08] -- Performing Test HAVE_CXX_FLAG_WALL - Success
[00:22:08] -- Performing Test HAVE_CXX_FLAG_WEXTRA
[00:22:09] -- Performing Test HAVE_CXX_FLAG_WEXTRA - Success
[00:22:09] -- Performing Test HAVE_CXX_FLAG_WSHADOW
[00:22:09] -- Performing Test HAVE_CXX_FLAG_WSHADOW - Success
[00:22:09] -- Performing Test HAVE_CXX_FLAG_PEDANTIC
[00:22:10] -- Performing Test HAVE_CXX_FLAG_PEDANTIC - Success
[00:22:10] -- Performing Test HAVE_CXX_FLAG_PEDANTIC_ERRORS
[00:22:10] -- Performing Test HAVE_CXX_FLAG_PEDANTIC_ERRORS - Success
[00:22:10] -- Performing Test HAVE_CXX_FLAG_WSHORTEN_64_TO_32
[00:22:11] -- Performing Test HAVE_CXX_FLAG_WSHORTEN_64_TO_32 - Success
[00:22:11] -- Performing Test HAVE_CXX_FLAG_WFLOAT_EQUAL
[00:22:11] -- Performing Test HAVE_CXX_FLAG_WFLOAT_EQUAL - Success
[00:22:11] -- Performing Test HAVE_CXX_FLAG_FSTRICT_ALIASING
[00:22:11] -- Performing Test HAVE_CXX_FLAG_FSTRICT_ALIASING - Success
[00:22:11] -- Performing Test HAVE_CXX_FLAG_FNO_EXCEPTIONS
[00:22:12] -- Performing Test HAVE_CXX_FLAG_FNO_EXCEPTIONS - Success
[00:22:12] -- Performing Test HAVE_CXX_FLAG_WSTRICT_ALIASING
[00:22:12] -- Performing Test HAVE_CXX_FLAG_WSTRICT_ALIASING - Success
[00:22:12] -- Performing Test HAVE_CXX_FLAG_WD654
[00:22:12] -- Performing Test HAVE_CXX_FLAG_WD654 - Failed
[00:22:12] -- Performing Test HAVE_CXX_FLAG_WTHREAD_SAFETY
[00:22:13] -- Performing Test HAVE_CXX_FLAG_WTHREAD_SAFETY - Success
[00:22:13] -- Performing Test HAVE_THREAD_SAFETY_ATTRIBUTES
[00:22:13] -- Performing Test HAVE_THREAD_SAFETY_ATTRIBUTES
[00:22:13] -- Performing Test HAVE_THREAD_SAFETY_ATTRIBUTES -- failed to compile
[00:22:13] -- Performing Test HAVE_CXX_FLAG_COVERAGE
[00:22:14] -- Performing Test HAVE_CXX_FLAG_COVERAGE - Success
[00:22:14] -- Performing Test HAVE_GNU_POSIX_REGEX
[00:22:14] -- Performing Test HAVE_GNU_POSIX_REGEX
[00:22:14] -- Performing Test HAVE_GNU_POSIX_REGEX -- failed to compile
[00:22:14] -- Performing Test HAVE_POSIX_REGEX
[00:22:14] -- Performing Test HAVE_POSIX_REGEX
[00:22:15] -- Performing Test HAVE_POSIX_REGEX -- success
[00:22:15] -- Performing Test HAVE_STEADY_CLOCK
[00:22:15] -- Performing Test HAVE_STEADY_CLOCK
[00:22:15] -- Performing Test HAVE_STEADY_CLOCK -- success
[00:22:20] -- Generating done
[00:22:20] CMake Warning:
[00:22:20]   Manually-specified variables were not used by the project:
[00:22:20] 
---
[00:22:40] [  0%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackReader.cpp.o
[00:22:40] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMBuildAttrs.cpp.o
[00:22:42] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMAttributeParser.cpp.o
[00:22:42] [  0%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/AsmWriterEmitter.cpp.o
[00:22:42] [  0%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/MsgPackWriter.cpp.o
[00:22:45] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/ARMWinEH.cpp.o
[00:22:45] [  0%] Building CXX object lib/BinaryFormat/CMakeFiles/LLVMBinaryFormat.dir/Wasm.cpp.o
[00:22:46] [  0%] Building CXX object lib/Support/CMakeFiles/LLVMSupport.dir/Allocator.cpp.o
[00:22:47] [  0%] Linking CXX static library ../libLLVMBinaryFormat.a
---
[00:27:23] [ 17%] Linking CXX executable ../../bin/not
[00:27:23] [ 17%] Built target not
[00:27:24] Scanning dependencies of target yaml-bench
[00:27:24] [ 17%] Building CXX object utils/yaml-bench/CMakeFiles/yaml-bench.dir/YAMLBench.cpp.o
[00:27:24] [ 17%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/WebAssemblyStackifierEmitter.cpp.o
[00:27:26] [ 17%] Building CXX object utils/TableGen/CMakeFiles/obj.llvm-tblgen.dir/CTagsEmitter.cpp.o
[00:27:26] [ 17%] Linking CXX executable ../../bin/yaml-bench
[00:27:26] [ 17%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/UDTLayout.cpp.o
[00:27:26] [ 17%] Built target yaml-bench
---
[00:27:29] [ 17%] Built target BugpointPasses_exports
[00:27:29] Scanning dependencies of target llvm-go
[00:27:29] [ 17%] Building Go executable llvm-go
[00:27:29] [ 17%] Built target llvm-go
[00:27:30] Scanning dependencies of target benchmark
[00:27:30] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/benchmark.cc.o
[00:27:30] [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleDescriptorBuilder.cpp.o
[00:27:30] [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleDescriptorBuilder.cpp.o
[00:27:33] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/benchmark_register.cc.o
[00:27:34] [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiModuleList.cpp.o
[00:27:34] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/colorprint.cc.o
[00:27:36] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/commandlineflags.cc.o
[00:27:36] [ 18%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/DbiStream.cpp.o
[00:27:37] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/complexity.cc.o
[00:27:39] [ 18%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndex.cpp.o
[00:27:39] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/console_reporter.cc.o
[00:27:41] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/counter.cc.o
[00:27:42] [ 18%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
[00:27:42] [ 18%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeIndexDiscovery.cpp.o
[00:27:43] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/csv_reporter.cc.o
[00:27:44] [ 18%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/json_reporter.cc.o
[00:27:44] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/GlobalsStream.cpp.o
[00:27:45] [ 19%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/reporter.cc.o
[00:27:46] [ 19%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
[00:27:46] [ 19%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeHashing.cpp.o
[00:27:46] [ 19%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/sleep.cc.o
[00:27:47] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/Hash.cpp.o
[00:27:47] [ 19%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/statistics.cc.o
[00:27:48] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[00:27:48] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/HashTable.cpp.o
[00:27:50] [ 19%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/string_util.cc.o
[00:27:51] [ 19%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
[00:27:51] [ 19%] Building CXX object lib/DebugInfo/CodeView/CMakeFiles/LLVMDebugInfoCodeView.dir/TypeStreamMerger.cpp.o
[00:27:52] [ 19%] Building CXX object utils/benchmark/src/CMakeFiles/benchmark.dir/sysinfo.cc.o
[00:27:53] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/InfoStreamBuilder.cpp.o
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:292:47: error: non-constant-expression cannot be narrowed from type 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') in initializer list [-Wc++11-narrowing]
[00:27:54]   } Cases[] = {{"hw.l1dcachesize", "Data", 1, CacheCounts[1]},
[00:27:54]                                               ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:292:47: note: insert an explicit cast to silence this issue
[00:27:54]   } Cases[] = {{"hw.l1dcachesize", "Data", 1, CacheCounts[1]},
[00:27:54]                                               ^~~~~~~~~~~~~~
[00:27:54]                                               static_cast<size_t>( )
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:293:54: error: non-constant-expression cannot be narrowed from type 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') in initializer list [-Wc++11-narrowing]
[00:27:54]                {"hw.l1icachesize", "Instruction", 1, CacheCounts[1]},
[00:27:54]                                                      ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:293:54: note: insert an explicit cast to silence this issue
[00:27:54]                {"hw.l1icachesize", "Instruction", 1, CacheCounts[1]},
[00:27:54]                                                      ^~~~~~~~~~~~~~
[00:27:54]                                                      static_cast<size_t>( )
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:294:49: error: non-constant-expression cannot be narrowed from type 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') in initializer list [-Wc++11-narrowing]
[00:27:54]                {"hw.l2cachesize", "Unified", 2, CacheCounts[2]},
[00:27:54]                                                 ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:294:49: note: insert an explicit cast to silence this issue
[00:27:54]                {"hw.l2cachesize", "Unified", 2, CacheCounts[2]},
[00:27:54]                                                 ^~~~~~~~~~~~~~
[00:27:54]                                                 static_cast<size_t>( )
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:295:49: error: non-constant-expression cannot be narrowed from type 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') in initializer list [-Wc++11-narrowing]
[00:27:54]                {"hw.l3cachesize", "Unified", 3, CacheCounts[3]}};
[00:27:54]                                                 ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:295:49: note: insert an explicit cast to silence this issue
[00:27:54]                {"hw.l3cachesize", "Unified", 3, CacheCounts[3]}};
[00:27:54]                                                 ^~~~~~~~~~~~~~
[00:27:54]                                                 static_cast<size_t>( )
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:292:47: warning: implicit conversion loses integer precision: 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') [-Wshorten-64-to-32]
[00:27:54]   } Cases[] = {{"hw.l1dcachesize", "Data", 1, CacheCounts[1]},
[00:27:54]                ~                              ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:293:54: warning: implicit conversion loses integer precision: 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') [-Wshorten-64-to-32]
[00:27:54]                {"hw.l1icachesize", "Instruction", 1, CacheCounts[1]},
[00:27:54]                ~                                     ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:294:49: warning: implicit conversion loses integer precision: 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') [-Wshorten-64-to-32]
[00:27:54]                {"hw.l2cachesize", "Unified", 2, CacheCounts[2]},
[00:27:54]                ~                                ^~~~~~~~~~~~~~
[00:27:54] /Users/travis/build/rust-lang/rust/src/llvm/utils/benchmark/src/sysinfo.cc:295:49: warning: implicit conversion loses integer precision: 'std::__1::array<unsigned long long, 4>::value_type' (aka 'unsigned long long') to 'size_t' (aka 'unsigned long') [-Wshorten-64-to-32]
[00:27:54]                {"hw.l3cachesize", "Unified", 3, CacheCounts[3]}};
[00:27:54]                ~                                ^~~~~~~~~~~~~~
[00:27:54] 4 warnings and 4 errors generated.
[00:27:54] make[3]: *** [utils/benchmark/src/CMakeFiles/benchmark.dir/sysinfo.cc.o] Error 1
[00:27:54] make[2]: *** [utils/benchmark/src/CMakeFiles/benchmark.dir/all] Error 2
[00:27:54] make[2]: *** Waiting for unfinished jobs....
[00:27:54] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/ModuleDebugStream.cpp.o
[00:27:55] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeBuiltinSymbol.cpp.o
[00:27:56] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeCompilandSymbol.cpp.o
[00:27:57] [ 19%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/NativeEnumModules.cpp.o
---
[00:28:09] [ 20%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiStream.cpp.o
[00:28:10] [ 20%] Building CXX object lib/DebugInfo/PDB/CMakeFiles/LLVMDebugInfoPDB.dir/Native/TpiStreamBuilder.cpp.o
[00:28:13] [ 20%] Linking CXX static library ../../libLLVMDebugInfoPDB.a
[00:28:13] [ 20%] Built target LLVMDebugInfoPDB
[00:28:13] make[1]: *** [all] Error 2
[00:28:13] command did not execute successfully, got: exit code: 2
[00:28:13] 
[00:28:13] 
[00:28:13] build script failed, must exit now', /Users/travis/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:28:13]  finished in 414.590
[00:28:13] travis_fold:end:llvm

[00:28:13] travis_time:end:llvm:start=1535599340096105000,finish=1535599754668441000,duration=414572336000
[00:28:13] travis_time:end:llvm:start=1535599340096105000,finish=1535599754668441000,duration=414572336000

[00:28:13] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:28:13] Build completed unsuccessfully in 0:23:09
[00:28:13] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:009563ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:174d861a
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 0
drwx------+ 15 travis  staff  510 Jan 25  2018 ..
drwx------   2 travis  staff   68 Dec  6  2017 .
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:262ec600
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_time:end:262ec600:start=1535599756885646000,finish=1535599756905329000,duration=19683000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0817cc32
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a4d1c3f
travis_time:start:1a4d1c3f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:052e7890
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
