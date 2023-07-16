plain
[00:25:28] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[00:25:28] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:25:28] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.36.0-dev" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:25:28] -- The CXX compiler identification is GNU 5.4.0
[00:25:28] -- The ASM compiler identification is GNU
[00:25:28] -- Found assembler: /usr/bin/cc
[00:25:28] -- Check for working C compiler: /usr/bin/cc
---
travis_time:end:2fa701b8:start=1557894794147929014,finish=1557894794158132210,duration=10203196
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:144e7e97
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1049ce6c
travis_time:start:1049ce6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:052b8493
$ dmesg | grep -i kill
