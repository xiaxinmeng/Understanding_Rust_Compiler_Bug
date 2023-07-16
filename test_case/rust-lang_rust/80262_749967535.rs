plain
[TIMING] StdLink { compiler: Compiler { stage: 0, host: TargetSelection { triple: "i686-pc-windows-msvc", file: None } }, target_compiler: Compiler { stage: 0, host: TargetSelection { triple: "i686-pc-windows-msvc", file: None } }, target: TargetSelection { triple: "i686-pc-windows-msvc", file: None } } -- 0.006
[TIMING] Std { target: TargetSelection { triple: "i686-pc-windows-msvc", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "i686-pc-windows-msvc", file: None } } } -- 101.621
Building LLVM for i686-pc-windows-msvc
running: "cmake" "D:\\a\\rust\\rust\\src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=ON" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=i686" "-DLLVM_DEFAULT_TARGET_TRIPLE=i686-pc-windows-msvc" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_USE_CRT_DEBUG=MT" "-DLLVM_USE_CRT_RELEASE=MT" "-DLLVM_USE_CRT_RELWITHDEBINFO=MT" "-DLLVM_BUILD_32_BITS=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.50.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro --target=i686-pc-windows-msvc" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro --target=i686-pc-windows-msvc" "-DCMAKE_INSTALL_PREFIX=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm" "-DCMAKE_ASM_FLAGS= -nologo -MT -Brepro" "-DCMAKE_ASM_COMPILER=C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.28.29333/bin/HostX64/x86/cl.exe" "-DCMAKE_BUILD_TYPE=Release"
CMake Error: The source directory "D:/a/rust/rust/src/llvm-project/llvm" does not exist.
Specify --help for usage, or press the help button on the CMake GUI.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\cmake-0.1.44\src\lib.rs:885:5
 finished in 0.088 seconds
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/test/compile-fail --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:03:40
Build completed unsuccessfully in 0:03:40
make: *** [Makefile:73: ci-subset-1] Error 1
