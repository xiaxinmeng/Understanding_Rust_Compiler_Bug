plain
100  150M    0  150M    0     0  5198k      0 --:--:--  0:00:29 --:--:-- 6775k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
-- The C compiler identification is GNU 5.5.0
-- The CXX compiler identification is GNU 5.5.0
-- The ASM compiler identification is GNU
---
-- Performing Test COMPILER_RT_HAS_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
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
-- For x86_64 builtins preferring x86_64/floatdidf.c to floatdidf.c
-- For x86_64 builtins preferring x86_64/floatdisf.c to floatdisf.c
-- For x86_64 builtins preferring x86_64/floatundidf.S to floatundidf.c
-- For x86_64 builtins preferring x86_64/floatundisf.S to floatundisf.c
---
-- Not building amdgpu-arch: hsa-runtime64 not found
-- LLD version: 14.0.0
-- Building BOLT runtime libraries for X86
fatal: Not a git repository (or any of the parent directories): .git
-- Targeting X86 in llvm-bolt
CMake Error at /tmp/llvm-project/bolt/tools/driver/CMakeLists.txt:36 (install):
  install PROGRAMS given no DESTINATION!

-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
100  150M    0  150M    0     0  5220k      0 --:--:--  0:00:29 --:--:-- 6705k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
-- The C compiler identification is GNU 5.5.0
-- The CXX compiler identification is GNU 5.5.0
-- The ASM compiler identification is GNU
---
-- Performing Test COMPILER_RT_HAS_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
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
-- For x86_64 builtins preferring x86_64/floatdidf.c to floatdidf.c
-- For x86_64 builtins preferring x86_64/floatdisf.c to floatdisf.c
-- For x86_64 builtins preferring x86_64/floatundidf.S to floatundidf.c
-- For x86_64 builtins preferring x86_64/floatundisf.S to floatundisf.c
---
-- Not building amdgpu-arch: hsa-runtime64 not found
-- LLD version: 14.0.0
-- Building BOLT runtime libraries for X86
fatal: Not a git repository (or any of the parent directories): .git
-- Targeting X86 in llvm-bolt
CMake Error at /tmp/llvm-project/bolt/tools/driver/CMakeLists.txt:36 (install):
  install PROGRAMS given no DESTINATION!

-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
100  150M    0  150M    0     0  5242k      0 --:--:--  0:00:29 --:--:-- 6410k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
-- The C compiler identification is GNU 5.5.0
-- The CXX compiler identification is GNU 5.5.0
-- The ASM compiler identification is GNU
---
-- Performing Test COMPILER_RT_HAS_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
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
-- For x86_64 builtins preferring x86_64/floatdidf.c to floatdidf.c
-- For x86_64 builtins preferring x86_64/floatdisf.c to floatdisf.c
-- For x86_64 builtins preferring x86_64/floatundidf.S to floatundidf.c
-- For x86_64 builtins preferring x86_64/floatundisf.S to floatundisf.c
---
-- Not building amdgpu-arch: hsa-runtime64 not found
-- LLD version: 14.0.0
-- Building BOLT runtime libraries for X86
fatal: Not a git repository (or any of the parent directories): .git
-- Targeting X86 in llvm-bolt
CMake Error at /tmp/llvm-project/bolt/tools/driver/CMakeLists.txt:36 (install):
  install PROGRAMS given no DESTINATION!

-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
100  150M    0  150M    0     0  5216k      0 --:--:--  0:00:29 --:--:-- 6414k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
-- The C compiler identification is GNU 5.5.0
-- The CXX compiler identification is GNU 5.5.0
-- The ASM compiler identification is GNU
---
-- Performing Test COMPILER_RT_HAS_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
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
-- For x86_64 builtins preferring x86_64/floatdidf.c to floatdidf.c
-- For x86_64 builtins preferring x86_64/floatdisf.c to floatdisf.c
-- For x86_64 builtins preferring x86_64/floatundidf.S to floatundidf.c
-- For x86_64 builtins preferring x86_64/floatundisf.S to floatundisf.c
---
-- Not building amdgpu-arch: hsa-runtime64 not found
-- LLD version: 14.0.0
-- Building BOLT runtime libraries for X86
fatal: Not a git repository (or any of the parent directories): .git
-- Targeting X86 in llvm-bolt
CMake Error at /tmp/llvm-project/bolt/tools/driver/CMakeLists.txt:36 (install):
  install PROGRAMS given no DESTINATION!

-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
Sending build context to Docker daemon  505.3kB

Step 1/46 : FROM ubuntu:20.04
 ---> 54c9d81cbb44
---
100  150M    0  150M    0     0  5188k      0 --:--:--  0:00:29 --:--:-- 6592k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
ERROR: An error was encountered with the build.
-- The C compiler identification is GNU 5.5.0
-- The CXX compiler identification is GNU 5.5.0
-- The ASM compiler identification is GNU
---
-- Performing Test COMPILER_RT_HAS_FLOAT16 - Failed
-- Performing Test COMPILER_RT_HAS_ASM_LSE
-- Performing Test COMPILER_RT_HAS_ASM_LSE - Failed
-- Builtin supported architectures: i386;x86_64
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
-- For x86_64 builtins preferring x86_64/floatdidf.c to floatdidf.c
-- For x86_64 builtins preferring x86_64/floatdisf.c to floatdisf.c
-- For x86_64 builtins preferring x86_64/floatundidf.S to floatundidf.c
-- For x86_64 builtins preferring x86_64/floatundisf.S to floatundisf.c
---
-- Not building amdgpu-arch: hsa-runtime64 not found
-- LLD version: 14.0.0
-- Building BOLT runtime libraries for X86
fatal: Not a git repository (or any of the parent directories): .git
-- Targeting X86 in llvm-bolt
CMake Error at /tmp/llvm-project/bolt/tools/driver/CMakeLists.txt:36 (install):
  install PROGRAMS given no DESTINATION!

-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Could NOT find Doxygen (missing: DOXYGEN_EXECUTABLE) 
-- Configuring incomplete, errors occurred!
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeOutput.log".
See also "/tmp/llvm-project/clang-build/CMakeFiles/CMakeError.log".
The command '/bin/sh -c ./build-clang.sh' returned a non-zero code: 1
##[error]Process completed with exit code 1.
Post job cleanup.
