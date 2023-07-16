plain
 94 82.1M   94 77.6M    0     0  2461k      0  0:00:34  0:00:32  0:00:02 2427k
 97 82.1M   97 80.0M    0     0  2462k      0  0:00:34  0:00:33  0:00:01 2473k
100 82.1M  100 82.1M    0     0  2461k      0  0:00:34  0:00:34 --:--:-- 2586k
[00:08:54] + cd gcc-4.8.5
[00:08:54] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:54] --2019-02-17 17:21:06--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:54] Resolving gcc.gnu.org... 209.132.180.131
[00:08:54] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:54] HTTP request sent, awaiting response... 200 OK
---
[00:48:57]  ---> 6442fd0c5fd9
[00:48:57] Step 25/41 : RUN ./build-clang.sh
[00:48:57]  ---> Running in 72ad830a7da2
[00:48:57] + source shared.sh
[00:48:57] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:48:57] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:48:57] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:48:57] + mkdir clang
[00:48:57] + cd clang
[00:48:57] + tar xzf - --strip-components=1
[00:48:57] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:48:57]                                  Dload  Upload   Total   Spent    Left  Speed
[00:48:58] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 9505k    0 9505k    0     0  4495k      0 --:--:--  0:00:02 --:--:-- 5739k
100 14.4M    0 14.4M    0     0  4762k      0 --:--:--  0:00:03 --:--:-- 5530k
100 17.1M    0 17.1M    0     0  5030k      0 --:--:--  0:00:03 --:--:-- 5808k
[00:49:14] + mkdir -p tools/lld
[00:49:14] + tar zxf - --strip-components=1 -C tools/lld
[00:49:14] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:49:14]                                  Dload  Upload   Total   Spent    Left  Speed
[00:49:14] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    616      0 --:--:-- --:--:-- --:--:--   625
---
[00:49:15] + cd clang-build
[00:49:15] + INC=/rustroot/include
[00:49:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:49:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:49:15] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:49:45] Sun Feb 17 18:01:57 UTC 2019 - building ...
[00:49:55] + hide_output make -j10
[00:49:55] + set +x
[00:50:25] Sun Feb 17 18:02:38 UTC 2019 - building ...
---
[02:01:50]  ---> 536527626913
[02:01:50] Step 32/41 : RUN ./build-perl.sh
[02:01:50]  ---> Running in 1b374a23f991
[02:01:51] + source shared.sh
[02:01:51] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:01:51]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:01:51]                                  Dload  Upload   Total   Spent    Left  Speed
[02:01:52] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:50]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[02:58:54]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[02:58:56]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[02:58:59] [RUSTC-TIMING] rustc_mir test:false 353.064
The job exceeded the maximum time limit for jobs, and has been terminated.
