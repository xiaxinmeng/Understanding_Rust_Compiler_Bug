plain
[00:22:36] [ 24%] Built target COFFOptionsTableGen
[00:22:36] [ 25%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/LayoutPass.cpp.o
[00:22:36] [ 26%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachOLinkingContext.cpp.o
[00:22:36] [ 27%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileBinaryReader.cpp.o
[00:22:37] /checkout/src/tools/lld/Common/ErrorHandler.cpp: In function 'void lld::exitLld(int)':
[00:22:37] /checkout/src/tools/lld/Common/ErrorHandler.cpp:52:34: error: 'class llvm::FileOutputBuffer' has no member named 'discard'
[00:22:37]      errorHandler().OutputBuffer->discard();
[00:22:37]                                   ^
[00:22:37] Common/CMakeFiles/lldCommon.dir/build.make:86: recipe for target 'Common/CMakeFiles/lldCommon.dir/ErrorHandler.cpp.o' failed
[00:22:37] make[2]: *** [Common/CMakeFiles/lldCommon.dir/ErrorHandler.cpp.o] Error 1
[00:22:37] CMakeFiles/Makefile2:117: recipe for target 'Common/CMakeFiles/lldCommon.dir/all' failed
[00:22:37] make[1]: *** [Common/CMakeFiles/lldCommon.dir/all] Error 2
[00:22:37] make[1]: *** Waiting for unfinished jobs....
[00:22:37] [ 28%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileFromAtoms.cpp.o
[00:22:37] [ 29%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileToAtoms.cpp.o
[00:22:37] [ 30%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileYAML.cpp.o
[00:22:37] [ 31%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ObjCPass.cpp.o
[00:22:37] [ 31%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ObjCPass.cpp.o
[00:22:37] [ 31%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ShimPass.cpp.o
[00:22:37] [ 32%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/StubsPass.cpp.o
[00:22:37] [ 33%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/TLVPass.cpp.o
[00:22:37] [ 34%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/WriterMachO.cpp.o
[00:22:38] [ 35%] Linking CXX static library ../../liblldMachO.a
[00:22:38] [ 35%] Built target lldMachO
[00:22:38] make: *** [all] Error 2
[00:22:38] Makefile:127: recipe for target 'all' failed
[00:22:38] command did not execute successfully, got: exit code: 2
[00:22:38] 
[00:22:38] 
[00:22:38] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:22:38]  finished in 6.736
[00:22:38] travis_fold:end:lld

[00:22:38] travis_time:end:lld:start=1536226291424387231,finish=1536226298160684210,duration=6736296979
---
travis_time:end:1cdcc988:start=1536226299013596953,finish=1536226299020133251,duration=6536298
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23779fb4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00ab93ff
travis_time:start:00ab93ff
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02e07ec0
$ dmesg | grep -i kill
