plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:14f9b05a
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:15] 
[00:01:15] Total download size: 4.9 M
[00:01:15] Downloading Packages:
[00:01:18] --------------------------------------------------------------------------------
[00:01:18] Total                                           1.6 MB/s | 4.9 MB     00:03     
[00:01:18] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:18] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:19] Running Transaction Test
[00:01:19] Finished Transaction Test
[00:01:19] Transaction Test Succeeded
[00:01:19] Running Transaction
---
[00:03:56] + hide_output make install
[00:03:56] + set +x
[00:04:17] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:17] + cd ..
[00:04:17] + rm -rf openssl-1.0.2k
[00:04:17] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:17] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:18]  ---> df1d5b9001a4
[00:04:18] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:18]  ---> a9f07b994ae1
[00:04:18] Step 15/41 : RUN ./build-curl.sh
[00:04:18] Step 15/41 : RUN ./build-curl.sh
[00:04:18]  ---> Running in 02ea7b9a34ce
[00:04:19] + source shared.sh
[00:04:19] + VERSION=7.51.0
[00:04:19] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:20]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:20]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:21] 
  0 2509k    0  2631    0     0   1686      0  0:25:24  0:00:01  0:25:23  1686
  0 2509k    0  2631    0     0   1686      0  0:25:24  0:00:01  0:25:23  1686
  2 2509k    2 59559    0     0  32802      0  0:01:18  0:00:01  0:01:17  217k
100 2509k  100 2509k    0     0   923k      0  0:00:02  0:00:02 --:--:-- 2167k
[00:04:21] + mkdir curl-build
[00:04:21] + cd curl-build
[00:04:21] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:44] + hide_output make -j10
[00:04:44] + set +x
[00:04:57] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:57] + hide_output make install
---
 84 67.8M   84 57.0M    0     0  7249k      0  0:00:09  0:00:08  0:00:01 7849k
 96 67.8M   96 65.3M    0     0  7386k      0  0:00:09  0:00:09 --:--:-- 7825k
100 67.8M  100 67.8M    0     0  7396k      0  0:00:09  0:00:09 --:--:-- 8356k
[00:08:34] + cd gcc-5.5.0
[00:08:34] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:34] --2019-03-04 17:46:24--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:34] Resolving gcc.gnu.org... 209.132.180.131
[00:08:34] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:34] HTTP request sent, awaiting response... 200 OK
---
[01:14:01]  ---> c795ee04c386
[01:14:01] Step 25/41 : RUN ./build-clang.sh
[01:14:01]  ---> Running in 02b63ce18906
[01:14:02] + source shared.sh
[01:14:02] + LLVM=llvmorg-8.0.0-rc2
[01:14:02] + mkdir llvm-project
[01:14:02] + tar xzf - --strip-components=1
[01:14:02] + tar xzf - --strip-components=1
[01:14:02] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:14:02]                                  Dload  Upload   Total   Spent    Left  Speed
[01:14:02] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    555      0 --:--:-- --:--:-- --:--:--   629
---
[01:14:21] + cd clang-build
[01:14:21] + INC=/rustroot/include
[01:14:21] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:14:21] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:14:21] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:14:51] Mon Mar 4 18:52:41 UTC 2019 - building ...
[01:15:00] + hide_output make -j10
[01:15:00] + set +x
[01:15:30] Mon Mar 4 18:53:20 UTC 2019 - building ...
---
[02:42:51]  ---> f9bde987855f
[02:42:51] Step 32/41 : RUN ./build-perl.sh
[02:42:51]  ---> Running in 66bf8ff243ab
[02:42:51] + source shared.sh
[02:42:51] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:42:52]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:42:52]                                  Dload  Upload   Total   Spent    Left  Speed
[02:42:53] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 95 17.0M   95 16.3M    0     0  13.2M      0  0:00:01  0:00:01 --:--:-- 15.6M
100 17.0M  100 17.0M    0     0  13.4M      0  0:00:01  0:00:01 --:--:-- 15.8M
[02:42:53] + cd perl-5.28.0
[02:42:53] + CC=gcc
[02:42:53] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:42:53] + hide_output ./configure.gnu
[02:42:53] + set +x
[02:43:23] Mon Mar 4 20:21:13 UTC 2019 - building ...
[02:43:35] + hide_output make -j10
---
[02:53:11]    Compiling arena v0.0.0 (/checkout/src/libarena)
[02:53:12]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[02:53:26]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[02:54:31]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
The job exceeded the maximum time limit for jobs, and has been terminated.
