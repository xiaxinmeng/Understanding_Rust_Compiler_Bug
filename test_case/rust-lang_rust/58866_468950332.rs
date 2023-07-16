plain
[00:01:21] 
[00:01:21] Total download size: 4.9 M
[00:01:21] Downloading Packages:
[00:01:24] --------------------------------------------------------------------------------
[00:01:24] Total                                           1.7 MB/s | 4.9 MB     00:02     
[00:01:24] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:24] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:24] Running Transaction Test
[00:01:24] Finished Transaction Test
[00:01:24] Transaction Test Succeeded
[00:01:24] Running Transaction
---
[00:04:07] + hide_output make install
[00:04:07] + set +x
[00:04:28] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:28] + cd ..
[00:04:28] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:28] + rm -rf openssl-1.0.2k
[00:04:28] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:29]  ---> 96ae4cc86c10
[00:04:29] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:29]  ---> d1a73e62d001
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29]  ---> Running in 4cd63043304b
[00:04:30] + source shared.sh
[00:04:30] + VERSION=7.51.0
[00:04:30] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:31]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:31]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:33] 
  0 2509k    0 14215    0     0   9320      0  0:04:35  0:00:01  0:04:34  9320
  0 2509k    0 14215    0     0   9320      0  0:04:35  0:00:01  0:04:34  9320
  2 2509k    2 59559    0     0  33588      0  0:01:16  0:00:01  0:01:15  177k
 81 2509k   81 2053k    0     0   756k      0  0:00:03  0:00:02  0:00:01 1712k
100 2509k  100 2509k    0     0   874k      0  0:00:02  0:00:02 --:--:-- 1857k
[00:04:33] + mkdir curl-build
[00:04:33] + cd curl-build
[00:04:33] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:57] + hide_output make -j10
[00:04:57] + set +x
[00:05:10] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:10] + hide_output make install
---
 88 67.8M   88 59.7M    0     0  6758k      0  0:00:10  0:00:09  0:00:01 7095k
 99 67.8M   99 67.2M    0     0  6851k      0  0:00:10  0:00:10 --:--:-- 7499k
100 67.8M  100 67.8M    0     0  6863k      0  0:00:10  0:00:10 --:--:-- 8245k
[00:09:02] + cd gcc-5.5.0
[00:09:02] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:02] --2019-03-02 16:16:57--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:03] Resolving gcc.gnu.org... 209.132.180.131
[00:09:03] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:03] HTTP request sent, awaiting response... 200 OK
---
[01:13:07]  ---> e8cb5579a0ce
[01:13:07] Step 25/41 : RUN ./build-clang.sh
[01:13:07]  ---> Running in e10584463d73
[01:13:08] + source shared.sh
[01:13:08] + LLVM=llvmorg-8.0.0-rc2
[01:13:08] + mkdir llvm-project
[01:13:08] + cd llvm-project
[01:13:08] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:13:08] + tar xzf - --strip-components=1
[01:13:08]                                  Dload  Upload   Total   Spent    Left  Speed
[01:13:08] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    634      0 --:--:-- --:--:-- --:--:--   715
---
[01:13:26] + cd clang-build
[01:13:26] + INC=/rustroot/include
[01:13:26] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:13:26] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:13:26] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:13:56] Sat Mar 2 17:21:50 UTC 2019 - building ...
[01:14:06] + hide_output make -j10
[01:14:06] + set +x
[01:14:36] Sat Mar 2 17:22:30 UTC 2019 - building ...
---
[02:38:36]  ---> b540de67493e
[02:38:36] Step 32/41 : RUN ./build-perl.sh
[02:38:36]  ---> Running in fdc0ef45a65b
[02:38:36] + source shared.sh
[02:38:36] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:38:36]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:38:36]                                  Dload  Upload   Total   Spent    Left  Speed
[02:38:38] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0 17.0M    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 64 17.0M   64 11.0M    0     0  8377k      0  0:00:02  0:00:01  0:00:01 8800k
100 17.0M  100 17.0M    0     0  10.5M      0  0:00:01  0:00:01 --:--:-- 11.0M
[02:38:38] + cd perl-5.28.0
[02:38:38] + CC=gcc
[02:38:38] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:38:38] + hide_output ./configure.gnu
[02:38:38] + set +x
[02:39:08] Sat Mar 2 18:47:02 UTC 2019 - building ...
[02:39:19] + hide_output make -j10
---
[02:57:55] [RUSTC-TIMING] rustc_metadata test:false 181.343
[02:58:36] [RUSTC-TIMING] rustc_typeck test:false 221.872
[02:59:15]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[02:59:17]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
The job exceeded the maximum time limit for jobs, and has been terminated.
