plain
[00:01:13] 
[00:01:13] Total download size: 4.9 M
[00:01:13] Downloading Packages:
[00:01:16] --------------------------------------------------------------------------------
[00:01:16] Total                                           1.7 MB/s | 4.9 MB     00:02     
[00:01:16] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:16] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:16] Running Transaction Test
[00:01:16] Finished Transaction Test
[00:01:16] Transaction Test Succeeded
[00:01:16] Running Transaction
---
[00:04:03] + hide_output make install
[00:04:03] + set +x
[00:04:25] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:25] + cd ..
[00:04:25] + rm -rf openssl-1.0.2k
[00:04:25] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:25] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:26]  ---> 34209d50db24
[00:04:26] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:26]  ---> 58d68df0fb9d
[00:04:26] Step 15/41 : RUN ./build-curl.sh
[00:04:26] Step 15/41 : RUN ./build-curl.sh
[00:04:26]  ---> Running in 8b1c82bb1528
[00:04:27] + source shared.sh
[00:04:27] + VERSION=7.51.0
[00:04:27] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:28]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:28]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:30] 
  0 2509k    0 14215    0     0   9267      0  0:04:37  0:00:01  0:04:36  9267
  0 2509k    0 14215    0     0   9267      0  0:04:37  0:00:01  0:04:36  9267
 28 2509k   28  704k    0     0   313k      0  0:00:08  0:00:02  0:00:06  966k
100 2509k  100 2509k    0     0   858k      0  0:00:02  0:00:02 --:--:-- 1796k
[00:04:30] + mkdir curl-build
[00:04:30] + cd curl-build
[00:04:30] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:55] + hide_output make -j10
[00:04:55] + set +x
[00:05:09] shared.sh: line 1:    15 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:09] + hide_output make install
---
 78 67.8M   78 53.4M    0     0  6857k      0  0:00:10  0:00:07  0:00:03 7353k
 90 67.8M   90 61.4M    0     0  7008k      0  0:00:09  0:00:08  0:00:01 7408k
100 67.8M  100 67.8M    0     0  7055k      0  0:00:09  0:00:09 --:--:-- 7709k
[00:09:08] + cd gcc-5.5.0
[00:09:08] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:08] --2019-02-17 23:23:08--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:08] Resolving gcc.gnu.org... 209.132.180.131
[00:09:08] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:08] HTTP request sent, awaiting response... 200 OK
---
[01:18:33]  ---> 2b93fbe595b0
[01:18:33] Step 25/41 : RUN ./build-clang.sh
[01:18:33]  ---> Running in e667474dc3dc
[01:18:34] + source shared.sh
[01:18:34] + LLVM=llvmorg-8.0.0-rc2
[01:18:34] + mkdir llvm-project
[01:18:34] + cd llvm-project
[01:18:34] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:18:34] + tar xzf - --strip-components=1
[01:18:34]                                  Dload  Upload   Total   Spent    Left  Speed
[01:18:34] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:18:52] + cd clang-build
[01:18:52] + INC=/rustroot/include
[01:18:52] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:18:52] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:18:52] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:22] Mon Feb 18 00:33:23 UTC 2019 - building ...
[01:19:33] + hide_output make -j10
[01:19:33] + set +x
[01:20:03] Mon Feb 18 00:34:03 UTC 2019 - building ...
---
[02:47:40]  ---> 0b8dc51d10c1
[02:47:40] Step 32/41 : RUN ./build-perl.sh
[02:47:40]  ---> Running in 3fa8f5600bd4
[02:47:41] + source shared.sh
[02:47:41] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:47:41]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:47:41]                                  Dload  Upload   Total   Spent    Left  Speed
[02:47:42] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 98 17.0M   98 16.8M    0     0  13.9M      0  0:00:01  0:00:01 --:--:-- 15.1M
100 17.0M  100 17.0M    0     0  14.0M      0  0:00:01  0:00:01 --:--:-- 15.2M
[02:47:42] + cd perl-5.28.0
[02:47:42] + CC=gcc
[02:47:42] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:47:42] + hide_output ./configure.gnu
[02:47:42] + set +x
[02:48:12] Mon Feb 18 02:02:12 UTC 2019 - building ...
[02:48:26] + hide_output make -j10
