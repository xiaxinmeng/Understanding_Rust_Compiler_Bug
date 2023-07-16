plain
[00:01:17] 
[00:01:17] Total download size: 4.9 M
[00:01:17] Downloading Packages:
[00:01:29] --------------------------------------------------------------------------------
[00:01:29] Total                                           439 kB/s | 4.9 MB     00:11     
[00:01:29] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:29] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:29] Running Transaction Test
[00:01:29] Finished Transaction Test
[00:01:29] Transaction Test Succeeded
[00:01:29] Running Transaction
---
[00:04:14] + hide_output make install
[00:04:14] + set +x
[00:04:36] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:36] + cd ..
[00:04:36] + rm -rf openssl-1.0.2k
[00:04:36] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:36] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:37]  ---> 6c395a40d237
[00:04:37] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:37]  ---> 11f2d64886af
[00:04:37] Step 15/41 : RUN ./build-curl.sh
[00:04:37] Step 15/41 : RUN ./build-curl.sh
[00:04:37]  ---> Running in 14eaba2064ac
[00:04:38] + source shared.sh
[00:04:38] + VERSION=7.51.0
[00:04:38] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:39]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:39]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:40] 
  0 2509k    0  1183    0     0    830      0  0:51:36  0:00:01  0:51:35   830
  0 2509k    0  1183    0     0    830      0  0:51:36  0:00:01  0:51:35   830
  8 2509k    8  220k    0     0   114k      0  0:00:21  0:00:01  0:00:20  441k
100 2509k  100 2509k    0     0   918k      0  0:00:02  0:00:02 --:--:-- 1919k
[00:04:40] + mkdir curl-build
[00:04:40] + cd curl-build
[00:04:40] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:06] + hide_output make -j10
[00:05:06] + set +x
[00:05:20] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:20] + hide_output make install
---
 79 67.8M   79 54.2M    0     0  6919k      0  0:00:10  0:00:08  0:00:02 7526k
 92 67.8M   92 62.4M    0     0  7083k      0  0:00:09  0:00:09 --:--:-- 7584k
100 67.8M  100 67.8M    0     0  7118k      0  0:00:09  0:00:09 --:--:-- 7914k
[00:09:18] + cd gcc-5.5.0
[00:09:18] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:18] --2019-02-16 03:57:27--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:18] Resolving gcc.gnu.org... 209.132.180.131
[00:09:18] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:18] HTTP request sent, awaiting response... 200 OK
---
[01:14:45]  ---> a85d5a08e225
[01:14:45] Step 25/41 : RUN ./build-clang.sh
[01:14:45]  ---> Running in eb02239e2819
[01:14:45] + source shared.sh
[01:14:45] + LLVM=llvmorg-8.0.0-rc2
[01:14:45] + mkdir llvm-project
[01:14:45] + cd llvm-project
[01:14:45] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:14:45] + tar xzf - --strip-components=1
[01:14:45]                                  Dload  Upload   Total   Spent    Left  Speed
[01:14:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    637      0 --:--:-- --:--:-- --:--:--   743
---
[01:15:03] + cd clang-build
[01:15:03] + INC=/rustroot/include
[01:15:03] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:15:03] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:03] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:33] Sat Feb 16 05:03:42 UTC 2019 - building ...
[01:15:43] + hide_output make -j10
[01:15:43] + set +x
[01:16:13] Sat Feb 16 05:04:22 UTC 2019 - building ...
---
[02:43:59]  ---> 06b64764a3d4
[02:43:59] Step 32/41 : RUN ./build-perl.sh
[02:43:59]  ---> Running in f6f7b8703bf2
[02:43:59] + source shared.sh
[02:43:59] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:44:00]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:44:00]                                  Dload  Upload   Total   Spent    Left  Speed
[02:44:01] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 24 17.0M   24 4304k    0     0  7253k      0  0:00:02 --:--:--  0:00:02 8325k
100 17.0M  100 17.0M    0     0  14.8M      0  0:00:01  0:00:01 --:--:-- 15.8M
[02:44:01] + cd perl-5.28.0
[02:44:01] + CC=gcc
[02:44:01] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:44:01] + hide_output ./configure.gnu
[02:44:01] + set +x
[02:44:31] Sat Feb 16 06:32:40 UTC 2019 - building ...
[02:44:43] + hide_output make -j10
---
[02:54:38] [RUSTC-TIMING] rustc_errors test:false 9.628
[02:55:35] [RUSTC-TIMING] syntax test:false 56.150
[02:55:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[02:56:03] [RUSTC-TIMING] syntax_ext test:false 28.808
The job exceeded the maximum time limit for jobs, and has been terminated.
