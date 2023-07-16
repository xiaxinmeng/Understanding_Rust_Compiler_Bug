plain
-- Looking for __register_frame
-- Looking for __register_frame - found
-- Looking for __deregister_frame
-- Looking for __deregister_frame - found
-- Looking for __unw_add_dynamic_fde
-- Looking for __unw_add_dynamic_fde - not found
-- Looking for _Unwind_Backtrace - found
-- Looking for getpagesize
-- Looking for getpagesize - found
-- Looking for sysconf
---

CMake Error at cmake/modules/CMakeLists.txt:3 (include):
  include could not find load file:

    FindPrefixFromConfig


CMake Error at cmake/modules/CMakeLists.txt:113 (find_prefix_from_config):
  Unknown CMake command "find_prefix_from_config".


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /checkout/obj/build/tmp/distcheck/vendor/cmake/src/lib.rs:885:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:01:52
Build completed unsuccessfully in 0:01:52
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed
