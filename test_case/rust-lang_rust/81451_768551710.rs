plain
-- Looking for pthread_create in pthreads - not found
-- Looking for pthread_create in pthread
-- Looking for pthread_create in pthread - found
-- Found Threads: TRUE  
-- Found ZLIB: /usr/lib/libz.so (found version "1.2.3.4") 
-- Looking for compress2 - found
-- Looking for xar_open in xar
-- Looking for xar_open in xar - not found
-- Looking for arc4random
---
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS
-- Performing Test CXX_SUPPORTS_FDATA_SECTIONS - Failed
-- Looking for os_signpost_interval_begin
-- Looking for os_signpost_interval_begin - not found
CMake Error at /rustroot/share/cmake-3.13/Modules/FindPackageHandleStandardArgs.cmake:137 (message):
Call Stack (most recent call first):
Call Stack (most recent call first):
  /rustroot/share/cmake-3.13/Modules/FindPackageHandleStandardArgs.cmake:378 (_FPHSA_FAILURE_MESSAGE)
  /rustroot/share/cmake-3.13/Modules/FindPython/Support.cmake:1130 (find_package_handle_standard_args)
  /rustroot/share/cmake-3.13/Modules/FindPython3.cmake:163 (include)
  CMakeLists.txt:700 (find_package)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit code: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 finished in 8.359 seconds
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
Build completed unsuccessfully in 0:01:19
