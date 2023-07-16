plain
[00:01:19] 
[00:01:19] Total download size: 4.9 M
[00:01:19] Downloading Packages:
[00:01:22] --------------------------------------------------------------------------------
[00:01:22] Total                                           1.7 MB/s | 4.9 MB     00:02     
[00:01:22] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:22] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:22] Running Transaction Test
[00:01:22] Finished Transaction Test
[00:01:22] Transaction Test Succeeded
[00:01:22] Running Transaction
---
[00:04:10] + hide_output make install
[00:04:10] + set +x
[00:04:32] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:32] + cd ..
[00:04:32] + rm -rf openssl-1.0.2k
[00:04:32] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:32] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:33]  ---> b87b33f9242c
[00:04:33] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:34]  ---> 914d7cfebe10
[00:04:34] Step 15/41 : RUN ./build-curl.sh
[00:04:34] Step 15/41 : RUN ./build-curl.sh
[00:04:34]  ---> Running in 529827efc392
[00:04:34] + source shared.sh
[00:04:34] + VERSION=7.51.0
[00:04:34] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:36]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:36]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:37] 
  0 2509k    0 14215    0     0   8439      0  0:05:04  0:00:01  0:05:03  8439
  0 2509k    0 14215    0     0   8439      0  0:05:04  0:00:01  0:05:03  8439
  8 2509k    8  202k    0     0  95044      0  0:00:27  0:00:02  0:00:25  379k
100 2509k  100 2509k    0     0   837k      0  0:00:02  0:00:02 --:--:-- 1903k
[00:04:37] + mkdir curl-build
[00:04:37] + cd curl-build
[00:04:37] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:03] + hide_output make -j10
[00:05:03] + set +x
[00:05:17] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:17] + hide_output make install
---
 83 67.8M   83 56.7M    0     0  6726k      0  0:00:10  0:00:08  0:00:02 7020k
 95 67.8M   95 64.8M    0     0  6886k      0  0:00:10  0:00:09  0:00:01 7123k
100 67.8M  100 67.8M    0     0  6843k      0  0:00:10  0:00:10 --:--:-- 7660k
[00:09:24] + cd gcc-5.5.0
[00:09:24] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:24] --2019-03-17 14:35:06--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:24] Resolving gcc.gnu.org... 209.132.180.131
[00:09:24] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:24] HTTP request sent, awaiting response... 200 OK
---
[01:19:15]  ---> 80aefca59256
[01:19:15] Step 25/41 : RUN ./build-clang.sh
[01:19:15]  ---> Running in bd96a0f92680
[01:19:15] + source shared.sh
[01:19:15] + LLVM=llvmorg-8.0.0-rc2
[01:19:15] + cd llvm-project
[01:19:15] + cd llvm-project
[01:19:15] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:19:15] + tar xzf - --strip-components=1
[01:19:15]                                  Dload  Upload   Total   Spent    Left  Speed
[01:19:15] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    558      0 --:--:-- --:--:-- --:--:--   644
---
[01:19:33] + cd clang-build
[01:19:33] + INC=/rustroot/include
[01:19:33] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:19:33] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:33] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:20:03] Sun Mar 17 15:45:46 UTC 2019 - building ...
[01:20:15] + hide_output make -j10
[01:20:15] + set +x
[01:20:45] Sun Mar 17 15:46:28 UTC 2019 - building ...
---
[02:49:45]  ---> 0782606e6ae4
[02:49:45] Step 32/41 : RUN ./build-perl.sh
[02:49:45]  ---> Running in 5c55c83b950b
[02:49:46] + source shared.sh
[02:49:46] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:49:46]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:49:46]                                  Dload  Upload   Total   Spent    Left  Speed
[02:49:47] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  1 17.0M    1  332k    0     0   887k      0  0:00:19 --:--:--  0:00:19 1042k
100 17.0M  100 17.0M    0     0  14.4M      0  0:00:01  0:00:01 --:--:-- 15.2M
[02:49:47] + cd perl-5.28.0
[02:49:47] + CC=gcc
[02:49:47] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:49:47] + hide_output ./configure.gnu
[02:49:47] + set +x
[02:50:17] Sun Mar 17 17:15:59 UTC 2019 - building ...
[02:50:31] + hide_output make -j10
---
[02:59:09]    Compiling term v0.0.0 (/checkout/src/libterm)
[02:59:09]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[02:59:11] [RUSTC-TIMING] getopts test:false 2.747
[02:59:15] [RUSTC-TIMING] term test:false 6.017
The job exceeded the maximum time limit for jobs, and has been terminated.
