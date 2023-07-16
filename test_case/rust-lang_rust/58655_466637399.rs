plain
[00:00:26] +src/ci/docker/run.sh dist-x86_64-linux
[00:00:26] travis_time:end:2096103c:start=1550907634840320124,finish=1550907661130307299,duration=26289987175
travis_fold:start:build_docker
travis_time:start:24e86f39
Attempting to download s3://rust-lang-ci-sccache2/docker/7e1a0f6dbf7ee5eb0f689d3f3f0b0c18b1a9c99aaacc2a649ce33a6db602dde1d880b783741255f4fb7da1afa8a7b1e7897822d59f692a8c90e11f791375e7c7
[00:00:26] Attempting with retry: curl -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://s3-us-west-1.amazonaws.com/rust-lang-ci-sccache2/docker/7e1a0f6dbf7ee5eb0f689d3f3f0b0c18b1a9c99aaacc2a649ce33a6db602dde1d880b783741255f4fb7da1afa8a7b1e7897822d59f692a8c90e11f791375e7c7
[00:00:26]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:27] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:01:23] 
[00:01:23] Total download size: 4.9 M
[00:01:23] Downloading Packages:
[00:01:26] --------------------------------------------------------------------------------
[00:01:26] Total                                           1.6 MB/s | 4.9 MB     00:02     
[00:01:26] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:26] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:27] Running Transaction Test
[00:01:27] Finished Transaction Test
[00:01:27] Transaction Test Succeeded
[00:01:27] Running Transaction
---
[00:04:11] + hide_output make install
[00:04:11] + set +x
[00:04:35] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:35] + cd ..
[00:04:35] + rm -rf openssl-1.0.2k
[00:04:35] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:35] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:36]  ---> 5138f885b07d
[00:04:36] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:36]  ---> 182e729ad48d
[00:04:36] Step 15/41 : RUN ./build-curl.sh
[00:04:36] Step 15/41 : RUN ./build-curl.sh
[00:04:36]  ---> Running in d1c8164971f1
[00:04:36] + source shared.sh
[00:04:36] + VERSION=7.51.0
[00:04:36] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:38]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:38]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:39] 
  0 2509k    0 14215    0     0   8537      0  0:05:01  0:00:01  0:05:00  8537
  0 2509k    0 14215    0     0   8537      0  0:05:01  0:00:01  0:05:00  8537
 26 2509k   26  657k    0     0   286k      0  0:00:08  0:00:02  0:00:06 1018k
100 2509k  100 2509k    0     0   838k      0  0:00:02  0:00:02 --:--:-- 1878k
[00:04:39] + mkdir curl-build
[00:04:39] + cd curl-build
[00:04:39] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:05] + hide_output make -j10
[00:05:05] + set +x
[00:05:18] shared.sh: line 1:    14 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:18] + hide_output make install
---
 81 67.8M   81 55.3M    0     0  7071k      0  0:00:09  0:00:08  0:00:01 7564k
 94 67.8M   94 63.7M    0     0  7242k      0  0:00:09  0:00:09 --:--:-- 7764k
100 67.8M  100 67.8M    0     0  7262k      0  0:00:09  0:00:09 --:--:-- 8163k
[00:09:13] + cd gcc-5.5.0
[00:09:13] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:13] --2019-02-23 07:49:48--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:13] Resolving gcc.gnu.org... 209.132.180.131
[00:09:13] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:13] HTTP request sent, awaiting response... 200 OK
---
[01:18:12]  ---> 0aec70ad4a32
[01:18:12] Step 25/41 : RUN ./build-clang.sh
[01:18:12]  ---> Running in 0bad16f4c08c
[01:18:12] + source shared.sh
[01:18:12] + LLVM=llvmorg-8.0.0-rc2
[01:18:12] + mkdir llvm-project
[01:18:12] + cd llvm-project
[01:18:12] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:18:12] + tar xzf - --strip-components=1
[01:18:12]                                  Dload  Upload   Total   Spent    Left  Speed
[01:18:13] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    590      0 --:--:-- --:--:-- --:--:--   669
---
[01:18:31] + cd clang-build
[01:18:31] + INC=/rustroot/include
[01:18:31] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:18:31] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:18:31] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:01] Sat Feb 23 08:59:36 UTC 2019 - building ...
[01:19:11] + hide_output make -j10
[01:19:11] + set +x
[01:19:41] Sat Feb 23 09:00:16 UTC 2019 - building ...
---
[02:45:52]  ---> 26b6c2d65046
[02:45:52] Step 32/41 : RUN ./build-perl.sh
[02:45:52]  ---> Running in 9ec7ccf6dc8f
[02:45:52] + source shared.sh
[02:45:52] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:45:52]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:45:52]                                  Dload  Upload   Total   Spent    Left  Speed
[02:45:53] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  7 17.0M    7 1255k    0     0  2965k      0  0:00:05 --:--:--  0:00:05 3366k
100 17.0M  100 17.0M    0     0  14.9M      0  0:00:01  0:00:01 --:--:-- 15.7M
[02:45:53] + cd perl-5.28.0
[02:45:53] + CC=gcc
[02:45:53] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:45:53] + hide_output ./configure.gnu
[02:45:53] + set +x
[02:46:23] Sat Feb 23 10:26:58 UTC 2019 - building ...
[02:46:37] + hide_output make -j10
---
[02:56:45] [RUSTC-TIMING] rustc_errors test:false 9.802
[02:57:41] [RUSTC-TIMING] syntax test:false 55.513
[02:57:41]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[02:58:10] [RUSTC-TIMING] syntax_ext test:false 28.682
The job exceeded the maximum time limit for jobs, and has been terminated.
