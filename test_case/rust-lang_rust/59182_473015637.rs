plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1b63fc9a
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:13] 
[00:01:13] Total download size: 4.9 M
[00:01:13] Downloading Packages:
[00:01:16] --------------------------------------------------------------------------------
[00:01:16] Total                                           1.8 MB/s | 4.9 MB     00:02     
[00:01:16] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:16] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:16] Running Transaction Test
[00:01:16] Finished Transaction Test
[00:01:16] Transaction Test Succeeded
[00:01:16] Running Transaction
---
[00:04:31] + set +x
[00:04:55] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:55] ./build-openssl.sh: line 15:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:55] + cd ..
[00:04:55] + rm -rf openssl-1.0.2k
[00:04:55] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:56]  ---> 79327664bf77
[00:04:56] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:57]  ---> 9ff05c80792c
[00:04:57] Step 15/41 : RUN ./build-curl.sh
[00:04:57] Step 15/41 : RUN ./build-curl.sh
[00:04:57]  ---> Running in 5831db0f1cdb
[00:04:57] + source shared.sh
[00:04:57] + VERSION=7.51.0
[00:04:57] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:59]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:59]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:00] 
  0 2509k    0 14215    0     0   9215      0  0:04:38  0:00:01  0:04:37  9215
  0 2509k    0 14215    0     0   9215      0  0:04:38  0:00:01  0:04:37  9215
 16 2509k   16  420k    0     0   194k      0  0:00:12  0:00:02  0:00:10  654k
100 2509k  100 2509k    0     0   877k      0  0:00:02  0:00:02 --:--:-- 1893k
[00:05:00] + mkdir curl-build
[00:05:00] + cd curl-build
[00:05:00] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:27] + hide_output make -j10
[00:05:27] + set +x
[00:05:42] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:42] + hide_output make install
---
 85 67.8M   85 57.7M    0     0  6237k      0  0:00:11  0:00:09  0:00:02 6050k
 96 67.8M   96 65.1M    0     0  6364k      0  0:00:10  0:00:10 --:--:-- 6972k
100 67.8M  100 67.8M    0     0  6349k      0  0:00:10  0:00:10 --:--:-- 7697k
[00:10:16] + cd gcc-5.5.0
[00:10:16] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:10:16] --2019-03-14 16:15:22--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:10:16] Resolving gcc.gnu.org... 209.132.180.131
[00:10:16] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:16] HTTP request sent, awaiting response... 200 OK
---
[01:26:35]  ---> 5b97c1979fc6
[01:26:35] Step 25/41 : RUN ./build-clang.sh
[01:26:35]  ---> Running in 0e119d003d46
[01:26:36] + source shared.sh
[01:26:36] + LLVM=llvmorg-8.0.0-rc2
[01:26:36] + mkdir llvm-project
[01:26:36] + cd llvm-project
[01:26:36] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:26:36] + tar xzf - --strip-components=1
[01:26:36]                                  Dload  Upload   Total   Spent    Left  Speed
[01:26:36] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    635      0 --:--:-- --:--:-- --:--:--   731
---
[01:26:55] + cd clang-build
[01:26:55] + INC=/rustroot/include
[01:26:55] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:26:55] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:26:55] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:27:25] Thu Mar 14 17:32:31 UTC 2019 - building ...
[01:27:38] + hide_output make -j10
[01:27:38] + set +x
[01:28:08] Thu Mar 14 17:33:15 UTC 2019 - building ...
