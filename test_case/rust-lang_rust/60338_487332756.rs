plain
[00:01:05] 
[00:01:05] Total download size: 4.9 M
[00:01:05] Downloading Packages:
[00:01:05] --------------------------------------------------------------------------------
[00:01:05] Total                                            22 MB/s | 4.9 MB     00:00     
[00:01:05] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:05] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:05] Running Transaction Test
[00:01:05] Finished Transaction Test
[00:01:05] Transaction Test Succeeded
[00:01:05] Running Transaction
---
[00:02:59] ./build-openssl.sh: line 14:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:02:59] + hide_output make install
[00:02:59] + set +x
[00:03:19] + cd ..
[00:03:19] + rm -rf openssl-1.0.2k
[00:03:20] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:20] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:20]  ---> 2650375c8786
[00:03:20] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:21]  ---> fcd666f836ed
[00:03:21] Step 15/41 : RUN ./build-curl.sh
[00:03:21] Step 15/41 : RUN ./build-curl.sh
[00:03:21]  ---> Running in 1e16b6dbeac5
[00:03:21] + source shared.sh
[00:03:21] + VERSION=7.51.0
[00:03:21] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:23]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:23]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:24] 
  0 2509k    0 14215    0     0   8525      0  0:05:01  0:00:01  0:05:00  8525
  0 2509k    0 14215    0     0   8525      0  0:05:01  0:00:01  0:05:00  8525
 23 2509k   23  601k    0     0   258k      0  0:00:09  0:00:02  0:00:07  889k
100 2509k  100 2509k    0     0   862k      0  0:00:02  0:00:02 --:--:-- 2007k
[00:03:24] + mkdir curl-build
[00:03:24] + cd curl-build
[00:03:24] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:48] + hide_output make -j10
[00:03:48] + set +x
[00:04:01] shared.sh: line 1:    15 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:01] + hide_output make install
---
 78 67.8M   78 53.0M    0     0  6916k      0  0:00:10  0:00:07  0:00:03 7420k
 89 67.8M   89 60.7M    0     0  7028k      0  0:00:09  0:00:08  0:00:01 7376k
100 67.8M  100 67.8M    0     0  7074k      0  0:00:09  0:00:09 --:--:-- 7654k
[00:07:49] + cd gcc-5.5.0
[00:07:49] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:07:49] --2019-04-27 22:46:58--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:07:49] Resolving gcc.gnu.org... 209.132.180.131
[00:07:49] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:07:49] HTTP request sent, awaiting response... 200 OK
---
[01:10:40]  ---> dc709d568cfd
[01:10:40] Step 25/41 : RUN ./build-clang.sh
[01:10:40]  ---> Running in ed5285753654
[01:10:40] + source shared.sh
[01:10:40] + LLVM=llvmorg-8.0.0-rc2
[01:10:40] + cd llvm-project
[01:10:40] + cd llvm-project
[01:10:40] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:10:40] + tar xzf - --strip-components=1
[01:10:40]                                  Dload  Upload   Total   Spent    Left  Speed
[01:10:40] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    624      0 --:--:-- --:--:-- --:--:--   704
---
[01:10:58] + cd clang-build
[01:10:58] + INC=/rustroot/include
[01:10:58] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:10:58] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:10:58] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:11:28] Sat Apr 27 23:50:37 UTC 2019 - building ...
[01:11:37] + hide_output make -j10
[01:11:37] + set +x
[01:12:07] Sat Apr 27 23:51:16 UTC 2019 - building ...
---
[02:36:33]  ---> b600693c4d4b
[02:36:33] Step 32/41 : RUN ./build-perl.sh
[02:36:33]  ---> Running in 74ec1cbe4023
[02:36:33] + source shared.sh
[02:36:33] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:36:33]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:36:33]                                  Dload  Upload   Total   Spent    Left  Speed
[02:36:35] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:54:40]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[02:55:49] [RUSTC-TIMING] rustc_incremental test:false 68.911
[02:58:10] [RUSTC-TIMING] rustc_metadata test:false 210.401
[02:58:51] [RUSTC-TIMING] rustc_typeck test:false 251.109
The job exceeded the maximum time limit for jobs, and has been terminated.
