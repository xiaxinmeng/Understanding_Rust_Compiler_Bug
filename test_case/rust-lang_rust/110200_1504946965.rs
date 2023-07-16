plain
CMAKE_x86_64-unknown-linux-gnu = None
CMAKE_x86_64_unknown_linux_gnu = None
HOST_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_UNREACHABLE_OPTIMIZE=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;LoongArch;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_WARNINGS=OFF" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.70.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
  Your CMake version is 3.16.3.  Starting with LLVM 17.0.0, the minimum
  version of CMake required to build LLVM will become 3.20.0, and using an
  older CMake will become an error.  Please upgrade your CMake to at least
  3.20.0 now to avoid issues in the future!
---
CMAKE_riscv64gc-unknown-linux-gnu = None
CMAKE_riscv64gc_unknown_linux_gnu = None
TARGET_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/riscv64gc-unknown-linux-gnu/llvm/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_UNREACHABLE_OPTIMIZE=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;LoongArch;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=riscv64" "-DLLVM_DEFAULT_TARGET_TRIPLE=riscv64-unknown-linux-gnu" "-DLLVM_ENABLE_WARNINGS=OFF" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-tblgen" "-DLLVM_NM=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-nm" "-DLLVM_CONFIG_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "-DLLVM_VERSION_SUFFIX=-rust-1.70.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_CROSSCOMPILING=True" "-DCMAKE_SYSTEM_NAME=Linux" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=riscv64-unknown-linux-gnu-gcc" "-DCMAKE_CXX_COMPILER=riscv64-unknown-linux-gnu-g++" "-DCMAKE_ASM_COMPILER=riscv64-unknown-linux-gnu-gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -march=rv64gc -mabi=lp64d -mcmodel=medany" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -march=rv64gc -mabi=lp64d -mcmodel=medany" "-DCMAKE_SHARED_LINKER_FLAGS= -latomic -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -latomic -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/riscv64gc-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -march=rv64gc -mabi=lp64d -mcmodel=medany" "-DCMAKE_BUILD_TYPE=Release"
  Your CMake version is 3.16.3.  Starting with LLVM 17.0.0, the minimum
  version of CMake required to build LLVM will become 3.20.0, and using an
  older CMake will become an error.  Please upgrade your CMake to at least
  3.20.0 now to avoid issues in the future!
---
[RUSTC-TIMING] rustc_workspace_hack test:false 0.017
   Compiling rls v2.0.0 (/checkout/src/tools/rls)
[RUSTC-TIMING] rls test:false 0.707
    Finished release [optimized] target(s) in 28.23s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  hashbrown 0.12.3 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/libhashbrown-114c669348e46308.rlib"
    `cargo` additionally enabled features {"raw"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/libhashbrown-d1f45968f8ef79e1.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:250:13
Build completed unsuccessfully in 0:17:28
