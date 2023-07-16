plain
[00:01:22] 
[00:01:22] Total download size: 4.9 M
[00:01:22] Downloading Packages:
[00:01:25] --------------------------------------------------------------------------------
[00:01:25] Total                                           1.6 MB/s | 4.9 MB     00:03     
[00:01:25] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:25] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:25] Running Transaction Test
[00:01:25] Finished Transaction Test
[00:01:25] Transaction Test Succeeded
[00:01:25] Running Transaction
---
[00:04:10] + hide_output make install
[00:04:10] + set +x
[00:04:32] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:32] + cd ..
[00:04:32] + rm -rf openssl-1.0.2k
[00:04:32] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:32] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:33]  ---> 784c42ee209e
[00:04:33] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:33]  ---> 5592947d82fd
[00:04:33] Step 15/41 : RUN ./build-curl.sh
[00:04:33] Step 15/41 : RUN ./build-curl.sh
[00:04:33]  ---> Running in 7ba508e70f67
[00:04:34] + source shared.sh
[00:04:34] + VERSION=7.51.0
[00:04:34] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:35]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:35]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:37] 
  0 2509k    0 14215    0     0   9006      0  0:04:45  0:00:01  0:04:44  9006
  0 2509k    0 14215    0     0   9006      0  0:04:45  0:00:01  0:04:44  9006
  4 2509k    4  106k    0     0  55772      0  0:00:46  0:00:01  0:00:45  247k
 93 2509k   93 2355k    0     0   799k      0  0:00:03  0:00:02  0:00:01 1712k
100 2509k  100 2509k    0     0   847k      0  0:00:02  0:00:02 --:--:-- 1803k
[00:04:37] + mkdir curl-build
[00:04:37] + cd curl-build
[00:04:37] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:05] + hide_output make -j10
[00:05:05] + set +x
[00:05:20] shared.sh: line 1:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:20] + hide_output make install
---
 85 67.8M   85 58.0M    0     0  7134k      0  0:00:09  0:00:08  0:00:01 7561k
 96 67.8M   96 65.7M    0     0  7219k      0  0:00:09  0:00:09 --:--:-- 7421k
100 67.8M  100 67.8M    0     0  7234k      0  0:00:09  0:00:09 --:--:-- 8134k
[00:09:15] + cd gcc-5.5.0
[00:09:15] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:15] --2019-03-16 07:52:58--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:15] Resolving gcc.gnu.org... 209.132.180.131
[00:09:15] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:15] HTTP request sent, awaiting response... 200 OK
---
[01:18:55]  ---> f2e11ea3fff3
[01:18:55] Step 25/41 : RUN ./build-clang.sh
[01:18:55]  ---> Running in a730da688f7d
[01:18:55] + source shared.sh
[01:18:55] + LLVM=llvmorg-8.0.0-rc2
[01:18:55] + mkdir llvm-project
[01:18:55] + cd llvm-project
[01:18:55] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:18:55] + tar xzf - --strip-components=1
[01:18:56]                                  Dload  Upload   Total   Spent    Left  Speed
[01:18:56] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    640      0 --:--:-- --:--:-- --:--:--   759
---
[01:19:14] + cd clang-build
[01:19:14] + INC=/rustroot/include
[01:19:14] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:19:14] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:14] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:19:44] Sat Mar 16 09:03:27 UTC 2019 - building ...
[01:19:55] + hide_output make -j10
[01:19:55] + set +x
[01:20:25] Sat Mar 16 09:04:08 UTC 2019 - building ...
---
[02:48:56]  ---> 228cefa307d9
[02:48:56] Step 32/41 : RUN ./build-perl.sh
[02:48:56]  ---> Running in 05376731bc02
[02:48:57] + source shared.sh
[02:48:57] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:48:57]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:48:57]                                  Dload  Upload   Total   Spent    Left  Speed
[02:48:58] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0 17.0M    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 17.0M  100 17.0M    0     0  13.6M      0  0:00:01  0:00:01 --:--:-- 15.2M
[02:48:58] + cd perl-5.28.0
[02:48:58] + CC=gcc
[02:48:58] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:48:58] + hide_output ./configure.gnu
[02:48:58] + set +x
[02:49:28] Sat Mar 16 10:33:11 UTC 2019 - building ...
[02:49:42] + hide_output make -j10
