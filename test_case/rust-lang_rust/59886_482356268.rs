plain
[00:01:43] Removing intermediate container 7b9f89d5ba88
[00:01:43]  ---> e1a040fbb7fc
[00:01:43] Step 4/12 : COPY scripts/musl-toolchain.sh /build/
[00:01:43]  ---> cc8010f53b3d
[00:01:43] Step 5/12 : RUN CFLAGS="-Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none"     CXXFLAGS="-Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none"     bash musl-toolchain.sh x86_64 && rm -rf build
[00:01:43] + ARCH=x86_64
[00:01:43] + TARGET=x86_64-linux-musl
[00:01:43] + OUTPUT=/usr/local
[00:01:43] + shift
[00:01:43] + shift
[00:01:43] + export 'CFLAGS=-fPIC -Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none'
[00:01:43] + CFLAGS='-fPIC -Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none'
[00:01:43] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:43] Cloning into 'musl-cross-make'...
[00:01:44] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:44] 
[00:01:44] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:44] changes and commit them, and you can discard any commits you make in this
[00:01:44] state without impacting any branches by performing another checkout.
[00:01:44] If you want to create a new branch to retain commits you create, you may
[00:01:44] If you want to create a new branch to retain commits you create, you may
[00:01:44] do so (now or later) by using -b with the checkout command again. Example:
[00:01:44]   git checkout -b <new-branch-name>
[00:01:44] 
[00:01:44] + cd musl-cross-make
[00:01:44] ++ nproc
---
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  102k    0  102k    0     0   146k      0 --:--:-- --:--:-- --:--:--  288k
[00:11:38] + mkdir libunwind-build
[00:11:38] + cd libunwind-build
[00:11:38] + cmake ../libunwind-release_70 -DLLVM_PATH=/build/llvm-release_70 -DLIBUNWIND_ENABLE_SHARED=0 -DCMAKE_C_COMPILER=x86_64-linux-musl-gcc -DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++ '-DCMAKE_C_FLAGS=-fPIC -Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none' '-DCMAKE_CXX_FLAGS=-Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none'
[00:11:38] -- The CXX compiler identification is GNU 6.3.0
[00:11:38] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
[00:11:38] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc -- works
[00:11:38] -- Detecting C compiler ABI info
---
[00:13:39] Step 8/12 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:13:39]  ---> Running in 7b318ca5d3ae
[00:13:39] Removing intermediate container 7b318ca5d3ae
[00:13:39]  ---> 74f11db0e4dd
[00:13:39] Step 9/12 : ENV CFLAGS_x86_64_unknown_linux_musl="-Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none     -Wl,--compress-debug-sections=none"
[00:13:39] Removing intermediate container 215c478584f9
[00:13:39]  ---> b6faf2a5f8be
[00:13:39] Step 10/12 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:13:39]  ---> Running in c2354282e615
---
[01:08:08] git could not determine the LLVM submodule commit hash. Assuming that an LLVM build is necessary.
[01:08:08] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-musl
[01:08:08] running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-musl" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_ENABLE_LIBXML2=OFF" "-DCMAKE_CROSSCOMPILING=True" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-tblgen" "-DLLVM_NATIVE_BUILD=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" "-DLLVM_VERSION_SUFFIX=-rust-1.35.0-nightly" "-DPYTHON_EXECUTABLE=/usr/bin/python2.7" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=x86_64-linux-musl-gcc" "-DCMAKE_CXX_COMPILER=x86_64-linux-musl-g++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static -Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -static -Wa,-mrelax-relocations=no -Wa,--compress-debug-sections=none -Wl,--compress-debug-sections=none -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-musl/llvm" "-DCMAKE_BUILD_TYPE=Release"
[01:08:08] -- The CXX compiler identification is GNU 6.3.0
[01:08:08] -- The ASM compiler identification is GNU
[01:08:08] -- Found assembler: /usr/local/bin/x86_64-linux-musl-gcc
[01:08:08] -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc
---
[02:59:14] [ 92%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/gcov.cpp.o
[02:59:14] [ 92%] Built target llvm-dwarfdump
[02:59:14] Scanning dependencies of target llvm-opt-report
[02:59:14] [ 92%] Building CXX object tools/llvm-opt-report/CMakeFiles/llvm-opt-report.dir/OptReport.cpp.o
The job exceeded the maximum time limit for jobs, and has been terminated.
