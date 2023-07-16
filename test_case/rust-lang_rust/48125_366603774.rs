
Building LLD for i686-pc-windows-gnu
running: "cmake" "C:\\projects\\rust\\src/tools/lld" "-G" "Ninja" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=gcc.exe" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=g++.exe" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -m32 -fno-omit-frame-pointer" "-DLLVM_CONFIG_PATH=C:\\projects\\rust\\build\\i686-pc-windows-gnu\\llvm\\bin\\llvm-config.exe" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_INSTALL_PREFIX=C:\\projects\\rust\\build\\i686-pc-windows-gnu\\lld" "-DCMAKE_BUILD_TYPE=Release"
-- The C compiler identification is GNU 6.3.0
-- The CXX compiler identification is GNU 6.3.0
-- Check for working C compiler: C:/projects/rust/sccache.exe
-- Check for working C compiler: C:/projects/rust/sccache.exe -- works
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Detecting C compile features
-- Detecting C compile features - done
-- Check for working CXX compiler: C:/projects/rust/sccache.exe
-- Check for working CXX compiler: C:/projects/rust/sccache.exe -- works
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Detecting CXX compile features
-- Detecting CXX compile features - done
-- Performing Test C_SUPPORTS_WERROR_DATE_TIME
CMake Error at C:/projects/rust/build/i686-pc-windows-gnu/lld/build/CMakeFiles/CMakeTmp/CMakeLists.txt:2 (set):
  Syntax error in cmake code at
    C:/projects/rust/build/i686-pc-windows-gnu/lld/build/CMakeFiles/CMakeTmp/CMakeLists.txt:2
  when parsing string
    C:\projects\rust\build\i686-pc-windows-gnu\llvm/lib/cmake/llvm
  Invalid character escape '\p'.
CMake Error at C:/Program Files (x86)/CMake/share/cmake-3.10/Modules/CheckCSourceCompiles.cmake:97 (try_compile):
  Failed to configure test project build system.
Call Stack (most recent call first):
  C:/Program Files (x86)/CMake/share/cmake-3.10/Modules/CheckCCompilerFlag.cmake:49 (CHECK_C_SOURCE_COMPILES)
  C:/projects/rust/build/i686-pc-windows-gnu/llvm/lib/cmake/llvm/HandleLLVMOptions.cmake:178 (check_c_compiler_flag)
  C:/projects/rust/build/i686-pc-windows-gnu/llvm/lib/cmake/llvm/HandleLLVMOptions.cmake:405 (add_flag_if_supported)
  CMakeLists.txt:56 (include)
-- Configuring incomplete, errors occurred!
See also "C:/projects/rust/build/i686-pc-windows-gnu/lld/build/CMakeFiles/CMakeOutput.log".
thread 'main' panicked at '
command did not execute successfully, got: exit code: 1
build script failed, must exit now', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.29\src\lib.rs:632:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
	finished in 4.645
failed to run: C:\projects\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:39:19
Command exited with code 1
