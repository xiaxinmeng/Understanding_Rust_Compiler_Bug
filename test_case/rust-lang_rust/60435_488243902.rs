plain
[00:01:21] 
[00:01:21] Total download size: 4.9 M
[00:01:21] Downloading Packages:
[00:01:21] --------------------------------------------------------------------------------
[00:01:21] Total                                           9.8 MB/s | 4.9 MB     00:00     
[00:01:21] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:21] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:22] Running Transaction Test
[00:01:22] Finished Transaction Test
[00:01:22] Transaction Test Succeeded
[00:01:22] Running Transaction
---
[00:03:17] + hide_output make install
[00:03:17] + set +x
[00:03:38] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:38] + cd ..
[00:03:38] + rm -rf openssl-1.0.2k
[00:03:38] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:38] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:39]  ---> 6cc8e1ef90b8
[00:03:39] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:39]  ---> f496c7f8ded4
[00:03:39] Step 15/41 : RUN ./build-curl.sh
[00:03:39] Step 15/41 : RUN ./build-curl.sh
[00:03:39]  ---> Running in 570f74f54e89
[00:03:39] + source shared.sh
[00:03:39] + VERSION=7.51.0
[00:03:39] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:41]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:41]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:42] 
  0 2509k    0 14215    0     0   9373      0  0:04:34  0:00:01  0:04:33  9373
  0 2509k    0 14215    0     0   9373      0  0:04:34  0:00:01  0:04:33  9373
  1 2509k    1 30599    0     0  18656      0  0:02:17  0:00:01  0:02:16  129k
 82 2509k   82 2060k    0     0   785k      0  0:00:03  0:00:02  0:00:01 1845k
100 2509k  100 2509k    0     0   913k      0  0:00:02  0:00:02 --:--:-- 2024k
[00:03:42] + mkdir curl-build
[00:03:42] + cd curl-build
[00:03:42] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:06] + hide_output make -j10
[00:04:06] + set +x
[00:04:18] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:18] + hide_output make install
---
 87 67.8M   87 59.6M    0     0  7125k      0  0:00:09  0:00:08  0:00:01 7534k
 99 67.8M   99 67.6M    0     0  7241k      0  0:00:09  0:00:09 --:--:-- 7655k
100 67.8M  100 67.8M    0     0  7243k      0  0:00:09  0:00:09 --:--:-- 8383k
[00:07:57] + cd gcc-5.5.0
[00:07:57] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:07:57] --2019-05-01 06:41:24--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:07:57] Resolving gcc.gnu.org... 209.132.180.131
[00:07:57] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:07:57] HTTP request sent, awaiting response... 200 OK
---
[01:16:09]  ---> 7da2129aa363
[01:16:09] Step 25/41 : RUN ./build-clang.sh
[01:16:09]  ---> Running in 296a1a7a3dbe
[01:16:10] + source shared.sh
[01:16:10] + LLVM=llvmorg-8.0.0-rc2
[01:16:10] + cd llvm-project
[01:16:10] + cd llvm-project
[01:16:10] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:16:10] + tar xzf - --strip-components=1
[01:16:10]                                  Dload  Upload   Total   Spent    Left  Speed
[01:16:10] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    539      0 --:--:-- --:--:-- --:--:--   612
---
[01:16:29] + cd clang-build
[01:16:29] + INC=/rustroot/include
[01:16:29] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:16:29] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:29] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:59] Wed May 1 07:50:26 UTC 2019 - building ...
[01:17:09] + hide_output make -j10
[01:17:09] + set +x
[01:17:39] Wed May 1 07:51:07 UTC 2019 - building ...
---
[02:44:54]  ---> 88e39b00251d
[02:44:54] Step 32/41 : RUN ./build-perl.sh
[02:44:54]  ---> Running in 90fee55f6b09
[02:44:55] + source shared.sh
[02:44:55] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:44:55]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:44:55]                                  Dload  Upload   Total   Spent    Left  Speed
[02:44:56] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:57:30]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[02:57:39] [RUSTC-TIMING] rustc_errors test:false 8.057
[02:58:49] [RUSTC-TIMING] syntax test:false 70.480
[02:58:49]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
The job exceeded the maximum time limit for jobs, and has been terminated.
