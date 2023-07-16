plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1975837e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:31] 
[00:01:31] Total download size: 4.9 M
[00:01:31] Downloading Packages:
[00:01:32] --------------------------------------------------------------------------------
[00:01:32] Total                                           5.1 MB/s | 4.9 MB     00:00     
[00:01:32] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:32] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:32] Running Transaction Test
[00:01:32] Finished Transaction Test
[00:01:32] Transaction Test Succeeded
[00:01:32] Running Transaction
---
[00:03:13] + hide_output make install
[00:03:13] + set +x
[00:03:34] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:34] + cd ..
[00:03:34] + rm -rf openssl-1.0.2k
[00:03:34] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:34] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:35] Removing intermediate container afe3730eefba
[00:03:35] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:35]  ---> 6a0f05f6cab0
[00:03:35] Step 15/41 : RUN ./build-curl.sh
[00:03:35] Step 15/41 : RUN ./build-curl.sh
[00:03:35]  ---> Running in f5d22b30791f
[00:03:36] + source shared.sh
[00:03:36] + VERSION=7.51.0
[00:03:36] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:37]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:37]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:39] 
  0 2509k    0 14215    0     0   8542      0  0:05:00  0:00:01  0:04:59  8542
  0 2509k    0 14215    0     0   8542      0  0:05:00  0:00:01  0:04:59  8542
  8 2509k    8  220k    0     0   101k      0  0:00:24  0:00:02  0:00:22  401k
100 2509k  100 2509k    0     0   866k      0  0:00:02  0:00:02 --:--:-- 2024k
[00:03:39] + mkdir curl-build
[00:03:39] + cd curl-build
[00:03:39] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:04] + hide_output make -j10
[00:04:04] + set +x
[00:04:17] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:17] ./build-curl.sh: line 38: 11527 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
---
 95 82.1M   95 78.4M    0     0  2890k      0  0:00:29  0:00:27  0:00:02 3025k
 99 82.1M   99 81.5M    0     0  2897k      0  0:00:29  0:00:28  0:00:01 3108k
100 82.1M  100 82.1M    0     0  2901k      0  0:00:29  0:00:29 --:--:-- 3210k
[00:08:20] + cd gcc-4.8.5
[00:08:20] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:20] --2018-11-14 16:58:22--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:20] Resolving gcc.gnu.org... 209.132.180.131
[00:08:20] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:20] HTTP request sent, awaiting response... 200 OK
---
 24 27.0M   24 6864k    0     0  4489k      0  0:00:06  0:00:01  0:00:05 6095k
 55 27.0M   55 14.9M    0     0  6049k      0  0:00:04  0:00:02  0:00:02 7194k
 82 27.0M   82 22.3M    0     0  6496k      0  0:00:04  0:00:03  0:00:01 7335k
100 27.0M  100 27.0M    0     0  6785k      0  0:00:04  0:00:04 --:--:-- 7528k
[00:44:04] + cd llvm-7.0.0.src
[00:44:04] + mkdir -p tools/clang
[00:44:04] + curl https://releases.llvm.org/7.0.0/cfe-7.0.0.src.tar.xz
[00:44:05] + tar xf - -C tools/clang --strip-components=1
[00:44:05] + xz -d
[00:44:05]                                  Dload  Upload   Total   Spent    Left  Speed
[00:44:07] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0 11.9M    0 62530    0     0   135k      0  0:01:30 --:--:--  0:01:30  136k
---
[00:44:07] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  2  894k    2 26814    0     0  92655      0  0:00:09 --:--:--  0:00:09 93428
100  894k  100  894k    0     0  2039k      0 --:--:-- --:--:-- --:--:-- 2050k
[00:44:07] + mkdir ../clang-build
[00:44:07] + cd ../clang-build
[00:44:07] + INC=/rustroot/include
[00:44:07] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:44:07] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:07] + hide_output cmake ../llvm-7.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:37] Wed Nov 14 17:34:39 UTC 2018 - building ...
[00:44:41] + hide_output make -j10
[00:44:41] + set +x
[00:45:11] Wed Nov 14 17:35:13 UTC 2018 - building ...
---
[01:46:36] + set +x
[01:46:39] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:46:39] + set +x
[01:46:43] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:46:43] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:46:43] + yes
[01:46:43] + yes
[01:46:43] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:46:44] + rm -rf linux-3.2.84
[01:46:45]  ---> 29d87450f8ee
[01:46:45] Removing intermediate container 4fdba4ca0356
[01:46:45] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:46:45] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:46:45]  ---> 60be790e5220
[01:46:45] Step 32/41 : RUN ./build-perl.sh
[01:46:45]  ---> Running in 4eec34067303
[01:46:45] + source shared.sh
[01:46:45] + tar xzf -
[01:46:45] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:46:45]                                  Dload  Upload   Total   Spent    Left  Speed
[01:46:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 40 17.0M   40 7053k    0     0  8161k      0  0:00:02 --:--:--  0:00:02 8697k
---
[02:59:10] [ 92%] Built target llvm-cxxfilt
[02:59:10] [ 92%] Building CXX object tools/llvm-demangle-fuzzer/CMakeFiles/llvm-demangle-fuzzer.dir/llvm-demangle-fuzzer.cpp.o
[02:59:10] [ 92%] Building CXX object tools/llvm-cov/CMakeFiles/llvm-cov.dir/CoverageExporterJson.cpp.o

Broadcast message from root@travis-job-56ed98e5-f4c1-431d-88d5-6e42c42ce3c4
 (unknown) at 19:49 ...
The system is going down for power off NOW!
[02:59:11] [ 92%] Linking CXX executable ../../bin/llvm-demangle-fuzzer
[02:59:11] [ 92%] Linking CXX executable ../../bin/llvm-cxxdump
[02:59:12] [ 92%] Built target llvm-demangle-fuzzer
[02:59:12] Scanning dependencies of target llvm-diff
