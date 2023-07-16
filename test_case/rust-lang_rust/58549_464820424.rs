plain
[00:01:01] 
[00:01:01] Total download size: 4.9 M
[00:01:01] Downloading Packages:
[00:01:04] --------------------------------------------------------------------------------
[00:01:04] Total                                           1.8 MB/s | 4.9 MB     00:02     
[00:01:04] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:04] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:04] Running Transaction Test
[00:01:04] Finished Transaction Test
[00:01:04] Transaction Test Succeeded
[00:01:04] Running Transaction
---
[00:03:47] + hide_output make install
[00:03:47] + set +x
[00:04:09] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:09] + cd ..
[00:04:09] + rm -rf openssl-1.0.2k
[00:04:09] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:09] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:10]  ---> afc0c6410fad
[00:04:10] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:10]  ---> 2b56fbf230d7
[00:04:10] Step 15/41 : RUN ./build-curl.sh
[00:04:10] Step 15/41 : RUN ./build-curl.sh
[00:04:10]  ---> Running in fa260e419b25
[00:04:10] + source shared.sh
[00:04:10] + VERSION=7.51.0
[00:04:10] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:12]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:12]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:13] 
  0 2509k    0  6975    0     0   4490      0  0:09:32  0:00:01  0:09:31  4490
  0 2509k    0  6975    0     0   4490      0  0:09:32  0:00:01  0:09:31  4490
 18 2509k   18  454k    0     0   208k      0  0:00:12  0:00:02  0:00:10  719k
100 2509k  100 2509k    0     0   877k      0  0:00:02  0:00:02 --:--:-- 1915k
[00:04:13] + mkdir curl-build
[00:04:13] + cd curl-build
[00:04:13] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:37] + hide_output make -j10
[00:04:37] + set +x
[00:04:51] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:51] + hide_output make install
---
 95 67.8M   95 64.5M    0     0  2540k      0  0:00:27  0:00:26  0:00:01 1790k
 97 67.8M   97 66.4M    0     0  2515k      0  0:00:27  0:00:27 --:--:-- 1786k
100 67.8M  100 67.8M    0     0  2485k      0  0:00:27  0:00:27 --:--:-- 1763k
[00:09:11] + cd gcc-5.5.0
[00:09:11] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:11] --2019-02-18 14:40:21--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:11] Resolving gcc.gnu.org... 209.132.180.131
[00:09:11] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:11] HTTP request sent, awaiting response... 200 OK
---
[01:15:26]  ---> 1db9b0f2fa0a
[01:15:26] Step 25/41 : RUN ./build-clang.sh
[01:15:26]  ---> Running in bb756da93eea
[01:15:26] + source shared.sh
[01:15:26] + LLVM=llvmorg-8.0.0-rc2
[01:15:26] + mkdir llvm-project
[01:15:26] + cd llvm-project
[01:15:26] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:15:26] + tar xzf - --strip-components=1
[01:15:26]                                  Dload  Upload   Total   Spent    Left  Speed
[01:15:27] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    410      0 --:--:-- --:--:-- --:--:--   451
---
[01:15:57] + cd clang-build
[01:15:57] + INC=/rustroot/include
[01:15:57] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:15:57] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:15:57] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:27] Mon Feb 18 15:47:37 UTC 2019 - building ...
[01:16:37] + hide_output make -j10
[01:16:37] + set +x
[01:17:07] Mon Feb 18 15:48:17 UTC 2019 - building ...
---
[02:45:56]  ---> 2016e3e17e05
[02:45:56] Step 32/41 : RUN ./build-perl.sh
[02:45:56]  ---> Running in b75b9e0b467a
[02:45:56] + source shared.sh
[02:45:56] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:45:56]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:45:56]                                  Dload  Upload   Total   Spent    Left  Speed
[02:45:57] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 30 17.0M   30 5293k    0     0  9038k      0  0:00:01 --:--:--  0:00:01 9470k
100 17.0M  100 17.0M    0     0  15.3M      0  0:00:01  0:00:01 --:--:-- 15.7M
[02:45:57] + cd perl-5.28.0
[02:45:57] + CC=gcc
[02:45:57] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:45:57] + hide_output ./configure.gnu
[02:45:57] + set +x
[02:46:27] Mon Feb 18 17:17:37 UTC 2019 - building ...
[02:46:39] + hide_output make -j10
---
[02:57:14] [RUSTC-TIMING] rustc_errors test:false 13.461
[02:58:11] [RUSTC-TIMING] syntax test:false 57.067
[02:58:11]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[02:58:40] [RUSTC-TIMING] syntax_ext test:false 28.681
The job exceeded the maximum time limit for jobs, and has been terminated.
