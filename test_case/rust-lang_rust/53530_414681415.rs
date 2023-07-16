plain
[00:01:38] 
[00:01:38] Total download size: 4.9 M
[00:01:38] Downloading Packages:
[00:01:38] --------------------------------------------------------------------------------
[00:01:38] Total                                            22 MB/s | 4.9 MB     00:00     
[00:01:38] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:38] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:38] Running Transaction Test
[00:01:38] Finished Transaction Test
[00:01:38] Transaction Test Succeeded
[00:01:38] Running Transaction
---
[00:03:13] + hide_output make install
[00:03:13] + set +x
[00:03:32] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:32] + cd ..
[00:03:32] + rm -rf openssl-1.0.2k
[00:03:32] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:32] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:33] Removing intermediate container 86c11a83fc6f
[00:03:33] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:33]  ---> 447a409ad779
[00:03:33] Step 15/38 : RUN ./build-curl.sh
[00:03:33] Step 15/38 : RUN ./build-curl.sh
[00:03:33]  ---> Running in 03e88d8bcbcd
[00:03:33] + source shared.sh
[00:03:33] + VERSION=7.51.0
[00:03:33] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:35]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:35]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:36] 
  0 2509k    0 14215    0     0   9165      0  0:04:40  0:00:01  0:04:39  9165
  0 2509k    0 14215    0     0   9165      0  0:04:40  0:00:01  0:04:39  9165
 52 2509k   52 1310k    0     0   533k      0  0:00:04  0:00:02  0:00:02 1426k
100 2509k  100 2509k    0     0   908k      0  0:00:02  0:00:02 --:--:-- 2061k
[00:03:36] + mkdir curl-build
[00:03:36] + cd curl-build
[00:03:36] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:58] + hide_output make -j10
[00:03:58] + set +x
[00:04:11] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:11] + hide_output make install
---
 94 82.1M   94 77.3M    0     0  3605k      0  0:00:23  0:00:21  0:00:02 3208k
 98 82.1M   98 80.8M    0     0  3605k      0  0:00:23  0:00:22  0:00:01 3451k
100 82.1M  100 82.1M    0     0  3586k      0  0:00:23  0:00:23 --:--:-- 3412k
[00:08:00] + cd gcc-4.8.5
[00:08:00] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:00] --2018-08-21 11:00:30--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:00] Resolving gcc.gnu.org... 209.132.180.131
[00:08:00] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:00] HTTP request sent, awaiting response... 200 OK
---
[00:41:50]  ---> 2eab8d65158e
[00:41:50] Step 25/38 : RUN ./build-clang.sh
[00:41:50]  ---> Running in 6ca89c5949a9
[00:41:51] + source shared.sh
[00:41:51] + LLVM=6.0.0
[00:41:51] + mkdir clang
[00:41:51] + cd clang
[00:41:51] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:41:51] + tar xf -
[00:41:51] + xz -d
[00:41:51]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:54] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 34 24.1M   34 8479k    0     0  7191k      0  0:00:03  0:00:01  0:00:02 8378k
 70 24.1M   70 16.8M    0     0  7940k      0  0:00:03  0:00:02  0:00:01 8600k
100 24.1M  100 24.1M    0     0  8111k      0  0:00:03  0:00:03 --:--:-- 8583k
[00:41:54] + cd llvm-6.0.0.src
[00:41:54] + mkdir -p tools/clang
[00:41:54] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:41:54] + xz -d
[00:41:54] + tar xf - -C tools/clang --strip-components=1
[00:41:54]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:55] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  1 11.4M    1  147k    0     0  1252k      0  0:00:09 --:--:--  0:00:09 1264k
  1 11.4M    1  147k    0     0  1252k      0  0:00:09 --:--:--  0:00:09 1264k
 65 11.4M   65 7667k    0     0  6853k      0  0:00:01  0:00:01 --:--:-- 6864k
100 11.4M  100 11.4M    0     0  7552k      0  0:00:01  0:00:01 --:--:-- 7559k
[00:41:55] + mkdir ../clang-build
[00:41:55] + cd ../clang-build
[00:41:55] + INC=/rustroot/include
[00:41:55] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:55] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:55] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:24] + hide_output make -j10
[00:42:24] + set +x
[00:42:54] Tue Aug 21 11:35:25 UTC 2018 - building ...
[00:43:24] Tue Aug 21 11:35:55 UTC 2018 - building ...
---
[01:33:07] Step 28/38 : RUN ./build-git.sh
[01:33:07]  ---> Running in bb35bcf78f7c
[01:33:07] + source shared.sh
[01:33:07] + tar xzf -
[01:33:07] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:33:07]                                  Dload  Upload   Total   Spent    Left  Speed
[01:33:08] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:34:08] + set +x
[01:34:09] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:34:09] + set +x
[01:34:13] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:34:13] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:34:13] + yes
[01:34:13] + yes
[01:34:13] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:34:13] + rm -rf linux-3.2.84
[01:34:14]  ---> c79326365065
[01:34:14] Removing intermediate container 909e408c9d09
[01:34:14] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[02:57:06] [RUSTC-TIMING] syntax_pos test:false 3.596
[02:57:06]    Compiling rustc-ap-rustc_errors v218.0.0
[02:57:13] [RUSTC-TIMING] rustc_errors test:false 6.983
[02:58:25] [RUSTC-TIMING] syntax test:false 72.122
The job exceeded the maximum time limit for jobs, and has been terminated.
