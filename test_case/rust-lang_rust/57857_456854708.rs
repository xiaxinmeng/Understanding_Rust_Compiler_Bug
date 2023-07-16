plain
[00:00:57] 
[00:00:57] Total download size: 4.9 M
[00:00:57] Downloading Packages:
[00:01:00] --------------------------------------------------------------------------------
[00:01:00] Total                                           1.6 MB/s | 4.9 MB     00:03     
[00:01:00] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:00] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:01] Running Transaction Test
[00:01:01] Finished Transaction Test
[00:01:01] Transaction Test Succeeded
[00:01:01] Running Transaction
---
[00:03:44] + hide_output make install
[00:03:44] + set +x
[00:04:05] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:05] + cd ..
[00:04:05] + rm -rf openssl-1.0.2k
[00:04:05] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:05] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:06]  ---> 394da2f4bf6c
[00:04:06] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:06]  ---> 347feff5f43d
[00:04:06] Step 15/41 : RUN ./build-curl.sh
[00:04:06] Step 15/41 : RUN ./build-curl.sh
[00:04:06]  ---> Running in 5b854b24d2de
[00:04:07] + source shared.sh
[00:04:07] + VERSION=7.51.0
[00:04:07] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:08]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:08]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:10] 
  0 2509k    0 14215    0     0   9199      0  0:04:39  0:00:01  0:04:38  9199
  0 2509k    0 14215    0     0   9199      0  0:04:39  0:00:01  0:04:38  9199
  4 2509k    4  114k    0     0  61306      0  0:00:41  0:00:01  0:00:40  271k
 94 2509k   94 2360k    0     0   838k      0  0:00:02  0:00:02 --:--:-- 1846k
100 2509k  100 2509k    0     0   885k      0  0:00:02  0:00:02 --:--:-- 1936k
[00:04:10] + mkdir curl-build
[00:04:10] + cd curl-build
[00:04:10] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:34] + hide_output make -j10
[00:04:34] + set +x
[00:04:47] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:47] + hide_output make install
---
 96 82.1M   96 79.3M    0     0  2736k      0  0:00:30  0:00:29  0:00:01 2503k
 99 82.1M   99 81.6M    0     0  2721k      0  0:00:30  0:00:30 --:--:-- 2565k
100 82.1M  100 82.1M    0     0  2720k      0  0:00:30  0:00:30 --:--:-- 2660k
[00:09:02] + cd gcc-4.8.5
[00:09:02] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:02] --2019-01-23 13:03:04--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:02] Resolving gcc.gnu.org... 209.132.180.131
[00:09:02] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:02] HTTP request sent, awaiting response... 200 OK
---
[00:45:19]  ---> 1439f877b702
[00:45:19] Step 25/41 : RUN ./build-clang.sh
[00:45:19]  ---> Running in e86e242ca106
[00:45:19] + source shared.sh
[00:45:19] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:45:19] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:45:19] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:45:19] + mkdir clang
[00:45:19] + cd clang
[00:45:19] + tar xzf - --strip-components=1
[00:45:19] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:45:19]                                  Dload  Upload   Total   Spent    Left  Speed
[00:45:19] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    806      0 --:--:-- --:--:-- --:--:--   844
---
100 9504k    0 9504k    0     0  4535k      0 --:--:--  0:00:02 --:--:-- 5466k
100 14.2M    0 14.2M    0     0  4691k      0 --:--:--  0:00:03 --:--:-- 5242k
100 17.1M    0 17.1M    0     0  4968k      0 --:--:--  0:00:03 --:--:-- 5548k
[00:45:32] + mkdir -p tools/lld
[00:45:32] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:45:32] + tar zxf - --strip-components=1 -C tools/lld
[00:45:32]                                  Dload  Upload   Total   Spent    Left  Speed
[00:45:32] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    834      0 --:--:-- --:--:-- --:--:--   848
---
[00:45:33] + cd clang-build
[00:45:33] + INC=/rustroot/include
[00:45:33] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:45:33] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:45:33] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:46:03] Wed Jan 23 13:40:04 UTC 2019 - building ...
[00:46:08] + hide_output make -j10
[00:46:08] + set +x
[00:46:38] Wed Jan 23 13:40:40 UTC 2019 - building ...
---
[01:50:35]  ---> 9d169687f58e
[01:50:35] Step 32/41 : RUN ./build-perl.sh
[01:50:35]  ---> Running in e79c2f7c164f
[01:50:36] + source shared.sh
[01:50:36] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:50:36]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:50:36]                                  Dload  Upload   Total   Spent    Left  Speed
[01:50:37] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:41]    Compiling quote v0.6.10
[02:58:42]    Compiling syn v0.15.22
[02:58:48] [RUSTC-TIMING] serde test:false 11.601
[02:58:48]    Compiling serde_json v1.0.33
The job exceeded the maximum time limit for jobs, and has been terminated.
