plain
 95 82.1M   95 78.7M    0     0  2197k      0  0:00:38  0:00:36  0:00:02 2196k
 98 82.1M   98 81.1M    0     0  2205k      0  0:00:38  0:00:37  0:00:01 2391k
100 82.1M  100 82.1M    0     0  2209k      0  0:00:38  0:00:38 --:--:-- 2382k
[00:05:17] + cd gcc-4.8.5
[00:05:17] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:05:17] --2019-02-16 06:54:49--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:05:17] Resolving gcc.gnu.org... 209.132.180.131
[00:05:17] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:05:17] HTTP request sent, awaiting response... 200 OK
---
[00:44:03]  ---> 53189f0c88f2
[00:44:03] Step 25/41 : RUN ./build-clang.sh
[00:44:03]  ---> Running in 3151550acbdb
[00:44:04] + source shared.sh
[00:44:04] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:44:04] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:44:04] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:44:04] + mkdir clang
[00:44:04] + cd clang
[00:44:04] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:44:04] + tar xzf - --strip-components=1
[00:44:04]                                  Dload  Upload   Total   Spent    Left  Speed
[00:44:04] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    762      0 --:--:-- --:--:-- --:--:--   806
---
100 9540k    0 9540k    0     0  4283k      0 --:--:--  0:00:02 --:--:-- 5600k
100 14.6M    0 14.6M    0     0  4644k      0 --:--:--  0:00:03 --:--:-- 5521k
100 17.1M    0 17.1M    0     0  4945k      0 --:--:--  0:00:03 --:--:-- 5867k
[00:44:16] + mkdir -p tools/lld
[00:44:16] + tar zxf - --strip-components=1 -C tools/lld
[00:44:16] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:44:16]                                  Dload  Upload   Total   Spent    Left  Speed
[00:44:16] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    814      0 --:--:-- --:--:-- --:--:--   830
100   157    0   157    0     0    814      0 --:--:-- --:--:-- --:--:--   830
[00:44:16] 
100 1346k    0 1346k    0     0  2146k      0 --:--:-- --:--:-- --:--:-- 2146k
[00:44:16] + mkdir clang-build
[00:44:16] + cd clang-build
[00:44:16] + INC=/rustroot/include
[00:44:16] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:44:16] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:16] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:46] Sat Feb 16 07:34:19 UTC 2019 - building ...
[00:44:53] + hide_output make -j10
[00:44:53] + set +x
[00:45:23] Sat Feb 16 07:34:55 UTC 2019 - building ...
---
[01:54:03]  ---> fed169bab981
[01:54:03] Step 32/41 : RUN ./build-perl.sh
[01:54:03]  ---> Running in 6550ce607346
[01:54:04] + source shared.sh
[01:54:04] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:54:04]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:54:04]                                  Dload  Upload   Total   Spent    Left  Speed
[01:54:05] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:59:14] [RUSTC-TIMING] tendril test:false 1.030
[02:59:14]    Compiling clap v2.32.0
[02:59:14]    Compiling phf_generator v0.7.22
[02:59:14]    Compiling url v1.7.2
The job exceeded the maximum time limit for jobs, and has been terminated.
