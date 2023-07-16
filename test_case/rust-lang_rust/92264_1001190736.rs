plain
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Success
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
-- Found Python3: /usr/bin/python3.5 (found suitable version "3.5.2", minimum required is "3.0") found components:  Interpreter 
CMake Error: File /checkout/src/llvm-project/llvm/llvm.spec.in does not exist.
CMake Error at CMakeLists.txt:862 (configure_file):
  configure_file Problem configuring file

-- Linker detection: GNU ld
-- Performing Test HAS_WERROR_GLOBAL_CTORS
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
-- Performing Test HAS_WERROR_GLOBAL_CTORS - Failed
CMake Error at CMakeLists.txt:968 (add_subdirectory):
  add_subdirectory given source "utils/TableGen" which is not an existing


CMake Error at cmake/modules/TableGen.cmake:9 (message):
  LLVM_TABLEGEN_EXE not set
  LLVM_TABLEGEN_EXE not set
Call Stack (most recent call first):
  include/llvm/IR/CMakeLists.txt:2 (tablegen)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 7.192 seconds
Build completed unsuccessfully in 0:01:05
