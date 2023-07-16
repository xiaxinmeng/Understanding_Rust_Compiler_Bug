plain
  0     0    0  170M    0     0  5111k      0 --:--:--  0:00:34 --:--:-- 6671k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -C ../clang/cmake/caches/PGO.cmake -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
loading initial cache file ../clang/cmake/caches/PGO.cmake
CMake Warning at CMakeLists.txt:5 (message):
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
---
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
-- Performing additional configure checks with target flags: -m32
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16 - Failed
-- For i386 builtins preferring i386/fp_mode.c to fp_mode.c
-- For i386 builtins preferring i386/ashldi3.S to ashldi3.c
-- For i386 builtins preferring i386/ashrdi3.S to ashrdi3.c
-- For i386 builtins preferring i386/divdi3.S to divdi3.c
-- For i386 builtins preferring i386/floatdidf.S to floatdidf.c
-- For i386 builtins preferring i386/floatdisf.S to floatdisf.c
-- For i386 builtins preferring i386/floatundidf.S to floatundidf.c
-- For i386 builtins preferring i386/floatundisf.S to floatundisf.c
-- For i386 builtins preferring i386/lshrdi3.S to lshrdi3.c
-- For i386 builtins preferring i386/moddi3.S to moddi3.c
-- For i386 builtins preferring i386/muldi3.S to muldi3.c
-- For i386 builtins preferring i386/udivdi3.S to udivdi3.c
-- For i386 builtins preferring i386/umoddi3.S to umoddi3.c
-- For i386 builtins preferring i386/floatdixf.S to floatdixf.c
-- For i386 builtins preferring i386/floatundixf.S to floatundixf.c
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16 - Failed
---
-- Looking for include file sys/inotify.h
-- Looking for include file sys/inotify.h - found
-- Setting next clang stage to: stage2-instrumented
-- LLD version: 16.0.0
-- Building BOLT runtime libraries for X86
-- Targeting X86 in llvm-bolt
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
CMake Error at /usr/share/cmake3/Modules/ExternalProject.cmake:3217 (add_custom_target):
  add_custom_target cannot create target "builtins" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt/lib/builtins".
  See documentation for policy CMP0002 for more details.
  cmake/modules/LLVMExternalProjectUtils.cmake:294 (ExternalProject_Add)
  runtimes/CMakeLists.txt:72 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:129 (builtin_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "compiler-rt" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt".  See
  documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt" because
  another target with the same name already exists.  The existing target is a
  custom target created in source directory "/tmp/llvm-project/compiler-rt".
  See documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt-stripped"
  because another target with the same name already exists.  The existing
  target is a custom target created in source directory
  "/tmp/llvm-project/compiler-rt".  See documentation for policy CMP0002 for
  more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
Command failed. Attempt 2/5:
Sending build context to Docker daemon  423.9kB

Step 1/27 : FROM centos:7
---
  0     0    0  170M    0     0  5105k      0 --:--:--  0:00:34 --:--:-- 6450k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -C ../clang/cmake/caches/PGO.cmake -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
loading initial cache file ../clang/cmake/caches/PGO.cmake
CMake Warning at CMakeLists.txt:5 (message):
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
---
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
-- Performing additional configure checks with target flags: -m32
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16 - Failed
-- For i386 builtins preferring i386/fp_mode.c to fp_mode.c
-- For i386 builtins preferring i386/ashldi3.S to ashldi3.c
-- For i386 builtins preferring i386/ashrdi3.S to ashrdi3.c
-- For i386 builtins preferring i386/divdi3.S to divdi3.c
-- For i386 builtins preferring i386/floatdidf.S to floatdidf.c
-- For i386 builtins preferring i386/floatdisf.S to floatdisf.c
-- For i386 builtins preferring i386/floatundidf.S to floatundidf.c
-- For i386 builtins preferring i386/floatundisf.S to floatundisf.c
-- For i386 builtins preferring i386/lshrdi3.S to lshrdi3.c
-- For i386 builtins preferring i386/moddi3.S to moddi3.c
-- For i386 builtins preferring i386/muldi3.S to muldi3.c
-- For i386 builtins preferring i386/udivdi3.S to udivdi3.c
-- For i386 builtins preferring i386/umoddi3.S to umoddi3.c
-- For i386 builtins preferring i386/floatdixf.S to floatdixf.c
-- For i386 builtins preferring i386/floatundixf.S to floatundixf.c
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16 - Failed
---
-- Looking for include file sys/inotify.h
-- Looking for include file sys/inotify.h - found
-- Setting next clang stage to: stage2-instrumented
-- LLD version: 16.0.0
-- Building BOLT runtime libraries for X86
-- Targeting X86 in llvm-bolt
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
CMake Error at /usr/share/cmake3/Modules/ExternalProject.cmake:3217 (add_custom_target):
  add_custom_target cannot create target "builtins" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt/lib/builtins".
  See documentation for policy CMP0002 for more details.
  cmake/modules/LLVMExternalProjectUtils.cmake:294 (ExternalProject_Add)
  runtimes/CMakeLists.txt:72 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:129 (builtin_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "compiler-rt" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt".  See
  documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt" because
  another target with the same name already exists.  The existing target is a
  custom target created in source directory "/tmp/llvm-project/compiler-rt".
  See documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt-stripped"
  because another target with the same name already exists.  The existing
  target is a custom target created in source directory
  "/tmp/llvm-project/compiler-rt".  See documentation for policy CMP0002 for
  more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
Command failed. Attempt 3/5:
Sending build context to Docker daemon  423.9kB

Step 1/27 : FROM centos:7
---
100  170M  100  170M    0     0  19.5M      0  0:00:08  0:00:08 --:--:-- 19.9M
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -C ../clang/cmake/caches/PGO.cmake -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
loading initial cache file ../clang/cmake/caches/PGO.cmake
CMake Warning at CMakeLists.txt:5 (message):
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
---
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
-- Performing additional configure checks with target flags: -m32
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16 - Failed
-- For i386 builtins preferring i386/fp_mode.c to fp_mode.c
-- For i386 builtins preferring i386/ashldi3.S to ashldi3.c
-- For i386 builtins preferring i386/ashrdi3.S to ashrdi3.c
-- For i386 builtins preferring i386/divdi3.S to divdi3.c
-- For i386 builtins preferring i386/floatdidf.S to floatdidf.c
-- For i386 builtins preferring i386/floatdisf.S to floatdisf.c
-- For i386 builtins preferring i386/floatundidf.S to floatundidf.c
-- For i386 builtins preferring i386/floatundisf.S to floatundisf.c
-- For i386 builtins preferring i386/lshrdi3.S to lshrdi3.c
-- For i386 builtins preferring i386/moddi3.S to moddi3.c
-- For i386 builtins preferring i386/muldi3.S to muldi3.c
-- For i386 builtins preferring i386/udivdi3.S to udivdi3.c
-- For i386 builtins preferring i386/umoddi3.S to umoddi3.c
-- For i386 builtins preferring i386/floatdixf.S to floatdixf.c
-- For i386 builtins preferring i386/floatundixf.S to floatundixf.c
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16 - Failed
---
-- Looking for include file sys/inotify.h
-- Looking for include file sys/inotify.h - found
-- Setting next clang stage to: stage2-instrumented
-- LLD version: 16.0.0
-- Building BOLT runtime libraries for X86
-- Targeting X86 in llvm-bolt
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
CMake Error at /usr/share/cmake3/Modules/ExternalProject.cmake:3217 (add_custom_target):
  add_custom_target cannot create target "builtins" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt/lib/builtins".
  See documentation for policy CMP0002 for more details.
  cmake/modules/LLVMExternalProjectUtils.cmake:294 (ExternalProject_Add)
  runtimes/CMakeLists.txt:72 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:129 (builtin_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "compiler-rt" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt".  See
  documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt" because
  another target with the same name already exists.  The existing target is a
  custom target created in source directory "/tmp/llvm-project/compiler-rt".
  See documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt-stripped"
  because another target with the same name already exists.  The existing
  target is a custom target created in source directory
  "/tmp/llvm-project/compiler-rt".  See documentation for policy CMP0002 for
  more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
Command failed. Attempt 4/5:
Sending build context to Docker daemon  423.9kB

Step 1/27 : FROM centos:7
---
  0     0    0  170M    0     0  5110k      0 --:--:--  0:00:34 --:--:-- 6587k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -C ../clang/cmake/caches/PGO.cmake -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
loading initial cache file ../clang/cmake/caches/PGO.cmake
CMake Warning at CMakeLists.txt:5 (message):
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
---
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
-- Performing additional configure checks with target flags: -m32
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16 - Failed
-- For i386 builtins preferring i386/fp_mode.c to fp_mode.c
-- For i386 builtins preferring i386/ashldi3.S to ashldi3.c
-- For i386 builtins preferring i386/ashrdi3.S to ashrdi3.c
-- For i386 builtins preferring i386/divdi3.S to divdi3.c
-- For i386 builtins preferring i386/floatdidf.S to floatdidf.c
-- For i386 builtins preferring i386/floatdisf.S to floatdisf.c
-- For i386 builtins preferring i386/floatundidf.S to floatundidf.c
-- For i386 builtins preferring i386/floatundisf.S to floatundisf.c
-- For i386 builtins preferring i386/lshrdi3.S to lshrdi3.c
-- For i386 builtins preferring i386/moddi3.S to moddi3.c
-- For i386 builtins preferring i386/muldi3.S to muldi3.c
-- For i386 builtins preferring i386/udivdi3.S to udivdi3.c
-- For i386 builtins preferring i386/umoddi3.S to umoddi3.c
-- For i386 builtins preferring i386/floatdixf.S to floatdixf.c
-- For i386 builtins preferring i386/floatundixf.S to floatundixf.c
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16 - Failed
---
-- Looking for include file sys/inotify.h
-- Looking for include file sys/inotify.h - found
-- Setting next clang stage to: stage2-instrumented
-- LLD version: 16.0.0
-- Building BOLT runtime libraries for X86
-- Targeting X86 in llvm-bolt
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
CMake Error at /usr/share/cmake3/Modules/ExternalProject.cmake:3217 (add_custom_target):
  add_custom_target cannot create target "builtins" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt/lib/builtins".
  See documentation for policy CMP0002 for more details.
  cmake/modules/LLVMExternalProjectUtils.cmake:294 (ExternalProject_Add)
  runtimes/CMakeLists.txt:72 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:129 (builtin_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "compiler-rt" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt".  See
  documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt" because
  another target with the same name already exists.  The existing target is a
  custom target created in source directory "/tmp/llvm-project/compiler-rt".
  See documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt-stripped"
  because another target with the same name already exists.  The existing
  target is a custom target created in source directory
  "/tmp/llvm-project/compiler-rt".  See documentation for policy CMP0002 for
  more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
Command failed. Attempt 5/5:
Sending build context to Docker daemon  423.9kB

Step 1/27 : FROM centos:7
---
  0     0    0  170M    0     0  5119k      0 --:--:--  0:00:34 --:--:-- 6656k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -C ../clang/cmake/caches/PGO.cmake -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
loading initial cache file ../clang/cmake/caches/PGO.cmake
CMake Warning at CMakeLists.txt:5 (message):
  Your CMake version is 3.17.5.  Starting with LLVM 17.0.0, the minimum
---
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
-- Performing additional configure checks with target flags: -m32
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16
-- Performing Test COMPILER_RT_HAS_i386_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16
-- Performing Test COMPILER_RT_HAS_i386_BFLOAT16 - Failed
-- For i386 builtins preferring i386/fp_mode.c to fp_mode.c
-- For i386 builtins preferring i386/ashldi3.S to ashldi3.c
-- For i386 builtins preferring i386/ashrdi3.S to ashrdi3.c
-- For i386 builtins preferring i386/divdi3.S to divdi3.c
-- For i386 builtins preferring i386/floatdidf.S to floatdidf.c
-- For i386 builtins preferring i386/floatdisf.S to floatdisf.c
-- For i386 builtins preferring i386/floatundidf.S to floatundidf.c
-- For i386 builtins preferring i386/floatundisf.S to floatundisf.c
-- For i386 builtins preferring i386/lshrdi3.S to lshrdi3.c
-- For i386 builtins preferring i386/moddi3.S to moddi3.c
-- For i386 builtins preferring i386/muldi3.S to muldi3.c
-- For i386 builtins preferring i386/udivdi3.S to udivdi3.c
-- For i386 builtins preferring i386/umoddi3.S to umoddi3.c
-- For i386 builtins preferring i386/floatdixf.S to floatdixf.c
-- For i386 builtins preferring i386/floatundixf.S to floatundixf.c
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16
-- Performing Test COMPILER_RT_HAS_x86_64_BFLOAT16 - Failed
---
-- Looking for include file sys/inotify.h
-- Looking for include file sys/inotify.h - found
-- Setting next clang stage to: stage2-instrumented
-- LLD version: 16.0.0
-- Building BOLT runtime libraries for X86
-- Targeting X86 in llvm-bolt
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
CMake Error at /usr/share/cmake3/Modules/ExternalProject.cmake:3217 (add_custom_target):
  add_custom_target cannot create target "builtins" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt/lib/builtins".
  See documentation for policy CMP0002 for more details.
  cmake/modules/LLVMExternalProjectUtils.cmake:294 (ExternalProject_Add)
  runtimes/CMakeLists.txt:72 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:129 (builtin_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "compiler-rt" because another target
  with the same name already exists.  The existing target is a custom target
  created in source directory "/tmp/llvm-project/compiler-rt".  See
  documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt" because
  another target with the same name already exists.  The existing target is a
  custom target created in source directory "/tmp/llvm-project/compiler-rt".
  See documentation for policy CMP0002 for more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



CMake Error at cmake/modules/LLVMExternalProjectUtils.cmake:371 (add_custom_target):
  add_custom_target cannot create target "install-compiler-rt-stripped"
  because another target with the same name already exists.  The existing
  target is a custom target created in source directory
  "/tmp/llvm-project/compiler-rt".  See documentation for policy CMP0002 for
  more details.
  runtimes/CMakeLists.txt:226 (llvm_ExternalProject_Add)
  runtimes/CMakeLists.txt:388 (runtime_default_target)



-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command has failed after 5 attempts.
##[error]Process completed with exit code 1.
Post job cleanup.
