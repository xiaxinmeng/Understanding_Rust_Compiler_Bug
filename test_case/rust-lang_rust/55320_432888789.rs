plain
[00:22:19] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for i686-unknown-linux-gnu
[00:22:19] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_BUILD_32_BITS=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=cc" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m32 -march=i686" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m32 -march=i686" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/i686-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:22:19] CMake Error: The source directory "/checkout/src/llvm" does not appear to contain CMakeLists.txt.
[00:22:19] Specify --help for usage, or press the help button on the CMake GUI.
[00:22:19] command did not execute successfully, got: exit code: 1
[00:22:19] 
[00:22:19] 
[00:22:19] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:22:19]  finished in 0.013
[00:22:19] travis_fold:end:llvm

[00:22:19] travis_time:end:llvm:start=1540433282102550664,finish=1540433282116294096,duration=13743432
[00:22:19] travis_time:end:llvm:start=1540433282102550664,finish=1540433282116294096,duration=13743432

[00:22:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:19] Build completed unsuccessfully in 0:17:54
[00:22:19] Makefile:28: recipe for target 'all' failed
[00:22:19] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1937ad22
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24651068:start=1540433282783730804,finish=1540433282789852959,duration=6122155
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07ee4e67
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bec05c0
travis_time:start:1bec05c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33cb664e
$ dmesg | grep -i kill
