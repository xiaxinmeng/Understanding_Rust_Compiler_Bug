plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between d5f9c40e6a9ecc62432e71e886cef83a4c2c9b98 and 48d29252375d4c46759ade4db0eeb0f1456be5d8
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    Finished release [optimized] target(s) in 50.93s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.60.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
CMake Error: The source directory "/checkout/src/llvm-project/llvm" does not exist.
Specify --help for usage, or press the help button on the CMake GUI.
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:00:51
cat: /tmp/toolstate/toolstates.json: No such file or directory
