plain
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
-- Found Python3: /usr/bin/python3.10 (found suitable version "3.10.6", minimum required is "3.0") found components: Interpreter 
CMake Error: File /checkout/src/llvm-project/llvm/llvm.spec.in does not exist.
CMake Error at CMakeLists.txt:996 (configure_file):
  configure_file Problem configuring file

-- Linker detection: GNU ld
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Looking for __x86_64__
-- Looking for __x86_64__ - found
CMake Error at CMakeLists.txt:1111 (add_subdirectory):
  add_subdirectory given source "utils/TableGen" which is not an existing


CMake Error at cmake/modules/TableGen.cmake:10 (message):
  LLVM_TABLEGEN_EXE not set
  LLVM_TABLEGEN_EXE not set
Call Stack (most recent call first):
  include/llvm/IR/CMakeLists.txt:2 (tablegen)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
 finished in 6.994 seconds
Build completed unsuccessfully in 0:01:04
