plain
100 59.8M  100 59.8M    0     0  6871k      0  0:00:08  0:00:08 --:--:-- 8200k
+ cd gcc-7.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2022-05-12 11:17:40 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2022-05-12 11:17:41 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2022-05-12 11:17:41 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2022-05-12 11:17:41 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.16.1.tar.bz2 [1626446/1626446] -> "./isl-0.16.1.tar.bz2" [1]
gmp-6.1.0.tar.bz2: OK
mpfr-3.1.4.tar.bz2: OK
mpc-1.0.3.tar.gz: OK
isl-0.16.1.tar.bz2: OK
All prerequisites downloaded successfully.
+ cd ../gcc-build
+ hide_output ../gcc-7.5.0/configure --prefix=/rustroot --enable-languages=c,c++ --disable-gnu-unique-object
+ set +x
++ nproc
---
100  150M    0  150M    0     0  5182k      0 --:--:--  0:00:29 --:--:-- 6462k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Thu May 12 11:53:39 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
 ---> 51940677a908
Step 36/48 : RUN ./build-ninja.sh
 ---> Running in d4e808b64ba2
+ source shared.sh
+ NINJA=v1.10.2
+ mkdir ninja
+ cd ninja
+ curl -L https://github.com/ninja-build/ninja/archive/refs/tags/v1.10.2.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0

100  208k  100  208k    0     0  1330k      0 --:--:-- --:--:-- --:--:-- 1330k
+ mkdir ninja-build
+ cd ninja-build
+ hide_output cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot
+ set +x
ERROR: An error was encountered with the build.
CMake Error at CMakeLists.txt:1 (cmake_minimum_required):
  CMake 3.15 or higher is required.  You are running version 3.13.4


-- Configuring incomplete, errors occurred!
The command '/bin/sh -c ./build-ninja.sh' returned a non-zero code: 1
Sending build context to Docker daemon  514.6kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
 ---> 51940677a908
Step 36/48 : RUN ./build-ninja.sh
 ---> Running in 73f2903d1a03
+ source shared.sh
+ NINJA=v1.10.2
+ mkdir ninja
+ cd ninja
+ curl -L https://github.com/ninja-build/ninja/archive/refs/tags/v1.10.2.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0

100  208k  100  208k    0     0  3798k      0 --:--:-- --:--:-- --:--:-- 3798k
+ mkdir ninja-build
+ cd ninja-build
+ hide_output cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot
+ set +x
ERROR: An error was encountered with the build.
CMake Error at CMakeLists.txt:1 (cmake_minimum_required):
  CMake 3.15 or higher is required.  You are running version 3.13.4


-- Configuring incomplete, errors occurred!
The command '/bin/sh -c ./build-ninja.sh' returned a non-zero code: 1
Sending build context to Docker daemon  514.6kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
 ---> 51940677a908
Step 36/48 : RUN ./build-ninja.sh
 ---> Running in bdaa75b9ba02
+ source shared.sh
+ NINJA=v1.10.2
+ mkdir ninja
+ cd ninja
+ curl -L https://github.com/ninja-build/ninja/archive/refs/tags/v1.10.2.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0

100  208k  100  208k    0     0  1392k      0 --:--:-- --:--:-- --:--:-- 1392k
+ mkdir ninja-build
+ cd ninja-build
+ hide_output cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot
+ set +x
ERROR: An error was encountered with the build.
CMake Error at CMakeLists.txt:1 (cmake_minimum_required):
  CMake 3.15 or higher is required.  You are running version 3.13.4


-- Configuring incomplete, errors occurred!
The command '/bin/sh -c ./build-ninja.sh' returned a non-zero code: 1
Sending build context to Docker daemon  514.6kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
 ---> 51940677a908
Step 36/48 : RUN ./build-ninja.sh
 ---> Running in bb489c021e2b
+ source shared.sh
+ NINJA=v1.10.2
+ mkdir ninja
+ cd ninja
+ curl -L https://github.com/ninja-build/ninja/archive/refs/tags/v1.10.2.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0

100  208k  100  208k    0     0  2296k      0 --:--:-- --:--:-- --:--:-- 2296k
+ mkdir ninja-build
+ cd ninja-build
+ hide_output cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot
+ set +x
ERROR: An error was encountered with the build.
CMake Error at CMakeLists.txt:1 (cmake_minimum_required):
  CMake 3.15 or higher is required.  You are running version 3.13.4


-- Configuring incomplete, errors occurred!
The command '/bin/sh -c ./build-ninja.sh' returned a non-zero code: 1
Sending build context to Docker daemon  514.6kB

Step 1/48 : FROM ubuntu:20.04
 ---> 53df61775e88
---
 ---> 51940677a908
Step 36/48 : RUN ./build-ninja.sh
 ---> Running in cb26b5027ca8
+ source shared.sh
+ NINJA=v1.10.2
+ mkdir ninja
+ cd ninja
+ curl -L https://github.com/ninja-build/ninja/archive/refs/tags/v1.10.2.tar.gz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
+ mkdir ninja-build
+ cd ninja-build
+ hide_output cmake .. -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot
+ set +x
ERROR: An error was encountered with the build.
CMake Error at CMakeLists.txt:1 (cmake_minimum_required):
  CMake 3.15 or higher is required.  You are running version 3.13.4


-- Configuring incomplete, errors occurred!
The command '/bin/sh -c ./build-ninja.sh' returned a non-zero code: 1
##[error]Process completed with exit code 1.
Post job cleanup.
