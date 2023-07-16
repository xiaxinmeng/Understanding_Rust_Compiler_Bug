plain
[00:25:08] [ 23%] Building CXX object Common/CMakeFiles/lldCommon.dir/Threads.cpp.o
[00:25:08] [ 24%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/LayoutPass.cpp.o
[00:25:08] [ 25%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachOLinkingContext.cpp.o
[00:25:08] [ 26%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileBinaryReader.cpp.o
[00:25:08] /checkout/src/tools/lld/Common/ErrorHandler.cpp: In function 'void lld::exitLld(int)':
[00:25:08] /checkout/src/tools/lld/Common/ErrorHandler.cpp:52:34: error: 'class llvm::FileOutputBuffer' has no member named 'discard'
[00:25:08]      errorHandler().OutputBuffer->discard();
[00:25:08]                                   ^
[00:25:08] Common/CMakeFiles/lldCommon.dir/build.make:86: recipe for target 'Common/CMakeFiles/lldCommon.dir/ErrorHandler.cpp.o' failed
[00:25:08] CMakeFiles/Makefile2:117: recipe for target 'Common/CMakeFiles/lldCommon.dir/all' failed
[00:25:08] make[2]: *** [Common/CMakeFiles/lldCommon.dir/ErrorHandler.cpp.o] Error 1
[00:25:08] make[1]: *** [Common/CMakeFiles/lldCommon.dir/all] Error 2
[00:25:08] make[1]: *** Waiting for unfinished jobs....
[00:25:09] [ 27%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileFromAtoms.cpp.o
[00:25:09] [ 28%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileToAtoms.cpp.o
[00:25:09] [ 29%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/MachONormalizedFileYAML.cpp.o
[00:25:09] [ 30%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ObjCPass.cpp.o
[00:25:09] [ 30%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ObjCPass.cpp.o
[00:25:09] [ 30%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/ShimPass.cpp.o
[00:25:09] [ 31%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/StubsPass.cpp.o
[00:25:09] [ 32%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/TLVPass.cpp.o
[00:25:09] [ 33%] Building CXX object lib/ReaderWriter/MachO/CMakeFiles/lldMachO.dir/WriterMachO.cpp.o
[00:25:10] [ 34%] Linking CXX static library ../../liblldMachO.a
[00:25:10] [ 34%] Built target lldMachO
[00:25:10] Makefile:127: recipe for target 'all' failed
[00:25:10] make: *** [all] Error 2
[00:25:10] command did not execute successfully, got: exit code: 2
[00:25:10] 
[00:25:10] 
[00:25:10] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:25:10]  finished in 7.608
[00:25:10] travis_fold:end:lld

[00:25:10] travis_time:end:lld:start=1536209962734215211,finish=1536209970343066811,duration=7608851600
---
travis_time:end:073dd234:start=1536209971300956270,finish=1536209971310682768,duration=9726498
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:030cda60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01745760
travis_time:start:01745760
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:392d8e68
$ dmesg | grep -i kill
