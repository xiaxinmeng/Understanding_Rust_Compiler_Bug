plain
CMAKE_x86_64-unknown-linux-gnu = None
CMAKE_x86_64_unknown_linux_gnu = None
HOST_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/compiler-rt" "-G" "Ninja" "-DCMAKE_C_COMPILER_TARGET=x86_64-unknown-linux-gnu" "-DCOMPILER_RT_BUILD_BUILTINS=OFF" "-DCOMPILER_RT_BUILD_CRT=OFF" "-DCOMPILER_RT_BUILD_LIBFUZZER=OFF" "-DCOMPILER_RT_BUILD_PROFILE=OFF" "-DCOMPILER_RT_BUILD_SANITIZERS=ON" "-DCOMPILER_RT_BUILD_XRAY=OFF" "-DCOMPILER_RT_DEFAULT_TARGET_ONLY=ON" "-DCOMPILER_RT_USE_LIBCXX=OFF" "-DLLVM_CONFIG_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/llvm-config" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=cc" "-DCMAKE_CXX_COMPILER=c++" "-DCMAKE_ASM_COMPILER=cc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -fembed-bitcode=off" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC -m64 -fembed-bitcode=off" "-DCMAKE_SHARED_LINKER_FLAGS=" "-DCMAKE_MODULE_LINKER_FLAGS=" "-DCMAKE_EXE_LINKER_FLAGS=" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 9.4.0
-- The ASM compiler identification is GNU
-- Found assembler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/cc
-- Check for working C compiler: /usr/bin/cc
CMake Error at /usr/share/cmake-3.16/Modules/CMakeTestCCompiler.cmake:60 (message):
-- Check for working C compiler: /usr/bin/cc -- broken

    "/usr/bin/cc"

  is not able to compile a simple test program.
  is not able to compile a simple test program.

  It fails with the following output:

    Change Dir: /checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers/build/CMakeFiles/CMakeTmp
    
    Run Build Command(s):/usr/bin/ninja cmTC_9815b && [1/2] Building C object CMakeFiles/cmTC_9815b.dir/testCCompiler.c.o
    FAILED: CMakeFiles/cmTC_9815b.dir/testCCompiler.c.o 
    /usr/bin/cc   -ffunction-sections -fdata-sections -fPIC -m64 -fembed-bitcode=off -o CMakeFiles/cmTC_9815b.dir/testCCompiler.c.o   -c testCCompiler.c
    cc: error: unrecognized command line option ‘-fembed-bitcode=off’
    ninja: build stopped: subcommand failed.
    

  


  CMake will not be able to correctly generate this project.
Call Stack (most recent call first):
  CMakeLists.txt:10 (project)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.48/src/lib.rs:975:5
 finished in 0.346 seconds
Build completed successfully in 0:03:49
