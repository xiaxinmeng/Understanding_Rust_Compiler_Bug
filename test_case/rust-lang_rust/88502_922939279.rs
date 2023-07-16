plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between db1fb85cff63ad5fffe435e17128f99f9e1d970c and a0e786bda0ea13b90a09053fb6bb89090fa4bf0b
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-- Targeting SystemZ
-- Targeting WebAssembly
-- Targeting X86
-- Targeting AVR
CMake Error at lib/Target/CMakeLists.txt:32 (add_subdirectory):
-- Targeting M68k
  add_subdirectory given source "M68k" which is not an existing directory.


CMake Error at cmake/modules/LLVM-Build.cmake:42 (get_property):
  get_property could not find TARGET M68k.  Perhaps it has not yet been
Call Stack (most recent call first):
Call Stack (most recent call first):
  lib/CMakeLists.txt:61 (LLVMBuildGenerateCFragment)


CMake Error at cmake/modules/LLVM-Build.cmake:43 (get_property):
  get_property could not find TARGET M68k.  Perhaps it has not yet been
Call Stack (most recent call first):
Call Stack (most recent call first):
  lib/CMakeLists.txt:61 (LLVMBuildGenerateCFragment)

-- BugpointPasses ignored -- Loadable modules not supported on this platform.
CMake Error at cmake/modules/LLVM-Config.cmake:138 (message):
  Target M68k is not in the set of libraries.
  Target M68k is not in the set of libraries.
Call Stack (most recent call first):
  cmake/modules/LLVM-Config.cmake:256 (llvm_expand_pseudo_components)
  cmake/modules/LLVM-Config.cmake:102 (llvm_map_components_to_libnames)
  cmake/modules/LLVM-Config.cmake:95 (explicit_llvm_config)
  cmake/modules/AddLLVM.cmake:908 (llvm_config)
  cmake/modules/AddLLVM.cmake:1209 (add_llvm_executable)
  tools/llvm-gsymutil/CMakeLists.txt:11 (add_llvm_tool)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
 finished in 12.007 seconds
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
Build completed unsuccessfully in 0:01:08
cat: /tmp/toolstate/toolstates.json: No such file or directory
