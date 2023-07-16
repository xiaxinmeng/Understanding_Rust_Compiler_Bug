plain
[00:01:36] 
[00:01:36] Total download size: 4.9 M
[00:01:36] Downloading Packages:
[00:01:39] --------------------------------------------------------------------------------
[00:01:39] Total                                           1.6 MB/s | 4.9 MB     00:02     
[00:01:39] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:39] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:39] Running Transaction Test
[00:01:39] Finished Transaction Test
[00:01:39] Transaction Test Succeeded
[00:01:39] Running Transaction
---
[00:05:09] + hide_output make install
[00:05:09] + set +x
[00:05:32] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:32] + cd ..
[00:05:32] + rm -rf openssl-1.0.2k
[00:05:32] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:05:32] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:05:33]  ---> 73f471e3c451
[00:05:33] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:05:33]  ---> ae3ce200fe3e
[00:05:33] Step 15/41 : RUN ./build-curl.sh
[00:05:33] Step 15/41 : RUN ./build-curl.sh
[00:05:33]  ---> Running in 0d5092356bab
[00:05:33] + source shared.sh
[00:05:33] + VERSION=7.51.0
[00:05:33] + tar xjf -
[00:05:33] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:35]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:36] 
  0 2509k    0 14215    0     0   9242      0  0:04:38  0:00:01  0:04:37  9242
 34 2509k   34  875k    0     0   376k      0  0:00:06  0:00:02  0:00:04 1096k
 34 2509k   34  875k    0     0   376k      0  0:00:06  0:00:02  0:00:04 1096k
100 2509k  100 2509k    0     0   884k      0  0:00:02  0:00:02 --:--:-- 1922k
[00:05:36] + mkdir curl-build
[00:05:36] + cd curl-build
[00:05:36] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:06:03] + hide_output make -j10
[00:06:03] + set +x
[00:06:16] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:06:16] + hide_output make install
---
 89 67.8M   89 60.6M    0     0  6378k      0  0:00:10  0:00:09  0:00:01 6275k
 98 67.8M   98 66.9M    0     0  6380k      0  0:00:10  0:00:10 --:--:-- 6796k
100 67.8M  100 67.8M    0     0  6363k      0  0:00:10  0:00:10 --:--:-- 6789k
[00:10:59] + cd gcc-5.5.0
[00:10:59] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:10:59] --2019-02-18 11:32:25--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:10:59] Resolving gcc.gnu.org... 209.132.180.131
[00:10:59] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:59] HTTP request sent, awaiting response... 200 OK
---
[01:16:30]  ---> 8f0b1cb2430a
[01:16:30] Step 25/41 : RUN ./build-clang.sh
[01:16:30]  ---> Running in ad0067fbc26d
[01:16:31] + source shared.sh
[01:16:31] + LLVM=llvmorg-8.0.0-rc2
[01:16:31] + mkdir llvm-project
[01:16:31] + cd llvm-project
[01:16:31] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:16:31] + tar xzf - --strip-components=1
[01:16:31]                                  Dload  Upload   Total   Spent    Left  Speed
[01:16:31] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    555      0 --:--:-- --:--:-- --:--:--   623
---
[01:16:49] + cd clang-build
[01:16:49] + INC=/rustroot/include
[01:16:49] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:16:49] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:49] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:17:19] Mon Feb 18 12:38:45 UTC 2019 - building ...
[01:17:31] + hide_output make -j10
[01:17:31] + set +x
[01:18:01] Mon Feb 18 12:39:27 UTC 2019 - building ...
---
[02:44:38]  ---> 63df0ad639c8
[02:44:38] Step 32/41 : RUN ./build-perl.sh
[02:44:38]  ---> Running in c89156415ae9
[02:44:39] + source shared.sh
[02:44:39] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:44:39]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:44:39]                                  Dload  Upload   Total   Spent    Left  Speed
[02:44:40] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 57 17.0M   57  9.8M    0     0  10.3M      0  0:00:01 --:--:--  0:00:01 11.4M
100 17.0M  100 17.0M    0     0  12.8M      0  0:00:01  0:00:01 --:--:-- 13.8M
[02:44:40] + cd perl-5.28.0
[02:44:40] + CC=gcc
[02:44:40] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:44:40] + hide_output ./configure.gnu
[02:44:40] + set +x
[02:45:10] Mon Feb 18 14:06:36 UTC 2019 - building ...
[02:45:28] + hide_output make -j10
---
[02:55:57] [RUSTC-TIMING] rustc_errors test:false 9.363
[02:56:53] [RUSTC-TIMING] syntax test:false 55.758
[02:56:53]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[02:57:21] [RUSTC-TIMING] syntax_ext test:false 28.099
The job exceeded the maximum time limit for jobs, and has been terminated.
