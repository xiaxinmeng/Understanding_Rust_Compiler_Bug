plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 2fa64d0e53c15e1920bd394500da010a4cdd057b and cfe235bf5f0e22424e58b337dd38f3dc07aea4b9
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
CMAKE_x86_64-unknown-linux-gnu = None
CMAKE_x86_64_unknown_linux_gnu = None
HOST_CMAKE = None
CMAKE = None
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;LoongArch;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.64.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 5.4.0
-- The ASM compiler identification is GNU
-- Found assembler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/cc
---
-- Targeting SystemZ
-- Targeting WebAssembly
-- Targeting X86
-- Targeting AVR
CMake Error at lib/Target/CMakeLists.txt:34 (add_subdirectory):
-- Targeting LoongArch
-- Targeting M68k
  add_subdirectory given source "LoongArch" which is not an existing



CMake Error at cmake/modules/LLVM-Build.cmake:42 (get_property):
  get_property could not find TARGET LoongArch.  Perhaps it has not yet been
Call Stack (most recent call first):
  lib/CMakeLists.txt:63 (LLVMBuildGenerateCFragment)



CMake Error at cmake/modules/LLVM-Build.cmake:43 (get_property):
  get_property could not find TARGET LoongArch.  Perhaps it has not yet been
Call Stack (most recent call first):
  lib/CMakeLists.txt:63 (LLVMBuildGenerateCFragment)



-- BugpointPasses ignored -- Loadable modules not supported on this platform.
CMake Error at cmake/modules/LLVM-Config.cmake:138 (message):
  Target LoongArch is not in the set of libraries.
Call Stack (most recent call first):
  cmake/modules/LLVM-Config.cmake:263 (llvm_expand_pseudo_components)
  cmake/modules/LLVM-Config.cmake:102 (llvm_map_components_to_libnames)
  cmake/modules/LLVM-Config.cmake:95 (explicit_llvm_config)
  cmake/modules/AddLLVM.cmake:942 (llvm_config)
  cmake/modules/AddLLVM.cmake:1248 (add_llvm_executable)
  tools/llvm-gsymutil/CMakeLists.txt:11 (add_llvm_tool)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
 finished in 10.255 seconds
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
Build completed unsuccessfully in 0:01:01
cat: /tmp/toolstate/toolstates.json: No such file or directory
