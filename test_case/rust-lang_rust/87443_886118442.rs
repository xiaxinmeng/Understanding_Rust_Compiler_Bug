
$ x build
Updating only changed submodules
Submodules updated in 0.01 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
warning: x.py has made several changes recently you may want to look at
help: consider looking at the changes in `src/bootstrap/CHANGELOG.md`
note: to silence this warning, add `changelog-seen = 2` at the top of `config.toml`
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
detected home dir change, cleaning out entire build directory
running: "cmake" "/home/joshua/rustc3/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=48" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-dev" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=gcc" "-DCMAKE_CXX_COMPILER=g++" "-DCMAKE_ASM_COMPILER=gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_INSTALL_PREFIX=/home/joshua/rustc3/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
CMake Error: The source directory "/home/joshua/rustc3/src/llvm-project/llvm" does not exist.
Specify --help for usage, or press the help button on the CMake GUI.
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	finished in 0.783 seconds
Build completed unsuccessfully in 0:00:01
