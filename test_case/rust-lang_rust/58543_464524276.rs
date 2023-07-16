plain
 90 67.8M   90 61.6M    0     0  4592k      0  0:00:15  0:00:13  0:00:02 3052k
 95 67.8M   95 64.8M    0     0  4517k      0  0:00:15  0:00:14  0:00:01 3004k
100 67.8M  100 67.8M    0     0  4477k      0  0:00:15  0:00:15 --:--:-- 3004k
[00:05:26] + cd gcc-5.5.0
[00:05:26] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:05:26] --2019-02-17 20:18:34--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:05:26] Resolving gcc.gnu.org... 209.132.180.131
[00:05:26] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:05:26] HTTP request sent, awaiting response... 200 OK
---
[01:12:14]  ---> 2815d4665045
[01:12:14] Step 25/41 : RUN ./build-clang.sh
[01:12:14]  ---> Running in 3042937e24ac
[01:12:14] + source shared.sh
[01:12:14] + LLVM=llvmorg-8.0.0-rc2
[01:12:14] + mkdir llvm-project
[01:12:14] + cd llvm-project
[01:12:14] + tar xzf - --strip-components=1
[01:12:14] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:12:14]                                  Dload  Upload   Total   Spent    Left  Speed
[01:12:14] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    573      0 --:--:-- --:--:-- --:--:--   647
---
[01:12:32] + cd clang-build
[01:12:32] + INC=/rustroot/include
[01:12:32] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:12:32] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:12:32] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:13:02] Sun Feb 17 21:26:10 UTC 2019 - building ...
[01:13:11] + hide_output make -j10
[01:13:11] + set +x
[01:13:42] Sun Feb 17 21:26:49 UTC 2019 - building ...
---
[02:37:38]  ---> 9c7bb51d146f
[02:37:38] Step 32/41 : RUN ./build-perl.sh
[02:37:38]  ---> Running in a1f0522e21a8
[02:37:39] + source shared.sh
[02:37:39] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:37:39]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:37:39]                                  Dload  Upload   Total   Spent    Left  Speed
[02:37:41] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:01 --:--:--     0
  0 17.0M    0  2174    0     0   1534      0  3:14:38  0:00:01  3:14:37  1593
100 17.0M  100 17.0M    0     0  7887k      0  0:00:02  0:00:02 --:--:-- 8084k
[02:37:41] + cd perl-5.28.0
[02:37:41] + CC=gcc
[02:37:41] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:37:41] + hide_output ./configure.gnu
[02:37:41] + set +x
[02:38:11] Sun Feb 17 22:51:19 UTC 2019 - building ...
[02:38:22] + hide_output make -j10
---
[02:55:47] [RUSTC-TIMING] rustc test:false 338.044
[02:55:47]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[02:55:47]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[02:57:17] [RUSTC-TIMING] rustc_incremental test:false 90.237
The job exceeded the maximum time limit for jobs, and has been terminated.
