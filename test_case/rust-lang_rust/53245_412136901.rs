plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:025a6ac0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:15:58] sha256:e7bde9c01b86c01d8384167b43662bf1c0b114cfd9870c1e05c828de2956dd27
[00:15:58] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:15:58] Sending build context to Docker daemon  502.8kB
[00:15:58] Step 1/38 : FROM centos:5
[00:16:09] Get https://registry-1.docker.io/v2/library/centos/manifests/5: Get https://auth.docker.io/token?scope=repository%3Alibrary%2Fcentos%3Apull&service=registry.docker.io: net/http: TLS handshake timeout
[00:16:10] Sending build context to Docker daemon  502.8kB
[00:16:10] Step 1/38 : FROM centos:5
[00:16:11] 5: Pulling from library/centos
[00:16:11] Digest: sha256:70fffd687ff9545662c30f9043108489c698662861cd5f76070f7e2cd350564f
---
[00:16:11]  ---> 3b1a45e1fbcd
[00:16:11] Step 33/38 : ENV HOSTS x86_64-unknown-linux-gnu
[00:16:11]  ---> Using cache
[00:16:11]  ---> c6ecaea432b7
[00:16:11] Step 34/38 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       --set llvm.thin-lto=true
[00:16:11]  ---> dd101889e594
[00:16:11] Step 35/38 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:16:11]  ---> Using cache
[00:16:11]  ---> b5994262dff6
---
travis_time:start:1dd1c7f2
configure: processing command line
[00:16:11] configure: 
[00:16:11] configure: target.x86_64-unknown-linux-gnu.linker := clang
[00:16:11] configure: target.x86_64-unknown-linux-gnu.ar := /rustroot/bin/llvm-ar
[00:16:11] configure: target.x86_64-unknown-linux-gnu.ranlib := /rustroot/bin/llvm-ranlib
[00:16:11] configure: llvm.thin-lto        := True
[00:16:11] configure: build.submodules     := False
[00:16:11] configure: build.compiler-docs  := True
[00:16:11] configure: build.profiler       := True
[00:16:11] configure: build.locked-deps    := True
---
##################################################################        92.6%
######################################################################## 100.0%
[00:16:21] extracting /checkout/obj/build/cache/2018-08-01/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
[00:16:21] downloading https://static.rust-lang.org/dist/2018-08-01/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:16:51] Warning: Transient problem: timeout Will retry in 1 seconds. 3 retries left.
                                                                           0.2%
                                                                           0.2%
                                                                           0.6%Warning: Transient problem: timeout Will retry in 2 seconds. 2 retries left.
[00:17:30] Throwing away 638103 bytes
                                                                           1.1%
##########                                                                15.3%
######################                                                    31.6%
#################################                                         47.0%
---

[00:36:46] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:36:46] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:36:46] -- The CXX compiler identification is Clang 6.0.0
[00:36:46] -- The ASM compiler identification is Clang
[00:36:46] -- Found assembler: /usr/local/bin/sccache
[00:36:46] -- Check for working C compiler: /usr/local/bin/sccache
---
[00:37:15] Call Stack (most recent call first):
[00:37:15]   CMakeLists.txt:616 (include)
[00:37:15] 
[00:37:15] 
[00:37:15] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER
[00:37:15] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER - Success
[00:37:16] -- Performing Test C_SUPPORTS_FPIC - Success
[00:37:16] -- Performing Test CXX_SUPPORTS_FPIC
[00:37:16] -- Performing Test CXX_SUPPORTS_FPIC - Success
[00:37:16] -- Building with -fPIC
---

[02:20:48] travis_fold:start:llvm
travis_time:start:llvm
Building Emscripten LLVM for x86_64-unknown-linux-gnu
[02:20:48] running: "cmake" "/checkout/src/llvm-emscripten" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=JSBackend" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten" "-DCMAKE_BUILD_TYPE=Release"
[02:20:48] -- The CXX compiler identification is Clang 6.0.0
[02:20:48] -- The ASM compiler identification is Clang
[02:20:48] -- Found assembler: /usr/local/bin/sccache
[02:20:48] -- Check for working C compiler: /usr/local/bin/sccache
---
[02:48:32] [ 89%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/llvm-cov.cpp.o
[02:48:36] [ 89%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/gcov.cpp.o
[02:48:42] [ 89%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CodeCoverage.cpp.o
[02:48:57] [ 89%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageExporterJson.cpp.o
The job exceeded the maximum time limit for jobs, and has been terminated.
