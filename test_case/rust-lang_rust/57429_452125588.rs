plain
[01:26:53] [ 55%] Built target LLVMAsmPrinter
[01:26:53] Scanning dependencies of target LLVMHello
[01:26:53] [ 55%] Building CXX object lib/Transforms/Hello/CMakeFiles/LLVMHello.dir/Hello.cpp.o
[01:26:55] [ 55%] Linking CXX shared module ../../LLVMHello.so
[01:26:55] /x-tools/x86_64-unknown-netbsd/lib/gcc/x86_64--netbsd/4.8.4/../../../../x86_64--netbsd/bin/ld: /x-tools/x86_64-unknown-netbsd/sysroot/usr/lib/libstdc++.a(new_op.o): relocation R_X86_64_32S against `_ZTVSt9bad_alloc' can not be used when making a shared object; recompile with -fPIC
[01:26:55] /x-tools/x86_64-unknown-netbsd/sysroot/usr/lib/libstdc++.a: could not read symbols: Bad value
[01:26:55] collect2: error: ld returned 1 exit status
[01:26:55] make[2]: *** [lib/LLVMHello.so] Error 1
[01:26:55] make[1]: *** [lib/Transforms/Hello/CMakeFiles/LLVMHello.dir/all] Error 2
[01:26:55] make[1]: *** Waiting for unfinished jobs....
[01:26:56] [ 55%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineFunctionPass.cpp.o
[01:26:57] [ 55%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/MachineFunctionPrinterPass.cpp.o
[01:27:00] [ 55%] Linking CXX static library ../../libLLVMInstCombine.a
[01:27:00] [ 55%] Built target LLVMInstCombine
---
[01:29:41] [ 59%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/WinEHPrepare.cpp.o
[01:29:42] [ 59%] Building CXX object lib/CodeGen/CMakeFiles/LLVMCodeGen.dir/XRayInstrumentation.cpp.o
[01:29:48] [ 59%] Linking CXX static library ../libLLVMCodeGen.a
[01:29:48] [ 59%] Built target LLVMCodeGen
[01:29:48] make: *** [all] Error 2
[01:29:48] command did not execute successfully, got: exit code: 2
[01:29:48] 
[01:29:48] 
[01:29:48] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[01:29:48]  finished in 1520.231
[01:29:48] travis_fold:end:llvm

[01:29:48] travis_time:end:llvm:start=1546903849849886532,finish=1546905370081854541,duration=1520231968009
---
travis_time:end:194365f8:start=1546905371507795050,finish=1546905371529050933,duration=21255883
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a52eec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f33ed0
travis_time:start:00f33ed0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08baf514
$ dmesg | grep -i kill
