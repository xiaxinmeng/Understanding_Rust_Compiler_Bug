plain
 95 82.1M   95 78.8M    0     0  2864k      0  0:00:29  0:00:28  0:00:01 2650k
 99 82.1M   99 81.8M    0     0  2871k      0  0:00:29  0:00:29 --:--:-- 2753k
100 82.1M  100 82.1M    0     0  2872k      0  0:00:29  0:00:29 --:--:-- 2865k
[00:04:55] + cd gcc-4.8.5
[00:04:55] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:04:55] --2019-02-16 18:58:45--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:04:55] Resolving gcc.gnu.org... 209.132.180.131
[00:04:55] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:04:55] HTTP request sent, awaiting response... 200 OK
---
[00:43:51]  ---> 9ce4b7a9a6ef
[00:43:51] Step 25/41 : RUN ./build-clang.sh
[00:43:51]  ---> Running in b1f891c99a31
[00:43:51] + source shared.sh
[00:43:51] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:43:51] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:43:51] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:43:51] + mkdir clang
[00:43:51] + cd clang
[00:43:51] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:43:51] + tar xzf - --strip-components=1
[00:43:51]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:51] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    849      0 --:--:-- --:--:-- --:--:--   902
---
100 6944k    0 6944k    0     0  4076k      0 --:--:--  0:00:01 --:--:-- 5588k
100 12.1M    0 12.1M    0     0  4505k      0 --:--:--  0:00:02 --:--:-- 5391k
100 17.1M    0 17.1M    0     0  5021k      0 --:--:--  0:00:03 --:--:-- 5804k
[00:44:03] + mkdir -p tools/lld
[00:44:03] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:44:03] + tar zxf - --strip-components=1 -C tools/lld
[00:44:03]                                  Dload  Upload   Total   Spent    Left  Speed
[00:44:03] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:44:04] + cd clang-build
[00:44:04] + INC=/rustroot/include
[00:44:04] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:44:04] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:04] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:34] Sat Feb 16 19:38:24 UTC 2019 - building ...
[00:44:41] + hide_output make -j10
[00:44:41] + set +x
[00:45:11] Sat Feb 16 19:39:02 UTC 2019 - building ...
---
[01:53:59]  ---> 591d3f4a941f
[01:53:59] Step 32/41 : RUN ./build-perl.sh
[01:53:59]  ---> Running in 032aee80d640
[01:54:00] + source shared.sh
[01:54:00] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:54:00]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:54:00]                                  Dload  Upload   Total   Spent    Left  Speed
[01:54:01] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:46]    Compiling serde_json v1.0.33
[02:58:46]    Compiling pest_derive v1.0.8
[02:58:46] [RUSTC-TIMING] num_integer test:false 1.426
[02:58:46]    Compiling syn v0.15.22
The job exceeded the maximum time limit for jobs, and has been terminated.
