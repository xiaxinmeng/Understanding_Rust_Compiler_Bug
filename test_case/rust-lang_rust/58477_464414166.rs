plain
 95 82.1M   95 78.3M    0     0  3297k      0  0:00:25  0:00:24  0:00:01 3008k
 99 82.1M   99 81.5M    0     0  3294k      0  0:00:25  0:00:25 --:--:-- 3127k
100 82.1M  100 82.1M    0     0  3295k      0  0:00:25  0:00:25 --:--:-- 3217k
[00:06:22] + cd gcc-4.8.5
[00:06:22] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:06:22] --2019-02-17 01:03:05--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:06:22] Resolving gcc.gnu.org... 209.132.180.131
[00:06:22] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:06:22] HTTP request sent, awaiting response... 200 OK
---
[00:42:02]  ---> bd2ee558172f
[00:42:02] Step 25/41 : RUN ./build-clang.sh
[00:42:02]  ---> Running in b241bcb01c08
[00:42:02] + source shared.sh
[00:42:02] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:42:02] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:42:02] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:42:02] + mkdir clang
[00:42:02] + cd clang
[00:42:02] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:42:02] + tar xzf - --strip-components=1
[00:42:02]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:03] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    761      0 --:--:-- --:--:-- --:--:--   793
---
100 9504k    0 9504k    0     0  4580k      0 --:--:--  0:00:02 --:--:-- 5772k
100 14.6M    0 14.6M    0     0  4889k      0 --:--:--  0:00:03 --:--:-- 5651k
100 17.1M    0 17.1M    0     0  5141k      0 --:--:--  0:00:03 --:--:-- 5915k
[00:42:14] + mkdir -p tools/lld
[00:42:14] + tar zxf - --strip-components=1 -C tools/lld
[00:42:14] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:42:14]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:14] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    764      0 --:--:-- --:--:-- --:--:--   773
100   157    0   157    0     0    764      0 --:--:-- --:--:-- --:--:--   773
[00:42:15] 
100 1346k    0 1346k    0     0  2218k      0 --:--:-- --:--:-- --:--:-- 2218k
[00:42:15] + mkdir clang-build
[00:42:15] + cd clang-build
[00:42:15] + INC=/rustroot/include
[00:42:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:15] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:45] Sun Feb 17 01:39:28 UTC 2019 - building ...
[00:42:49] + hide_output make -j10
[00:42:49] + set +x
[00:43:19] Sun Feb 17 01:40:02 UTC 2019 - building ...
---
[01:47:15]  ---> f11c40de0a94
[01:47:15] Step 32/41 : RUN ./build-perl.sh
[01:47:15]  ---> Running in 4d7bc871db7c
[01:47:15] + source shared.sh
[01:47:15] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:47:15]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:47:15]                                  Dload  Upload   Total   Spent    Left  Speed
[01:47:16] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:01] [RUSTC-TIMING] rand test:false 4.174
[02:58:01]    Compiling tempfile v3.0.5
[02:58:02] [RUSTC-TIMING] tempfile test:false 1.007
[02:58:02]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
The job exceeded the maximum time limit for jobs, and has been terminated.
