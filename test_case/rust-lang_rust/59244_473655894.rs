plain
[00:01:17] 
[00:01:17] Total download size: 4.9 M
[00:01:17] Downloading Packages:
[00:01:20] --------------------------------------------------------------------------------
[00:01:20] Total                                           1.6 MB/s | 4.9 MB     00:02     
[00:01:20] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:20] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:20] Running Transaction Test
[00:01:20] Finished Transaction Test
[00:01:20] Transaction Test Succeeded
[00:01:20] Running Transaction
---
[00:04:05] + hide_output make install
[00:04:05] + set +x
[00:04:28] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:28] + cd ..
[00:04:28] + rm -rf openssl-1.0.2k
[00:04:28] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:28] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:29]  ---> 205c02c5066c
[00:04:29] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:29]  ---> 0b72fa9c1fc8
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29]  ---> Running in dab8f4b69052
[00:04:29] + source shared.sh
[00:04:29] + VERSION=7.51.0
[00:04:29] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:31]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:31]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:32] 
  0 2509k    0  2631    0     0   1571      0  0:27:15  0:00:01  0:27:14  1571
  0 2509k    0  2631    0     0   1571      0  0:27:15  0:00:01  0:27:14  1571
 41 2509k   41 1041k    0     0   421k      0  0:00:05  0:00:02  0:00:03 1308k
100 2509k  100 2509k    0     0   844k      0  0:00:02  0:00:02 --:--:-- 1934k
[00:04:32] + mkdir curl-build
[00:04:32] + cd curl-build
[00:04:32] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:57] + hide_output make -j10
[00:04:57] + set +x
[00:05:10] shared.sh: line 1:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:10] + hide_output make install
---
 81 67.8M   81 55.0M    0     0  6543k      0  0:00:10  0:00:08  0:00:02 6919k
 93 67.8M   93 63.1M    0     0  6721k      0  0:00:10  0:00:09  0:00:01 7072k
100 67.8M  100 67.8M    0     0  6739k      0  0:00:10  0:00:10 --:--:-- 7696k
[00:09:05] + cd gcc-5.5.0
[00:09:05] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:05] --2019-03-17 08:26:32--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:05] Resolving gcc.gnu.org... 209.132.180.131
[00:09:05] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:05] HTTP request sent, awaiting response... 200 OK
---
[01:15:14]  ---> 5b3b6aac6f3d
[01:15:14] Step 25/41 : RUN ./build-clang.sh
[01:15:14]  ---> Running in 6b6b7ad4613e
[01:15:15] + source shared.sh
[01:15:15] + LLVM=llvmorg-8.0.0-rc2
[01:15:15] + cd llvm-project
[01:15:15] + cd llvm-project
[01:15:15] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:15:15] + tar xzf - --strip-components=1
[01:15:15]                                  Dload  Upload   Total   Spent    Left  Speed
[01:15:15] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    666      0 --:--:-- --:--:-- --:--:--   764
---
[01:15:32] + cd clang-build
[01:15:32] + INC=/rustroot/include
[01:15:32] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:15:32] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:32] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:02] Sun Mar 17 09:33:29 UTC 2019 - building ...
[01:16:12] + hide_output make -j10
[01:16:12] + set +x
[01:16:42] Sun Mar 17 09:34:09 UTC 2019 - building ...
---
[02:42:31]  ---> 08d9115a420f
[02:42:31] Step 32/41 : RUN ./build-perl.sh
[02:42:31]  ---> Running in e04e60e9bc9a
[02:42:32] + source shared.sh
[02:42:32] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:42:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:42:32]                                  Dload  Upload   Total   Spent    Left  Speed
[02:42:33] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 20 17.0M   20 3669k    0     0  6782k      0  0:00:02 --:--:--  0:00:02 7725k
100 17.0M  100 17.0M    0     0  15.2M      0  0:00:01  0:00:01 --:--:-- 16.1M
[02:42:33] + cd perl-5.28.0
[02:42:33] + CC=gcc
[02:42:33] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:42:33] + hide_output ./configure.gnu
[02:42:33] + set +x
[02:43:03] Sun Mar 17 11:00:30 UTC 2019 - building ...
[02:43:13] + hide_output make -j10
---
[02:53:39]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[02:54:21] [RUSTC-TIMING] syntax test:false 60.626
[02:54:21]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[02:54:49] [RUSTC-TIMING] syntax_ext test:false 27.704
The job exceeded the maximum time limit for jobs, and has been terminated.
