plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4603ac31b0655793a82f110f544dc1c6abc57bb7 and f2b977f8d43ff78c9a2630c6c124a5b57de27dfe
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Detecting CXX compile features
-- Detecting CXX compile features - done
CMake Error at cmake/modules/CheckCompilerVersion.cmake:44 (message):
  Host GCC version should be at least 7.1 because LLVM will soon use new C++
  features which your toolchain version doesn't support.  Your version is
  5.4.0.  You can temporarily opt out using
  LLVM_TEMPORARILY_ALLOW_OLD_TOOLCHAIN, but very soon your toolchain won't be
Call Stack (most recent call first):
  cmake/modules/CheckCompilerVersion.cmake:49 (check_compiler_version)
  cmake/config-ix.cmake:15 (include)
  CMakeLists.txt:766 (include)
  CMakeLists.txt:766 (include)


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
 finished in 1.219 seconds
Build completed unsuccessfully in 0:01:01
cat: /tmp/toolstate/toolstates.json: No such file or directory
