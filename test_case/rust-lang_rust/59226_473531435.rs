plain
[00:01:12] 
[00:01:12] Total download size: 4.9 M
[00:01:12] Downloading Packages:
[00:01:15] --------------------------------------------------------------------------------
[00:01:15] Total                                           1.8 MB/s | 4.9 MB     00:02     
[00:01:15] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:15] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:15] Running Transaction Test
[00:01:15] Finished Transaction Test
[00:01:15] Transaction Test Succeeded
[00:01:15] Running Transaction
---
[00:04:33] + hide_output make install
[00:04:33] + set +x
[00:04:58] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:58] + cd ..
[00:04:58] + rm -rf openssl-1.0.2k
[00:04:58] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:58] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:59]  ---> 0db687a7aaa1
[00:04:59] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:59]  ---> 54c0c0190fbd
[00:04:59] Step 15/41 : RUN ./build-curl.sh
[00:04:59] Step 15/41 : RUN ./build-curl.sh
[00:04:59]  ---> Running in 84cae35775cc
[00:05:00] + source shared.sh
[00:05:00] + VERSION=7.51.0
[00:05:00] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:01]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:01]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:03] 
  0 2509k    0 14215    0     0   9292      0  0:04:36  0:00:01  0:04:35  9292
  0 2509k    0 14215    0     0   9292      0  0:04:36  0:00:01  0:04:35  9292
 27 2509k   27  699k    0     0   307k      0  0:00:08  0:00:02  0:00:06  923k
100 2509k  100 2509k    0     0   845k      0  0:00:02  0:00:02 --:--:-- 1734k
[00:05:03] + mkdir curl-build
[00:05:03] + cd curl-build
[00:05:03] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:33] Sat Mar 16 10:51:59 UTC 2019 - building ...
[00:05:33] + hide_output make -j10
[00:05:33] + set +x
[00:05:48] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
---
 81 67.8M   81 55.4M    0     0  6242k      0  0:00:11  0:00:09  0:00:02 6400k
 93 67.8M   93 63.3M    0     0  6422k      0  0:00:10  0:00:10 --:--:-- 6788k
100 67.8M  100 67.8M    0     0  6403k      0  0:00:10  0:00:10 --:--:-- 7468k
[00:10:14] + cd gcc-5.5.0
[00:10:14] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:10:14] --2019-03-16 10:56:41--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:10:15] Resolving gcc.gnu.org... 209.132.180.131
[00:10:15] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:15] HTTP request sent, awaiting response... 200 OK
---
[01:22:51]  ---> 552fbf314283
[01:22:51] Step 25/41 : RUN ./build-clang.sh
[01:22:51]  ---> Running in 60da9823dd5f
[01:22:51] + source shared.sh
[01:22:51] + LLVM=llvmorg-8.0.0-rc2
[01:22:51] + mkdir llvm-project
[01:22:51] + cd llvm-project
[01:22:51] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:22:51] + tar xzf - --strip-components=1
[01:22:51]                                  Dload  Upload   Total   Spent    Left  Speed
[01:22:51] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    637      0 --:--:-- --:--:-- --:--:--   739
---
[01:23:09] + cd clang-build
[01:23:09] + INC=/rustroot/include
[01:23:09] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:23:09] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:23:09] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:23:39] Sat Mar 16 12:10:06 UTC 2019 - building ...
[01:23:54] + hide_output make -j10
[01:23:54] + set +x
[01:24:24] Sat Mar 16 12:10:51 UTC 2019 - building ...
---
[02:58:02] Step 32/41 : RUN ./build-perl.sh
[02:58:02]  ---> Running in 1e508b4593ac
[02:58:02] + source shared.sh
[02:58:02] + tar xzf -
[02:58:02] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:58:03]                                  Dload  Upload   Total   Spent    Left  Speed
[02:58:04] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 96 17.0M   96 16.4M    0     0  13.7M      0  0:00:01  0:00:01 --:--:-- 15.0M
100 17.0M  100 17.0M    0     0  13.8M      0  0:00:01  0:00:01 --:--:-- 15.1M
[02:58:04] + cd perl-5.28.0
[02:58:04] + CC=gcc
[02:58:04] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:58:04] + hide_output ./configure.gnu
[02:58:04] + set +x
[02:58:34] Sat Mar 16 13:45:00 UTC 2019 - building ...
[02:58:53] + hide_output make -j10
[02:58:53] + hide_output make -j10
[02:58:53] + set +x
The job exceeded the maximum time limit for jobs, and has been terminated.
