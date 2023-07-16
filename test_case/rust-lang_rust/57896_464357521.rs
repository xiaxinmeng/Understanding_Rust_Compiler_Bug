plain
 93 82.1M   93 77.0M    0     0  1547k      0  0:00:54  0:00:51  0:00:03 2758k
 97 82.1M   97 79.8M    0     0  1570k      0  0:00:53  0:00:52  0:00:01 2819k
100 82.1M  100 82.1M    0     0  1592k      0  0:00:52  0:00:52 --:--:-- 2861k
[00:05:28] + cd gcc-4.8.5
[00:05:28] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:05:28] --2019-02-16 12:57:23--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:05:28] Resolving gcc.gnu.org... 209.132.180.131
[00:05:28] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:05:29] HTTP request sent, awaiting response... 200 OK
---
[00:42:26]  ---> a4e8d576cc17
[00:42:26] Step 25/41 : RUN ./build-clang.sh
[00:42:26]  ---> Running in 79a2496171af
[00:42:26] + source shared.sh
[00:42:26] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:42:26] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:42:26] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:42:26] + mkdir clang
[00:42:26] + cd clang
[00:42:26] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:42:26] + tar xzf - --strip-components=1
[00:42:26]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:27] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 8816k    0 8816k    0     0  4461k      0 --:--:--  0:00:01 --:--:-- 5694k
100 13.7M    0 13.7M    0     0  4748k      0 --:--:--  0:00:02 --:--:-- 5503k
100 17.1M    0 17.1M    0     0  5013k      0 --:--:--  0:00:03 --:--:-- 5716k
[00:42:38] + mkdir -p tools/lld
[00:42:38] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:42:38] + tar zxf - --strip-components=1 -C tools/lld
[00:42:38]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:38] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    727      0 --:--:-- --:--:-- --:--:--   740
---
[00:42:39] + cd clang-build
[00:42:39] + INC=/rustroot/include
[00:42:39] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:39] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:39] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:09] Sat Feb 16 13:35:04 UTC 2019 - building ...
[00:43:14] + hide_output make -j10
[00:43:14] + set +x
[00:43:44] Sat Feb 16 13:35:39 UTC 2019 - building ...
---
[01:50:32]  ---> 4a096348fc26
[01:50:32] Step 32/41 : RUN ./build-perl.sh
[01:50:32]  ---> Running in f043392dc0e4
[01:50:33] + source shared.sh
[01:50:33] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:50:33]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:50:33]                                  Dload  Upload   Total   Spent    Left  Speed
[01:50:34] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
