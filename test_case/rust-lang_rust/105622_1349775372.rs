plain
[  0%] Building CXX object lib/ubsan/CMakeFiles/RTUbsan.osx.dir/ubsan_init.cpp.o
[  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/sanitizer_mac_libcdep.cpp.o
[  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/sanitizer_deadlock_detector1.cpp.o
[  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/sanitizer_posix_libcdep.cpp.o
clang: error: unable to execute command: Abort trap: 6
clang: error: clang frontend command failed due to signal (use -v to see invocation)
Apple clang version 13.1.6 (clang-1316.0.21.2.5)
Target: x86_64h-apple-darwin
[  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/sanitizer_deadlock_detector2.cpp.o
Thread model: posix
InstalledDir: /Applications/Xcode_13.4.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin
InstalledDir: /Applications/Xcode_13.4.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin
clang: note: diagnostic msg: Error generating preprocessed source(s) - cannot generate preprocessed source with multiple -arch options.
make[3]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/sanitizer_stack_store.cpp.o] Error 254
make[3]: *** Waiting for unfinished jobs....
make[2]: *** [lib/sanitizer_common/CMakeFiles/RTSanitizerCommonSymbolizer.osx.dir/all] Error 2
make[2]: *** Waiting for unfinished jobs....
[  0%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommonLibc.osx.dir/sanitizer_stoptheworld_linux_libcdep.cpp.o
[  0%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.osx.dir/asan_descriptions.cpp.o
[  0%] Building CXX object lib/asan/CMakeFiles/RTAsan_dynamic.osx.dir/asan_errors.cpp.o
[  0%] Built target RTSanitizerCommonCoverage.osx
---
[ 80%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/sanitizer_win.cpp.o
[ 80%] Building CXX object lib/sanitizer_common/CMakeFiles/RTSanitizerCommon.osx.dir/sanitizer_termination.cpp.o
[ 80%] Built target RTSanitizerCommon.osx
[ 80%] Built target RTAsan_dynamic.osx
make[1]: *** [lib/asan/CMakeFiles/clang_rt.asan_osx_dynamic.dir/rule] Error 2
make: *** [clang_rt.asan_osx_dynamic] Error 2
command did not execute successfully, got: exit status: 2


build script failed, must exit now', /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
 finished in 84.429 seconds
Build completed unsuccessfully in 0:20:26
