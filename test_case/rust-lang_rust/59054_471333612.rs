plain
[00:01:20] 
[00:01:20] Total download size: 4.9 M
[00:01:20] Downloading Packages:
[00:01:22] --------------------------------------------------------------------------------
[00:01:22] Total                                           1.8 MB/s | 4.9 MB     00:02     
[00:01:22] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:22] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:22] Running Transaction Test
[00:01:22] Finished Transaction Test
[00:01:22] Transaction Test Succeeded
[00:01:23] Running Transaction
---
[00:04:08] + hide_output make install
[00:04:08] + set +x
[00:04:30] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:30] + cd ..
[00:04:30] + rm -rf openssl-1.0.2k
[00:04:30] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:30] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:31]  ---> f2e67aeafcd4
[00:04:31] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:32]  ---> 471a6412d298
[00:04:32] Step 15/41 : RUN ./build-curl.sh
[00:04:32] Step 15/41 : RUN ./build-curl.sh
[00:04:32]  ---> Running in 6a9f7cabb59b
[00:04:32] + source shared.sh
[00:04:32] + VERSION=7.51.0
[00:04:32] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:34]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:34]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:35] 
  0 2509k    0  1183    0     0    776      0  0:55:11  0:00:01  0:55:10   776
  0 2509k    0  1183    0     0    776      0  0:55:11  0:00:01  0:55:10   776
 56 2509k   56 1420k    0     0   570k      0  0:00:04  0:00:02  0:00:02 1466k
100 2509k  100 2509k    0     0   896k      0  0:00:02  0:00:02 --:--:-- 1969k
[00:04:35] + mkdir curl-build
[00:04:35] + cd curl-build
[00:04:35] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:02] + hide_output make -j10
[00:05:02] + set +x
[00:05:16] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:16] + hide_output make install
---
 79 67.8M   79 53.8M    0     0  6850k      0  0:00:10  0:00:08  0:00:02 7212k
 91 67.8M   91 61.8M    0     0  6992k      0  0:00:09  0:00:09 --:--:-- 7158k
100 67.8M  100 67.8M    0     0  7028k      0  0:00:09  0:00:09 --:--:-- 7616k
[00:09:14] + cd gcc-5.5.0
[00:09:14] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:14] --2019-03-10 16:08:46--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:14] Resolving gcc.gnu.org... 209.132.180.131
[00:09:14] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:14] HTTP request sent, awaiting response... 200 OK
---
[01:14:52]  ---> 59c3369741b9
[01:14:52] Step 25/41 : RUN ./build-clang.sh
[01:14:52]  ---> Running in b577d2bca641
[01:14:52] + source shared.sh
[01:14:52] + LLVM=llvmorg-8.0.0-rc2
[01:14:52] + mkdir llvm-project
[01:14:52] + cd llvm-project
[01:14:52] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:14:52] + tar xzf - --strip-components=1
[01:14:52]                                  Dload  Upload   Total   Spent    Left  Speed
[01:14:53] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    566      0 --:--:-- --:--:-- --:--:--   657
---
[01:15:11] + cd clang-build
[01:15:11] + INC=/rustroot/include
[01:15:11] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:15:11] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:11] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:42] Sun Mar 10 17:15:13 UTC 2019 - building ...
[01:15:53] + hide_output make -j10
[01:15:53] + set +x
[01:16:23] Sun Mar 10 17:15:55 UTC 2019 - building ...
---
[02:44:52]  ---> 07458566a534
[02:44:52] Step 32/41 : RUN ./build-perl.sh
[02:44:52]  ---> Running in edd48f553a04
[02:44:52] + source shared.sh
[02:44:52] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:44:53]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:44:53]                                  Dload  Upload   Total   Spent    Left  Speed
[02:44:54] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 30 17.0M   30 5357k    0     0  7827k      0  0:00:02 --:--:--  0:00:02 8959k
100 17.0M  100 17.0M    0     0  13.9M      0  0:00:01  0:00:01 --:--:-- 15.0M
[02:44:54] + cd perl-5.28.0
[02:44:54] + CC=gcc
[02:44:54] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:44:54] + hide_output ./configure.gnu
[02:44:54] + set +x
[02:45:24] Sun Mar 10 18:44:56 UTC 2019 - building ...
[02:45:39] + hide_output make -j10
