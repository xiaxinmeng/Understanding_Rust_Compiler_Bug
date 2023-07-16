plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:11cb7495
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:15] 
[00:01:15] Total download size: 4.9 M
[00:01:15] Downloading Packages:
[00:01:16] --------------------------------------------------------------------------------
[00:01:16] Total                                           8.3 MB/s | 4.9 MB     00:00     
[00:01:16] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:16] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:16] Running Transaction Test
[00:01:16] Finished Transaction Test
[00:01:16] Transaction Test Succeeded
[00:01:16] Running Transaction
---
[00:03:16] + hide_output make install
[00:03:16] + set +x
[00:03:37] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:37] + cd ..
[00:03:37] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:37] + rm -rf openssl-1.0.2k
[00:03:38] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:38]  ---> ce07fde55f4f
[00:03:38] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:39]  ---> 44158b853d09
[00:03:39] Step 15/41 : RUN ./build-curl.sh
[00:03:39] Step 15/41 : RUN ./build-curl.sh
[00:03:39]  ---> Running in 72bcf85d36b6
[00:03:39] + source shared.sh
[00:03:39] + VERSION=7.51.0
[00:03:39] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:41]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:41]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:42] 
  0 2509k    0 14215    0     0   9224      0  0:04:38  0:00:01  0:04:37  9224
  0 2509k    0 14215    0     0   9224      0  0:04:38  0:00:01  0:04:37  9224
 60 2509k   60 1510k    0     0   601k      0  0:00:04  0:00:02  0:00:02 1539k
100 2509k  100 2509k    0     0   896k      0  0:00:02  0:00:02 --:--:-- 1982k
[00:03:42] + mkdir curl-build
[00:03:42] + cd curl-build
[00:03:42] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:07] + hide_output make -j10
[00:04:07] + set +x
[00:04:21] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:21] + hide_output make install
---
 81 67.8M   81 55.3M    0     0  6931k      0  0:00:10  0:00:08  0:00:02 7452k
 93 67.8M   93 63.4M    0     0  7078k      0  0:00:09  0:00:09 --:--:-- 7476k
100 67.8M  100 67.8M    0     0  7070k      0  0:00:09  0:00:09 --:--:-- 7987k
[00:08:14] + cd gcc-5.5.0
[00:08:14] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:14] --2019-04-29 08:44:17--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:14] Resolving gcc.gnu.org... 209.132.180.131
[00:08:14] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:15] HTTP request sent, awaiting response... 200 OK
---
[01:17:22]  ---> d087b53ac911
[01:17:22] Step 25/41 : RUN ./build-clang.sh
[01:17:22]  ---> Running in 6742b738db6b
[01:17:22] + source shared.sh
[01:17:22] + LLVM=llvmorg-8.0.0-rc2
[01:17:22] + cd llvm-project
[01:17:22] + tar xzf - --strip-components=1
[01:17:22] + tar xzf - --strip-components=1
[01:17:22] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:17:22]                                  Dload  Upload   Total   Spent    Left  Speed
[01:17:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:17:41] + cd clang-build
[01:17:41] + INC=/rustroot/include
[01:17:41] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:17:41] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:17:41] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:18:12] Mon Apr 29 09:54:14 UTC 2019 - building ...
[01:18:22] + hide_output make -j10
[01:18:22] + set +x
[01:18:52] Mon Apr 29 09:54:55 UTC 2019 - building ...
---
[02:46:25]  ---> cbfdbcce3543
[02:46:25] Step 32/41 : RUN ./build-perl.sh
[02:46:25]  ---> Running in 34c614d0bba4
[02:46:25] + source shared.sh
[02:46:25] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:46:25]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:46:25]                                  Dload  Upload   Total   Spent    Left  Speed
[02:46:26] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
