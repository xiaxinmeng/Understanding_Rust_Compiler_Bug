plain
[00:04:49] + hide_output make install
[00:04:49] + set +x
[00:05:12] shared.sh: line 11:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:12] + cd ..
[00:05:12] + rm -rf openssl-1.0.2k
[00:05:12] ./build-openssl.sh: line 25:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:05:12] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:05:13]  ---> cf32abd84576
[00:05:13] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:05:14]  ---> 7d1439a9a808
[00:05:14] Step 15/41 : RUN ./build-curl.sh
[00:05:14] Step 15/41 : RUN ./build-curl.sh
[00:05:14]  ---> Running in 7751f8977b0c
[00:05:14] + source shared.sh
[00:05:14] + VERSION=7.51.0
[00:05:14] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:16]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:16]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:17] 
  0 2509k    0 14215    0     0   9357      0  0:04:34  0:00:01  0:04:33  9357
  0 2509k    0 14215    0     0   9357      0  0:04:34  0:00:01  0:04:33  9357
  8 2509k    8  220k    0     0   109k      0  0:00:22  0:00:02  0:00:20  413k
100 2509k  100 2509k    0     0   886k      0  0:00:02  0:00:02 --:--:-- 1900k
[00:05:17] + mkdir curl-build
[00:05:17] + cd curl-build
[00:05:17] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:45] + hide_output make -j10
[00:05:45] + set +x
[00:05:59] shared.sh: line 11:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:59] + hide_output make install
---
 94 82.1M   94 77.6M    0     0  2821k      0  0:00:29  0:00:28  0:00:01 2521k
 98 82.1M   98 80.5M    0     0  2824k      0  0:00:29  0:00:29 --:--:-- 2688k
100 82.1M  100 82.1M    0     0  2831k      0  0:00:29  0:00:29 --:--:-- 2924k
[00:10:32] + cd gcc-4.8.5
[00:10:32] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:10:32] --2018-12-23 21:51:41--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:10:33] Resolving gcc.gnu.org... 209.132.180.131
[00:10:33] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:33] HTTP request sent, awaiting response... 200 OK
---
[00:47:46]  ---> 54ee804f6daa
[00:47:46] Step 25/41 : RUN ./build-clang.sh
[00:47:46]  ---> Running in 6a08e954ffd5
[00:47:46] + source shared.sh
[00:47:46] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:47:46] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:47:46] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:47:46] + mkdir clang
[00:47:46] + cd clang
[00:47:46] + tar xzf - --strip-components=1
[00:47:46] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:47:46]                                  Dload  Upload   Total   Spent    Left  Speed
[00:47:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    880      0 --:--:-- --:--:-- --:--:--   929
---
100 7248k    0 7248k    0     0  4330k      0 --:--:--  0:00:01 --:--:-- 5429k
100 12.1M    0 12.1M    0     0  4594k      0 --:--:--  0:00:02 --:--:-- 5226k
100 17.1M    0 17.1M    0     0  5094k      0 --:--:--  0:00:03 --:--:-- 5678k
[00:47:59] + mkdir -p tools/lld
[00:47:59] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:47:59] + tar zxf - --strip-components=1 -C tools/lld
[00:47:59]                                  Dload  Upload   Total   Spent    Left  Speed
[00:47:59] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    843      0 --:--:-- --:--:-- --:--:--   862
---
[00:47:59] + cd clang-build
[00:47:59] + INC=/rustroot/include
[00:47:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:47:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:47:59] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:48:29] Sun Dec 23 22:29:38 UTC 2018 - building ...
[00:48:38] + hide_output make -j10
[00:48:38] + set +x
[00:49:08] Sun Dec 23 22:30:16 UTC 2018 - building ...
---
[01:54:04]  ---> e6b7c0999027
[01:54:04] Step 32/41 : RUN ./build-perl.sh
[01:54:04]  ---> Running in c23aee71840a
[01:54:05] + source shared.sh
[01:54:05] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:54:05]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:54:05]                                  Dload  Upload   Total   Spent    Left  Speed
[01:54:06] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:54:16] [RUSTC-TIMING] syntax_ext test:false 25.722
[02:58:15] [RUSTC-TIMING] rustc test:false 264.632
[02:58:15]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[02:58:15]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
The job exceeded the maximum time limit for jobs, and has been terminated.
