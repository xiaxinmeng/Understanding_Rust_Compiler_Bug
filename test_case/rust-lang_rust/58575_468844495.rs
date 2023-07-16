plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:203d8810
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:11] 
[00:01:11] Total download size: 4.9 M
[00:01:11] Downloading Packages:
[00:01:14] --------------------------------------------------------------------------------
[00:01:14] Total                                           1.6 MB/s | 4.9 MB     00:03     
[00:01:14] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:14] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:14] Running Transaction Test
[00:01:14] Finished Transaction Test
[00:01:14] Transaction Test Succeeded
[00:01:14] Running Transaction
---
[00:04:00] + hide_output make install
[00:04:00] + set +x
[00:04:22] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:22] + cd ..
[00:04:22] + rm -rf openssl-1.0.2k
[00:04:22] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:22] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:23]  ---> 4b9f39ea6547
[00:04:23] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:23]  ---> 5a9ec6b1a99f
[00:04:23] Step 15/41 : RUN ./build-curl.sh
[00:04:23] Step 15/41 : RUN ./build-curl.sh
[00:04:23]  ---> Running in 72e292752b62
[00:04:24] + source shared.sh
[00:04:24] + VERSION=7.51.0
[00:04:24] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:25]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:25]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:26] 
  0 2509k    0  6975    0     0   4560      0  0:09:23  0:00:01  0:09:22  4560
  0 2509k    0  6975    0     0   4560      0  0:09:23  0:00:01  0:09:22  4560
  7 2509k    7  200k    0     0    98k      0  0:00:25  0:00:02  0:00:23  389k
100 2509k  100 2509k    0     0   862k      0  0:00:02  0:00:02 --:--:-- 1813k
[00:04:27] + mkdir curl-build
[00:04:27] + cd curl-build
[00:04:27] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:52] + hide_output make -j10
[00:04:52] + set +x
[00:05:06] shared.sh: line 1:    15 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:06] ./build-curl.sh: line 29: 11530 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
---
 98 67.8M   98 66.7M    0     0  1705k      0  0:00:40  0:00:40 --:--:--  774k
 99 67.8M   99 67.5M    0     0  1684k      0  0:00:41  0:00:41 --:--:--  811k
100 67.8M  100 67.8M    0     0  1680k      0  0:00:41  0:00:41 --:--:--  837k
[00:09:59] + cd gcc-5.5.0
[00:09:59] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:59] --2019-03-01 20:27:52--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:59] Resolving gcc.gnu.org... 209.132.180.131
[00:10:00] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:00] HTTP request sent, awaiting response... 200 OK
---
[01:19:21]  ---> 2456b7b31a73
[01:19:21] Step 25/41 : RUN ./build-clang.sh
[01:19:21]  ---> Running in 3f224f90fa78
[01:19:21] + source shared.sh
[01:19:21] + LLVM=llvmorg-8.0.0-rc2
[01:19:21] + mkdir llvm-project
[01:19:21] + cd llvm-project
[01:19:21] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:19:21] + tar xzf - --strip-components=1
[01:19:21]                                  Dload  Upload   Total   Spent    Left  Speed
[01:19:22] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    569      0 --:--:-- --:--:-- --:--:--   653
---
[01:19:41] + cd clang-build
[01:19:41] + INC=/rustroot/include
[01:19:41] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:19:41] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:41] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:20:11] Fri Mar 1 21:38:04 UTC 2019 - building ...
[01:20:22] + hide_output make -j10
[01:20:22] + set +x
[01:20:52] Fri Mar 1 21:38:45 UTC 2019 - building ...
---
[02:45:56]  ---> 1e10c3d27f70
[02:45:56] Step 32/41 : RUN ./build-perl.sh
[02:45:56]  ---> Running in f9f2c21ef3d0
[02:45:56] + source shared.sh
[02:45:56] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:45:56]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:45:56]                                  Dload  Upload   Total   Spent    Left  Speed
[02:45:57] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 17.0M  100 17.0M    0     0  14.4M      0  0:00:01  0:00:01 --:--:-- 15.4M
[02:45:57] + cd perl-5.28.0
[02:45:57] + CC=gcc
[02:45:57] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:45:57] + hide_output ./configure.gnu
[02:45:57] + set +x
[02:46:27] Fri Mar 1 23:04:20 UTC 2019 - building ...
[02:46:41] + hide_output make -j10
---
[02:56:29]    Compiling arena v0.0.0 (/checkout/src/libarena)
[02:56:29]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[02:56:34]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[02:57:41]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
The job exceeded the maximum time limit for jobs, and has been terminated.
