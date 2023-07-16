plain
[00:04:13] + hide_output make install
[00:04:13] + set +x
[00:04:30] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:30] + cd ..
[00:04:30] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:30] + rm -rf openssl-1.0.2k
[00:04:30] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:31] Removing intermediate container b933f01476bf
[00:04:31] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:31]  ---> cc5407687805
[00:04:31] Step 15/41 : RUN ./build-curl.sh
[00:04:31] Step 15/41 : RUN ./build-curl.sh
[00:04:31]  ---> Running in ddb98e4a1ec3
[00:04:31] + source shared.sh
[00:04:31] + VERSION=7.51.0
[00:04:31] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:32]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:34] 
  0 2509k    0 14215    0     0   9172      0  0:04:40  0:00:01  0:04:39  9172
  0 2509k    0 14215    0     0   9172      0  0:04:40  0:00:01  0:04:39  9172
  8 2509k    8  202k    0     0    98k      0  0:00:25  0:00:02  0:00:23  379k
100 2509k  100 2509k    0     0   927k      0  0:00:02  0:00:02 --:--:-- 2160k
[00:04:34] + mkdir curl-build
[00:04:34] + cd curl-build
[00:04:34] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:52] + hide_output make -j10
[00:04:52] + set +x
[00:05:04] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:04] + hide_output make install
---
 92 82.1M   92 75.6M    0     0  3772k      0  0:00:22  0:00:20  0:00:02 3416k
 96 82.1M   96 79.5M    0     0  3780k      0  0:00:22  0:00:21  0:00:01 3435k
100 82.1M  100 82.1M    0     0  3777k      0  0:00:22  0:00:22 --:--:-- 3670k
[00:08:39] + cd gcc-4.8.5
[00:08:39] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:39] --2018-12-23 18:48:58--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:40] Resolving gcc.gnu.org... 209.132.180.131
[00:08:40] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:40] HTTP request sent, awaiting response... 200 OK
---
[00:41:36]  ---> 08028ed70d3a
[00:41:36] Step 25/41 : RUN ./build-clang.sh
[00:41:36]  ---> Running in 96b19586af78
[00:41:36] + source shared.sh
[00:41:36] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:41:36] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:41:36] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:41:36] + mkdir clang
[00:41:36] + cd clang
[00:41:36] + tar xzf - --strip-components=1
[00:41:36] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:41:36]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:37] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    921      0 --:--:-- --:--:-- --:--:--   969
---
100 10.4M    0 10.4M    0     0  4636k      0 --:--:--  0:00:02 --:--:-- 5419k
100 16.0M    0 16.0M    0     0  4982k      0 --:--:--  0:00:03 --:--:-- 5541k
100 17.1M    0 17.1M    0     0  5177k      0 --:--:--  0:00:03 --:--:-- 5746k
[00:41:49] + mkdir -p tools/lld
[00:41:49] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:41:49] + tar zxf - --strip-components=1 -C tools/lld
[00:41:49]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:49] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    911      0 --:--:-- --:--:-- --:--:--   923
100   157    0   157    0     0    911      0 --:--:-- --:--:-- --:--:--   923
[00:41:49] 
100 1346k    0 1346k    0     0  2360k      0 --:--:-- --:--:-- --:--:-- 2360k
[00:41:49] + mkdir clang-build
[00:41:49] + cd clang-build
[00:41:49] + INC=/rustroot/include
[00:41:49] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:49] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:49] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:19] Sun Dec 23 19:22:38 UTC 2018 - building ...
[00:42:22] + hide_output make -j10
[00:42:22] + set +x
[00:42:52] Sun Dec 23 19:23:11 UTC 2018 - building ...
---
[01:43:15]  ---> 9d9f7f2a6f6b
[01:43:15] Step 32/41 : RUN ./build-perl.sh
[01:43:15]  ---> Running in 9519602b9356
[01:43:15] + source shared.sh
[01:43:15] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:43:15]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:43:15]                                  Dload  Upload   Total   Spent    Left  Speed
[01:43:17] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:40]    Compiling quote v0.6.10
[02:58:41] [RUSTC-TIMING] jobserver test:false 4.201
[02:58:42]    Compiling serde_json v1.0.33

Broadcast message from root@travis-job-32ecb7a7-f2f5-4370-b488-9854cca9a527
 (unknown) at 21:39 ...
The system is going down for power off NOW!
[02:58:53] 
[02:58:54] Session terminated, killing shell...[RUSTC-TIMING] serde_json test:false 12.002
