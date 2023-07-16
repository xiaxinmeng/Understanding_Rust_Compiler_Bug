plain
[00:04:52] + hide_output make install
[00:04:52] + set +x
[00:05:11] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:11] + cd ..
[00:05:11] + rm -rf openssl-1.0.2k
[00:05:11] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:05:11] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:05:12] Removing intermediate container b26588aa1031
[00:05:12] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:05:12]  ---> c8fa79e89a9b
[00:05:12] Step 15/41 : RUN ./build-curl.sh
[00:05:12] Step 15/41 : RUN ./build-curl.sh
[00:05:12]  ---> Running in f7443690323f
[00:05:12] + source shared.sh
[00:05:12] + VERSION=7.51.0
[00:05:12] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:13]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:13]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:15] 
  0 2509k    0 14215    0     0   9291      0  0:04:36  0:00:01  0:04:35  9291
  0 2509k    0 14215    0     0   9291      0  0:04:36  0:00:01  0:04:35  9291
  2 2509k    2 59559    0     0  33498      0  0:01:16  0:00:01  0:01:15  177k
 94 2509k   94 2366k    0     0   859k      0  0:00:02  0:00:02 --:--:-- 1921k
100 2509k  100 2509k    0     0   906k      0  0:00:02  0:00:02 --:--:-- 2016k
[00:05:15] + mkdir curl-build
[00:05:15] + cd curl-build
[00:05:15] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:36] + hide_output make -j10
[00:05:36] + set +x
[00:05:48] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:48] + hide_output make install
---
 91 82.1M   91 75.0M    0     0  4034k      0  0:00:20  0:00:19  0:00:01 3754k
 96 82.1M   96 79.0M    0     0  4037k      0  0:00:20  0:00:20 --:--:-- 3687k
100 82.1M  100 82.1M    0     0  4048k      0  0:00:20  0:00:20 --:--:-- 3881k
[00:09:29] + cd gcc-4.8.5
[00:09:29] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:29] --2018-12-24 00:51:40--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:29] Resolving gcc.gnu.org... 209.132.180.131
[00:09:29] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:29] HTTP request sent, awaiting response... 200 OK
---
[00:43:11]  ---> 0640c711d7b1
[00:43:11] Step 25/41 : RUN ./build-clang.sh
[00:43:11]  ---> Running in fb8c52e36e21
[00:43:11] + source shared.sh
[00:43:11] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:43:11] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:43:11] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:43:11] + mkdir clang
[00:43:11] + cd clang
[00:43:11] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:43:11] + tar xzf - --strip-components=1
[00:43:11]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:11] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    687      0 --:--:-- --:--:-- --:--:--   718
---
[00:43:16] 
 33 17.1M   33 5934k    0     0  7104k      0  0:00:02 --:--:--  0:00:02 7104k
100 17.1M  100 17.1M    0     0  10.1M      0  0:00:01  0:00:01 --:--:-- 13.3M
[00:43:16] + mkdir -p tools/lld
[00:43:16] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:43:16] + tar zxf - --strip-components=1 -C tools/lld
[00:43:16]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:16] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:43:16] + cd clang-build
[00:43:16] + INC=/rustroot/include
[00:43:16] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:43:16] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:16] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:46] Mon Dec 24 01:25:57 UTC 2018 - building ...
[00:43:49] + hide_output make -j10
[00:43:49] + set +x
[00:44:19] Mon Dec 24 01:26:30 UTC 2018 - building ...
---
[01:45:31]  ---> 869b26fdbed3
[01:45:31] Step 32/41 : RUN ./build-perl.sh
[01:45:31]  ---> Running in b93984e71f11
[01:45:31] + source shared.sh
[01:45:31] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:45:31]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:45:31]                                  Dload  Upload   Total   Spent    Left  Speed
[01:45:32] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:54:37] [RUSTC-TIMING] memmap test:false 0.663
[02:54:38] [RUSTC-TIMING] num_cpus test:false 1.292
[02:54:43] [RUSTC-TIMING] rustc_llvm test:false 2.511

Broadcast message from root@travis-job-4e3bb012-e590-4b65-8b05-0c7329b74a43
 (unknown) at 3:41 ...
The system is going down for power off NOW!
